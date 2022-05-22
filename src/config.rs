use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use log::{error, info};
use notify::op::CLOSE_WRITE;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};

use crate::binding::Binding;
use crate::event::MouseButton;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub shape_button: MouseButton,
    pub bindings: Vec<Binding>,
}

pub fn load(file_path: &str) -> Config {
    let content = fs::read_to_string(file_path).unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();
    config
}

pub fn get_config_path() -> PathBuf {
    let config_path: PathBuf = [
        dirs_sys::home_dir().unwrap().to_str().unwrap(),
        ".config",
        "mouse-actions.json",
    ]
    .iter()
    .collect();
    config_path
}

pub fn get_config() -> Config {
    let config_path = get_config_path();
    load(config_path.to_str().unwrap())
}

pub fn watch_config(config: Arc<Mutex<Config>>) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher
            .watch(get_config_path(), RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent {
                    path: Some(_),
                    op: Ok(op),
                    cookie: _,
                }) => {
                    if op == CLOSE_WRITE {
                        info!("Reload the config !");
                        *config.lock().unwrap() = get_config();
                    }
                }
                Ok(event) => info!("watcher: broken event: {:?}", event),
                Err(e) => {
                    error!("watcher: watch error: {:?}", e);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    });
}

pub fn init_config_file_if_not_exists() {
    let config_path = get_config_path();
    if !config_path.exists() {
        let empty_config = Config {
            shape_button: MouseButton::Right,
            bindings: vec![],
        };
        let serialized = serde_json::to_string_pretty(&empty_config).unwrap();

        let mut config_file = match File::create(&config_path) {
            Err(err) => panic!(
                "couldn't create config file {}: {err}",
                config_path.display()
            ),
            Ok(file) => file,
        };

        match config_file.write_all(serialized.as_bytes()) {
            Err(err) => panic!("couldn't write to {}: {err}", config_path.display()),
            Ok(_) => println!("successfully wrote to {}", config_path.display()),
        }
    }
}

pub fn save_config(config: &Config) {
    let config_path = get_config_path();
    let serialized = serde_json::to_string_pretty(&config).unwrap();

    let mut config_file = fs::OpenOptions::new()
        .write(true)
        .open(&config_path)
        .unwrap();

    match config_file.write_all(serialized.as_bytes()) {
        Err(err) => panic!("couldn't write to {}: {err}", config_path.display()),
        Ok(_) => println!("successfully wrote to {}", config_path.display()),
    }
}

pub fn open_config() {
    let config_path = get_config_path();
    println!("Open config file with xdg-open : {:?}", config_path);
    Command::new("xdg-open")
        .args(config_path.to_str())
        .status()
        .map_err(|err| error!("Command err: {:?}", err))
        .ok();
}

#[cfg(test)]
mod tests {
    use crate::event::{ClickEvent, Edge, KeyboardModifier, MouseButton, PressState};

    use super::*;

    #[test]
    fn test_json_serialize() {
        let config = Config {
            shape_button: MouseButton::Right,
            bindings: vec![Binding {
                event: ClickEvent {
                    button: MouseButton::Left,
                    edges: vec![Edge::Top, Edge::Left],
                    modifiers: vec![KeyboardModifier::ControlLeft],
                    event_type: PressState::Press,
                    shape: vec![0.0, 1.0, 2.0],
                },
                cmd: vec![String::from("xlogo")],
            }],
        };

        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let expected = r#"{
  "shape_button": "Right",
  "bindings": [
    {
      "event": {
        "button": "Left",
        "edges": [
          "Top",
          "Left"
        ],
        "modifiers": [
          "ControlLeft"
        ],
        "event_type": "Press",
        "shape": [
          0.0,
          1.0,
          2.0
        ]
      },
      "cmd": [
        "xlogo"
      ]
    }
  ]
}"#;
        println!("serialized = {}", serialized);
        assert_eq!(
            serialized
                .split('\n')
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
            expected
                .split('\n')
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
        );
    }

    #[test]
    fn test_json_deserialize() {
        let serialized = r#"{
  "shape_button": "Right",
  "bindings": [
    {
      "event": {
        "button": "Left",
        "edges": [
          "Top",
          "Left"
        ],
        "modifiers": [
          "ControlLeft"
        ],
        "event_type": "Press",
        "shape": [
          0.0,
          1.0,
          2.0
        ]
      },
      "cmd": [
        "xlogo"
      ]
    }
  ]
}"#;
        let config: Config = serde_json::from_str(serialized).unwrap();
        println!("config = {:?}", config);
        assert_eq!(config.shape_button, MouseButton::Right);
        let binding = &config.bindings[0];
        assert_eq!(binding.cmd[0], "xlogo");
        assert_eq!(binding.event.button, MouseButton::Left);
        assert_eq!(binding.event.edges[0], Edge::Top);
        assert_eq!(binding.event.edges[1], Edge::Left);
        assert_eq!(binding.event.modifiers[0], KeyboardModifier::ControlLeft);
        assert_eq!(binding.event.event_type, PressState::Press);
        assert_eq!(binding.event.shape, vec![0.0, 1.0, 2.0]);
    }
}
