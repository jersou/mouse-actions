#[macro_use]
extern crate lazy_static;

// use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use clap::Parser;
use env_logger::Env;
use log::{debug, error, trace};
use rdev::GrabError;

use crate::args::MouseActionsCommands;
use crate::single_instance::get_instance;

mod args;
mod binding;
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
    config::init_config_file_if_not_exists();
    let args: Arc<args::Args> = Arc::new(args::Args::parse());
    trace!("args = {args:#?}!");

    let config: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config::get_config()));
    config::watch_config(config.clone());

    // let mut last_loop: Option<Instant> = None;
    // loop {
    let res: Result<(), GrabError> = match args.command {
        Some(MouseActionsCommands::OpenConfig) => {
            config::open_config();
            std::process::exit(0);
        }
        Some(MouseActionsCommands::Trace) => {
            let _instance = get_instance().unwrap();
            grab::start_grab_binding(args.clone(), config.clone(), process_event::trace_event)
        }
        Some(MouseActionsCommands::Record) => {
            let _instance = get_instance().unwrap();
            println!(
                "Start record event : draw a shape with the {:?} button :",
                config.lock().unwrap().shape_button
            );
            grab::start_grab_binding(args.clone(), config.clone(), record::record_event)
        }
        Some(MouseActionsCommands::Start) | None => {
            let _instance = get_instance().unwrap();
            grab::start_grab_binding(args.clone(), config.clone(), process_event::process_event)
        }
        Some(MouseActionsCommands::ListBindings) => {
            config
                .lock()
                .unwrap()
                .bindings
                .iter()
                .for_each(|b| println!(" - {}\n    {:?}\n", b.comment, b.cmd));
            std::process::exit(0);
        }
    };
    if let Err(error) = res {
        error!("Grab Error: {:?}", error);
    }
    // match last_loop {
    //     None => thread::sleep(time::Duration::from_millis(100)),
    //     Some(last) => {
    //         let ms_elapsed = last.elapsed();
    //         if ms_elapsed.lt(&time::Duration::from_millis(1000)) {
    //             thread::sleep(time::Duration::from_millis(5000));
    //         } else {
    //             thread::sleep(time::Duration::from_millis(100))
    //         }
    //     }
    // }
    //     last_loop = Some(Instant::now());
    // }
}
