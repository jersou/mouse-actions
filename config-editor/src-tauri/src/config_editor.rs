// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mouse_actions;

#[tauri::command]
fn get_default_config_path() -> String {
    format!(
        "get_default_config_path={:?}",
        mouse_actions::config::get_config_path(&None)
    )
}

#[tauri::command]
fn get_version() -> String {
    format!("v{}", mouse_actions::process_args::get_version())
}

#[tauri::command]
fn get_json_config() -> String {
    let args = mouse_actions::args::parse();
    mouse_actions::config::get_json_config(&args)
}

pub fn open_config_editor() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_default_config_path,
            get_version,
            get_json_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
