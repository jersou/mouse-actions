use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;

use log::{debug, info};

use crate::args::Args;
use crate::binding::Binding;
use crate::cmd_str_spliter::str_cmd_to_array;
use crate::config::{save_config, Config};
use crate::event;
use crate::event::EventType::Shape;
use crate::event::{ClickEvent, MouseButton};
use crate::grab::normalize_points;

lazy_static::lazy_static! {
    static ref RECORD_IN_PROGRESS: Mutex<bool> = Mutex::new(false);
}

pub fn record_event(config: Arc<Mutex<Config>>, event: ClickEvent, args: Arc<Args>) -> bool {
    // ignore events if record in progress, or left click
    if *RECORD_IN_PROGRESS.lock().unwrap()
        || event.button == MouseButton::Left
            && event.shapes_angles.is_empty()
            && event.edges.is_empty()
            && event.modifiers.is_empty()
    {
        true
    } else {
        if config.lock().unwrap().shape_button != event.button
            || event.event_type != event::EventType::Press
        {
            *RECORD_IN_PROGRESS.lock().unwrap() = true;
            thread::Builder::new()
                .name("record_event".to_string())
                .spawn(move || {
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

                        let mut event = reduce_shape_precision(event);
                        if let Some(shapes_xy) = event.shapes_xy.first() {
                            event.shapes_xy = vec![normalize_points(&shapes_xy, false)];
                            event.event_type = Shape;
                        }

                        let binding = Binding {
                            comment,
                            event,
                            cmd: str_cmd_to_array(cmd_string),
                            cmd_str: String::new(),
                        };
                        info!("push : {binding:#?}");
                        config.lock().unwrap().bindings.push(binding);
                        save_config(&config.lock().unwrap(), &args.config_path);
                        // FIXME
                        println!(
                            "\nStart record event : draw a shape with the {:?} button :",
                            config.lock().unwrap().shape_button
                        );
                    } else {
                        std::process::exit(0);
                    }
                })
                .unwrap();
        }
        false
    }
}

pub fn reduce_shape_precision(event: ClickEvent) -> ClickEvent {
    ClickEvent {
        shapes_angles: event
            .shapes_angles
            .iter()
            .map(|shape_angles| {
                shape_angles
                    .iter()
                    .map(|angle| (angle * 100.0).round() / 100.0)
                    .collect()
            })
            .collect(),
        ..event
    }
}
