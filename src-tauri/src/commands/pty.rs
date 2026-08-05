use crate::state::PtyState;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Write;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn start_pty(
    app: AppHandle,
    id: u32,
    cwd: String,
    state: State<'_, PtyState>,
) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let app_handle = app.clone();

    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_handle.emit(&format!("pty-{}", id), data);
                }
                Err(_) => break,
            }
        }
    });

    let mut cmd = if cfg!(target_os = "windows") {
        CommandBuilder::new("cmd.exe")
    } else {
        CommandBuilder::new("bash")
    };
    cmd.cwd(cwd);

    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

    state.masters.lock().unwrap().insert(id, pair.master);
    state.writers.lock().unwrap().insert(id, writer);
    Ok(())
}

#[tauri::command]
pub async fn write_to_pty(id: u32, data: String, state: State<'_, PtyState>) -> Result<(), String> {
    let mut writers = state.writers.lock().unwrap();
    if let Some(writer) = writers.get_mut(&id) {
        writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}
