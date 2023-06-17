// Prevents additional console window on Windows in release, DO NOT REMOVE!!

use mouse_actions;
use mouse_actions::config;

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

#[tauri::command(async)]
fn stop() {
    let ma_exe_path = std::env::current_exe().unwrap();
    mouse_actions::process_event::process_cmd(vec![
        ma_exe_path.to_str().unwrap().to_string(),
        String::from("stop"),
    ])
}

#[tauri::command(async)]
fn start() {
    let ma_exe_path = std::env::current_exe().unwrap();
    let args = mouse_actions::args::parse();
    let mut cmd: Vec<String> = Vec::new();
    cmd.push(ma_exe_path.to_str().unwrap().to_string());
    // Args::
    // FIXME : generic forward args
    if args.no_listen {
        cmd.push(String::from("--no-listen"));
    }
    if args.config_path.is_some() {
        cmd.push(String::from("--config-path"));
        cmd.push(args.config_path.unwrap());
    }
    if args.log_level.is_some() {
        cmd.push(String::from("--log-level"));
        cmd.push(args.log_level.unwrap());
    }
    cmd.push(String::from("start"));
    mouse_actions::process_event::process_cmd(cmd)
}

#[tauri::command(async)]
fn get_config() -> config::Config {
    let args = mouse_actions::args::parse();
    let config_path = config::get_config_path(&args.config_path);
    config::init_config_file_if_not_exists(&config_path);
    config::get_config(&config_path)
}

#[tauri::command(async)]
fn save_config(new_config: config::Config) {
    let args = mouse_actions::args::parse();
    config::save_config(&new_config, &args.config_path)
}

pub fn open_config_editor() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_default_config_path,
            get_version,
            get_config,
            save_config,
            stop,
            start
        ])
        .setup(|app| {
            tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
                .title(format!(
                    "Mouse Actions Config Editor v{}",
                    mouse_actions::process_args::get_version()
                ))
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
