use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs, process};

use anyhow::{anyhow, Context};
use fs2::FileExt;
use log::info;
use rustix::process::{kill_process, Pid, Signal};

pub fn kill_from_pid_file(pid_file: &mut fs::File) -> anyhow::Result<String> {
    let mut pid_str = String::new();
    pid_file.read_to_string(&mut pid_str)?;
    let pid: u32 = pid_str.parse()?;
    info!("try killing the old instance with pid {pid}");
    unsafe {
        let _ = kill_process(
            Pid::from_raw(pid).context("pid conversion error")?,
            Signal::Int,
        );
    }
    Ok(pid_str)
}

pub fn write_pid_to_file(pid_file_path: &Path) -> anyhow::Result<()> {
    fs::write(pid_file_path, process::id().to_string()).map_err(anyhow::Error::msg)
}

pub fn get_instance() -> anyhow::Result<fs::File> {
    get_instance_(0)
}

// FIXME refactor
pub fn get_instance_(try_index: u32) -> anyhow::Result<fs::File> {
    let pid_file_path = env::temp_dir().join("mouse_actions.pid");

    if let Ok(mut pid_file) = fs::File::open(&pid_file_path) {
        if pid_file.try_lock_exclusive().is_ok() {
            write_pid_to_file(&pid_file_path)?;
            Ok(pid_file)
        } else {
            info!("another instance is running ! try {try_index}");
            if try_index > 10 {
                Err(anyhow!("another instance is running !"))
            } else {
                kill_from_pid_file(&mut pid_file)?;
                sleep(Duration::from_millis(1000));
                get_instance_(try_index + 1)
            }
        }
    } else {
        let res = fs::File::create(&pid_file_path);
        if let Ok(pid_file) = res {
            pid_file.lock_exclusive()?;
            write_pid_to_file(&pid_file_path)?;
            Ok(pid_file)
        } else {
            res.map_err(anyhow::Error::msg)
        }
    }
}
