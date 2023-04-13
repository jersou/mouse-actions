// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_default_config_path() -> String {
    format!("get_default_config_path={:?}", mouse_actions::config::get_config_path(&None))
}

use mouse_actions;


fn main() {
let path = mouse_actions::config::get_config_path(&None);
    println!("{path:?}");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_default_config_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
