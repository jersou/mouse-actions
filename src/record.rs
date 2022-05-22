use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::args::Args;
use crate::binding::Binding;
use crate::cmd_from_string::cmd_from_str;
use crate::config::{save_config, Config};
use crate::event::{ClickEvent, MouseButton, PressState};

lazy_static! {
    static ref RECORD_IN_PROGRESS: Mutex<bool> = Mutex::new(false);
}

pub fn record_event(config: Arc<Mutex<Config>>, event: ClickEvent, _args: Arc<Args>) -> bool {
    // ignore events if record in progress, or left click
    if *RECORD_IN_PROGRESS.lock().unwrap()
        || event.button == MouseButton::Left
            && event.shape.is_empty()
            && event.edges.is_empty()
            && event.modifiers.is_empty()
    {
        true
    } else {
        if config.lock().unwrap().shape_button != event.button
            || event.event_type != PressState::Press
        {
            *RECORD_IN_PROGRESS.lock().unwrap() = true;
            thread::spawn(move || {
                println!("record_event : {:?}", event);
                println!("command to bind (empty to cancel) : ");
                let mut input_string = String::new();
                stdin()
                    .read_line(&mut input_string)
                    .ok()
                    .expect("Failed to read line");
                let input_string = input_string.trim();
                *RECORD_IN_PROGRESS.lock().unwrap() = false;
                if !input_string.is_empty() {
                    println!("input_string : {}", input_string);
                    let binding = Binding {
                        event,
                        cmd: cmd_from_str(input_string),
                    };

                    config.lock().unwrap().bindings.push(binding);
                    save_config(&config.lock().unwrap());
                }
            });
        }
        false
    }
}
