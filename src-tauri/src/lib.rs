mod commands;
mod state;

use state::PtyState;
use tauri::Manager;

fn init_workspace(app: &tauri::AppHandle) {
    let path = app
        .path()
        .app_local_data_dir()
        .expect("Failed to get local data dir")
        .join("zenith_workspace");
    let _ = std::fs::create_dir_all(&path);

    let c_file = path.join("main.c");
    if !c_file.exists() {
        let _ = std::fs::write(
            &c_file,
            "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World! (C)\\n\");\n    return 0;\n}",
        );
    }
    let cpp_file = path.join("main.cpp");
    if !cpp_file.exists() {
        let _ = std::fs::write(
            &cpp_file,
            "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World! (C++)\" << std::endl;\n    return 0;\n}",
        );
    }
    let rs_file = path.join("main.rs");
    if !rs_file.exists() {
        let _ = std::fs::write(
            &rs_file,
            "fn main() {\n    println!(\"Hello, World! (Rust)\");\n}",
        );
    }
    let html_file = path.join("index.html");
    if !html_file.exists() {
        let _ = std::fs::write(
            &html_file,
            "<!DOCTYPE html>\n<html>\n<head><title>Hello</title></head>\n<body>\n    <h1>Hello, World! (HTML)</h1>\n</body>\n</html>",
        );
    }
}

pub fn run() {
    tauri::Builder::default()
        .manage(PtyState::new())
        .setup(|app| {
            // SỬA LỖI Ở ĐÂY: Bỏ dấu & đi
            init_workspace(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fs::list_directory,
            commands::fs::read_file,
            commands::fs::save_file,
            commands::fs::create_new_file,
            commands::fs::create_new_folder,
            commands::fs::get_default_workspace,
            commands::fs::get_compiler_path,
            commands::fs::install_compiler,
            commands::pty::start_pty,
            commands::pty::write_to_pty,
            commands::ai::ask_ai
        ])
        .run(tauri::generate_context!())
        .expect("Error while running ZenithIDE");
}
