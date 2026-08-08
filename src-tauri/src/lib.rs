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
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(PtyState::new())
        .setup(|app| {
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
            commands::fs::get_app_version,
            commands::fs::get_system_info,
            commands::pty::start_pty,
            commands::pty::write_to_pty,
            commands::ai::ask_ai
        ])
        .run(tauri::generate_context!())
        .expect("Error while running ZenithIDE");
}
