#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let window = app.get_window("main").expect("get window error");
            set_shadow(&window, true).expect("error while set_shadow");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
