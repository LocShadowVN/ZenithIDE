use std::fs;
use std::io::Write;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileNode>, String> {
    let mut nodes = Vec::new();
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.starts_with('.') {
            nodes.push(FileNode {
                name,
                path: path.to_string_lossy().to_string(),
                is_dir: path.is_dir(),
            });
        }
    }
    nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));
    Ok(nodes)
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_new_file(app: AppHandle, lang: String) -> Result<String, String> {
    let path = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("zenith_workspace");
    let _ = fs::create_dir_all(&path);

    let ext = match lang.as_str() {
        "c" => "c",
        "cpp" => "cpp",
        "rust" => "rs",
        "html" => "html",
        _ => "txt",
    };

    let mut file_name = format!("untitled.{}", ext);
    let mut count = 1;
    while path.join(&file_name).exists() {
        file_name = format!("untitled_{}.{}", count, ext);
        count += 1;
    }

    let file_path = path.join(&file_name);
    let default_content = match lang.as_str() {
        "c" => "#include <stdio.h>\n\nint main() {\n    \n    return 0;\n}",
        "cpp" => "#include <iostream>\n\nint main() {\n    \n    return 0;\n}",
        "rust" => "fn main() {\n    \n}",
        "html" => "<!DOCTYPE html>\n<html>\n<head>\n    <title>Document</title>\n</head>\n<body>\n    \n</body>\n</html>",
        _ => "",
    };

    fs::write(&file_path, default_content).map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn create_new_folder(app: AppHandle, name: String) -> Result<String, String> {
    let path = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("zenith_workspace")
        .join(&name);
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_default_workspace(app: AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("zenith_workspace");
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_compiler_path(app: AppHandle, lang: String) -> Result<String, String> {
    let local_dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;

    if lang == "c" || lang == "cpp" {
        let mingw_gcc = local_dir.join("compilers/mingw64/bin/gcc.exe");
        if mingw_gcc.exists() {
            return Ok(mingw_gcc.to_string_lossy().to_string());
        }

        let mingw_gpp = local_dir.join("compilers/mingw64/bin/g++.exe");
        if mingw_gpp.exists() {
            return Ok(mingw_gpp.to_string_lossy().to_string());
        }

        let exe_name = if lang == "c" { "gcc" } else { "g++" };
        if let Ok(path) = which::which(exe_name) {
            return Ok(path.to_string_lossy().to_string());
        }

        return Err("Not Installed".to_string());
    }

    if lang == "rust" {
        if let Ok(path) = which::which("rustc") {
            return Ok(path.to_string_lossy().to_string());
        }
        return Err("Not Installed".to_string());
    }

    Err("Unsupported language".to_string())
}

#[tauri::command]
pub async fn install_compiler(app: AppHandle, lang: String) -> Result<(), String> {
    if lang == "c" || lang == "cpp" {
        if !cfg!(target_os = "windows") {
            return Err(
                "On Linux/Mac, please install gcc/g++ using your package manager.".to_string(),
            );
        }

        let local_dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
        let target_dir = local_dir.join("compilers");
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        let url = "https://github.com/brechtsanders/winlibs_mingw/releases/download/13.2.0-16.0.6-11.0.1-msvcrt-10.5.0/winlibs-x86_64-MCF-posix-13.2.0-16.0.6-11.0.1-msvcrt-10.5.0.zip";

        let _ = app.emit("compiler-status", "Downloading C/C++...");
        let res = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let total = res.content_length().unwrap_or(1);
        let mut file =
            fs::File::create(target_dir.join("compiler.zip")).map_err(|e| e.to_string())?;

        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| e.to_string())?;
            file.write_all(&chunk).map_err(|e| e.to_string())?;
            downloaded += chunk.len() as u64;
            let percent = (downloaded * 100) / total;
            let _ = app.emit("compiler-progress", percent);
        }
        drop(file);

        let _ = app.emit("compiler-status", "Extracting C/C++...");
        let zip_file =
            fs::File::open(target_dir.join("compiler.zip")).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(zip_file).map_err(|e| e.to_string())?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            let outpath = match file.enclosed_name() {
                Some(path) => target_dir.join(path),
                None => continue,
            };
            if file.is_dir() {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
                let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
        fs::remove_file(target_dir.join("compiler.zip")).map_err(|e| e.to_string())?;
    } else if lang == "rust" {
        let _ = app.emit("compiler-status", "Installing Rust...");

        if cfg!(target_os = "windows") {
            let url = "https://win.rustup.rs/x86_64";
            let res = reqwest::get(url).await.map_err(|e| e.to_string())?;
            let bytes = res.bytes().await.map_err(|e| e.to_string())?;
            let temp_dir = std::env::temp_dir();
            let exe_path = temp_dir.join("rustup-init.exe");
            fs::write(&exe_path, &bytes).map_err(|e| e.to_string())?;

            let status = std::process::Command::new(&exe_path)
                .args(["-y", "--default-toolchain", "stable-gnu"])
                .status()
                .map_err(|e| e.to_string())?;

            fs::remove_file(&exe_path).ok();
            if !status.success() {
                return Err("Rust installation failed".to_string());
            }
        } else {
            let status = std::process::Command::new("sh")
                .args([
                    "-c",
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
                ])
                .status()
                .map_err(|e| e.to_string())?;
            if !status.success() {
                return Err("Rust installation failed".to_string());
            }
        }
    }

    let _ = app.emit("compiler-status", "Done");
    Ok(())
}

#[tauri::command]
pub async fn get_app_version(app: AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}

#[tauri::command]
pub async fn get_system_info() -> Result<String, String> {
    let os = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Unknown OS"
    };

    let arch = std::env::consts::ARCH;
    Ok(format!("{} ({})", os, arch))
}
