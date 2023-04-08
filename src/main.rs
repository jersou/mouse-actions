#[macro_use]
extern crate lazy_static;

use std::process::exit;
use std::{thread, time};
// use std::time::Instant;
use std::sync::{Arc, Mutex};

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, trace};
use rdev::GrabError;

use crate::args::MouseActionsCommands;
use crate::single_instance::get_instance;

mod args;
mod binding;
mod check_input_perm;
mod cmd_from_string;
mod compare_angles;
mod config;
mod event;
mod grab;
mod listen;
mod points_to_angles;
mod process_event;
mod record;
mod single_instance;
mod trace_svg;

fn main() {
    debug!("Start main");
    // FIXME : to avoid "Release Enter key event" to be lost (if run the script by Enter press in a terminal)
    thread::sleep(time::Duration::from_millis(300));

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Arc<args::Args> = Arc::new(args::Args::parse());
    trace!("args = {args:#?}!");

    let config_path_from_args = args.config_path.clone();
    let config_path = config::get_config_path(&config_path_from_args);
    config::init_config_file_if_not_exists(&config_path);

    let config: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config::get_config(&config_path)));
    config::watch_config(config.clone(), config_path.clone());

    // let mut last_loop: Option<Instant> = None;
    // loop {
    let res: Result<(), GrabError> = match args.command {
        Some(MouseActionsCommands::OpenConfig) => {
            config::open_config(config_path);
            exit(0);
        }
        Some(MouseActionsCommands::Trace) => {
            let _instance = get_instance().unwrap();
            grab::start_grab_binding(args.clone(), config, process_event::trace_event)
        }
        Some(MouseActionsCommands::Record) => {
            let _instance = get_instance().unwrap();
            println!(
                "Start record event : draw a shape with the {:?} button :",
                config.lock().unwrap().shape_button
            );
            grab::start_grab_binding(args.clone(), config, record::record_event)
        }
        Some(MouseActionsCommands::Start) | None => {
            let _instance = get_instance().unwrap();
            grab::start_grab_binding(args.clone(), config, process_event::process_event)
        }
        Some(MouseActionsCommands::ListBindings) => {
            config
                .lock()
                .unwrap()
                .bindings
                .iter()
                .for_each(|b| println!(" - {}\n    {:?}\n", b.comment, b.cmd));
            exit(0);
        }

        Some(MouseActionsCommands::GrabOneEvent) => {
            let _instance = get_instance().unwrap();
            grab::start_grab_binding(args.clone(), config, process_event::grab_one_event)
        }
        Some(MouseActionsCommands::Stop) => {
            if single_instance::kill().unwrap_or(false) {
                info!("mouse_actions successfully stopped");
                exit(0);
            } else {
                error!("mouse_actions stop error !");
                exit(1);
            }
        }
        Some(MouseActionsCommands::Status) => {
            if single_instance::is_running() {
                info!("mouse_actions is running");
                exit(0);
            } else {
                info!("mouse_actions is stopped");
                exit(1);
            }
        }
    };
    if let Err(error) = res {
        error!("Grab Error: {:#?}", error);
    }
}
