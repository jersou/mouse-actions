use std::io;
use std::io::{ErrorKind, Read};
use std::ops::Deref;
use std::process::exit;
use std::sync::Arc;

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, trace};
use rdev::GrabError;

use crate::args::{Args, MouseActionsCommands};
use crate::config::get_config_from_args;
use crate::single_instance::get_instance;
use crate::{config, grab, process_event, record, single_instance};

#[cfg(target_os = "linux")]
static DEV_PATH: &str = "/dev/input";

pub fn main() {
    debug!("Start main");
    trace!("version : {}", get_version());
    // TODO add this init to config_editor
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Arc<Args> = Arc::new(Args::parse());
    trace!("args = {args:#?}!");
    process_args(args)
}

pub fn get_version() -> String {
    let version: String = format!(
        "{}-{} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_DESCRIBE"),
        env!("VERGEN_BUILD_DATE")
    );
    version
}

pub fn process_args(args: Arc<Args>) {
    if args.version {
        println!("{}", get_version());
    } else {
        // TODO enum resut (nor only GrabError)
        let res: Result<(), GrabError> = match args.command {
            Some(MouseActionsCommands::Start) | None => start(&args),
            Some(MouseActionsCommands::Trace) => trace(&args),
            Some(MouseActionsCommands::Record) => record(&args),
            Some(MouseActionsCommands::ListBindings) => list_bindings(&args),
            Some(MouseActionsCommands::GrabOneEvent) => grab_one_event(&args),
            Some(MouseActionsCommands::Stop) => stop(),
            Some(MouseActionsCommands::Status) => status(),
            Some(MouseActionsCommands::ShowConfig) => show_config(&args),
            Some(MouseActionsCommands::SetConfig) => set_config(&args),
        };
        if let Err(error) = res {
            process_error(&error);
            exit(4);
        }
    }
}

pub fn trace(args: &Arc<Args>) -> Result<(), GrabError> {
    let _instance = get_instance().unwrap();
    let config = get_config_from_args(args, false);
    grab::start_grab_binding(args.clone(), config, process_event::trace_event)
}

pub fn record(args: &Arc<Args>) -> Result<(), GrabError> {
    let _instance = get_instance().unwrap();
    let config = get_config_from_args(&args, true);
    println!(
        "Start record event : draw a shape with the {:?} button :",
        config.lock().unwrap().shape_button
    );
    grab::start_grab_binding(args.clone(), config, record::record_event)
}

pub fn start(args: &Arc<Args>) -> Result<(), GrabError> {
    let _instance = get_instance().unwrap();
    let config = get_config_from_args(&args, true);
    grab::start_grab_binding(args.clone(), config, process_event::process_event)
}

pub fn list_bindings(args: &Arc<Args>) -> Result<(), GrabError> {
    let config = get_config_from_args(&args, false);
    config
        .lock()
        .unwrap()
        .bindings
        .iter()
        .for_each(|b| println!(" - {}\n    {:?}\n", b.comment, b.cmd));
    Ok(())
}

pub fn grab_one_event(args: &Arc<Args>) -> Result<(), GrabError> {
    let _instance = get_instance().unwrap();
    let config = get_config_from_args(&args, false);
    grab::start_grab_binding(args.clone(), config, process_event::grab_one_event)
}

pub fn stop() -> Result<(), GrabError> {
    if single_instance::kill().unwrap_or(false) {
        info!("mouse_actions successfully stopped");
        exit(0);
    } else {
        error!("mouse_actions stop error !");
        exit(1);
    }
}

pub fn status() -> Result<(), GrabError> {
    if single_instance::is_running() {
        info!("mouse_actions is running");
        exit(0);
    } else {
        info!("mouse_actions is stopped");
        exit(1);
    }
}

pub fn show_config(args: &Arc<Args>) -> Result<(), GrabError> {
    let config = get_config_from_args(&args, false);
    let c = config.lock().unwrap();
    let serialized = serde_json::to_string_pretty(c.deref()).unwrap();
    println!("{serialized}");
    exit(0);
}

pub fn set_config(args: &Arc<Args>) -> Result<(), GrabError> {
    let mut stdin_str = String::new();
    io::stdin().read_to_string(&mut stdin_str).unwrap();
    // check the deserialization
    let config = config::load_from_str(&stdin_str);
    config::save_config(&config, &args.config_path);
    Ok(())
}

pub fn process_error(error: &GrabError) {
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
}
