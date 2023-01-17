#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env;
use redb::Error;
use tauri::Manager;
use ruoyi_gen::data::Data;
use ruoyi_gen::rst::Rst;
use window_shadows::set_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn test(msg: &str) -> String {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().unwrap();
    dir.to_str().unwrap().to_owned()
}

#[tauri::command]
fn set_data(key: &str, val: &str) -> Rst {
    match Data::set(key, val) {
        Ok(_) => Rst::ok(None),
        Err(e) => Rst::err(e.to_string())
    }
}

#[tauri::command]
fn get_data(key: &str) -> Rst {
    match Data::get(key) {
        Ok(d) => Rst::ok(d),
        Err(e) => Rst::err(e.to_string())
    }
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("error while set_shadow" );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test,set_data,get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
