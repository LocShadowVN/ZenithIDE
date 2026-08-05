use std::fs;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub name: String, pub path: String, pub is_dir: bool,
}

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileNode>, String> {
    let mut nodes = Vec::new();
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.starts_with('.') {
            nodes.push(FileNode { name, path: path.to_string_lossy().to_string(), is_dir: path.is_dir() });
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
pub async fn get_default_workspace(app: AppHandle) -> Result<String, String> {
    let path = app.path().app_local_data_dir().map_err(|e| e.to_string())?.join("zenith_workspace");
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_compiler_path(app: AppHandle, lang: String) -> Result<String, String> {
    let exe_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    
    if lang == "c" {
        let mingw_gcc = exe_dir.join("bin/mingw/bin/gcc.exe");
        if mingw_gcc.exists() {
            return Ok(mingw_gcc.to_string_lossy().to_string());
        }
        if cfg!(target_os = "windows") {
            return Err("MinGW (gcc) not found. Please reinstall ZenithIDE.".to_string());
        }
        return Ok("gcc".to_string());
    }
    
    if lang == "cpp" {
        let mingw_gpp = exe_dir.join("bin/mingw/bin/g++.exe");
        if mingw_gpp.exists() {
            return Ok(mingw_gpp.to_string_lossy().to_string());
        }
        if cfg!(target_os = "windows") {
            return Err("MinGW (g++) not found. Please reinstall ZenithIDE.".to_string());
        }
        return Ok("g++".to_string());
    }
    
    if lang == "rust" {
        return Ok("rustc".to_string());
    }
    
    Err("Unsupported language".to_string())
}
