#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};

use clap::Parser;
use env_logger::Env;
use log::{error, trace};
use rdev::GrabError;

use crate::args::MouseActionsCommands;

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

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    config::init_config_file_if_not_exists();
    let args: Arc<args::Args> = Arc::new(args::Args::parse());
    trace!("args = {:#?}!", args);

    let config: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config::get_config()));
    config::watch_config(config.clone());

    let res: Result<(), GrabError> = match args.command {
        Some(MouseActionsCommands::OpenConfig) => {
            config::open_config();
            Ok(())
        }
        Some(MouseActionsCommands::Trace) => {
            grab::start_grab_binding(args, config, process_event::trace_event)
        }
        Some(MouseActionsCommands::Record) => {
            grab::start_grab_binding(args, config, record::record_event)
        }
        Some(MouseActionsCommands::Start) | None => {
            grab::start_grab_binding(args, config, process_event::process_event)
        }
    };
    if let Err(error) = res {
        error!("Grab Error: {:?}", error);
        std::process::exit(1)
    }
}
