#[macro_use]
extern crate lazy_static;

use std::io::ErrorKind;
use std::ops::Deref;
use std::process::exit;
use std::sync::{Arc, Mutex};

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, trace};
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

#[cfg(target_os = "linux")]
static DEV_PATH: &str = "/dev/input";

// FIXME refactor

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    debug!("Start main");
    let version: String = format!(
        "{}-{} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_DESCRIBE"),
        env!("VERGEN_BUILD_DATE")
    );
    trace!("version : {version}");

    let args: Arc<args::Args> = Arc::new(args::Args::parse());
    trace!("args = {args:#?}!");
    // FIXME refactor
    if args.version {
        println!("{version}");
        exit(0);
    }

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
        Some(MouseActionsCommands::ShowConfig) => {
            let c = config.lock().unwrap();
            let serialized = serde_json::to_string_pretty(c.deref()).unwrap();
            println!("{serialized}");
            exit(0);
        }
    };
    if let Err(error) = res {
        error!("Grab Error: {:#?}", error);

        #[cfg(target_os = "linux")]
        {
            if let GrabError::IoError(io_err) = error {
                if io_err.kind() == ErrorKind::PermissionDenied {
                    error!("The user must be in the file group of {DEV_PATH} files, usually 'input' or 'plugdev' :");
                    error!("  $ sudo usermod -a -G input $USER");
                    error!("  $ sudo usermod -a -G plugdev $USER");
                    error!(
                        r#"  $ sudo tee /etc/udev/rules.d/80-mouse-actions.rules <<<'KERNEL=="uinput", SUBSYSTEM=="misc", TAG+="uaccess", OPTIONS+="static_node=uinput"'"#
                    );
                    error!("Then restart to apply this user modifications.");
                }
            }
        }
        exit(4);
    }
}
