use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;

use log::{debug, error};

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
                debug!("record_event : {:?}", event);
                println!("command to bind (empty to stop the record) : ");
                let mut input_string = String::new();
                stdin()
                    .read_line(&mut input_string)
                    .expect("Failed to read line");
                let cmd_string = input_string.trim();

                *RECORD_IN_PROGRESS.lock().unwrap() = false;
                if !cmd_string.is_empty() {
                    println!("comment : ");
                    let mut input_string = String::new();
                    stdin()
                        .read_line(&mut input_string)
                        .expect("Failed to read line");
                    let comment = input_string.trim().to_string();

                    let event = reduce_shape_precision(event);

                    match cmd_from_str(cmd_string) {
                        Ok(cmd) => {
                            let binding = Binding {
                                comment,
                                event,
                                cmd,
                            };
                            config.lock().unwrap().bindings.push(binding);
                            save_config(&config.lock().unwrap());
                            // FIXME
                            println!(
                                "\nStart record event : draw a shape with the {:?} button :",
                                config.lock().unwrap().shape_button
                            );
                        }
                        Err(err) => error!("Error while command parsing : {:#?}", err),
                    };
                } else {
                    std::process::exit(0);
                }
            });
        }
        false
    }
}

pub fn reduce_shape_precision(event: ClickEvent) -> ClickEvent {
    ClickEvent {
        shape: event
            .shape
            .iter()
            .map(|angle| (angle * 100.0).round() / 100.0)
            .collect(),
        ..event
    }
}
