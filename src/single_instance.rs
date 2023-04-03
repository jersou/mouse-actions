use std::thread::sleep;
use std::time::Duration;
use std::{env, fs, process};

use anyhow::{anyhow, Context};
use log::info;
use rustix::process::{kill_process, Pid, Signal};
use single_instance::SingleInstance;
use tempfile::{Builder, NamedTempFile};

pub fn get_instance() -> anyhow::Result<(SingleInstance, NamedTempFile)> {
    get_instance_(0)
}

pub fn get_instance_(try_index: u32) -> anyhow::Result<(SingleInstance, NamedTempFile)> {
    let instance = SingleInstance::new("mouse_actions")?;

    let pid_file_path = env::temp_dir().join("mouse_actions.pid");

    if instance.is_single() {
        // write the pid to <temp dir>/mouse_actions.pid file
        let _ = fs::remove_file(pid_file_path);
        // FIXME use pid_file_path
        let pid_file = Builder::new()
            .prefix("mouse_actions")
            .rand_bytes(0)
            .suffix(".pid")
            .tempfile()?;
        fs::write(&pid_file, process::id().to_string())?;
        Ok((instance, pid_file))
    } else if try_index > 10 {
        Err(anyhow!("another instance is running !"))
    } else {
        info!("another instance is running ! try {try_index}");
        let pid_str = fs::read_to_string(pid_file_path)?;
        let pid: u32 = pid_str.parse()?;
        info!("try killing the old instance with pid {pid}");
        unsafe {
            let _ = kill_process(
                Pid::from_raw(pid).context("pid conversion error")?,
                Signal::Int,
            );
        }
        sleep(Duration::from_millis(1000));
        get_instance_(try_index + 1)
    }
}
