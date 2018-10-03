use config;
use error::{ErrorKind, Result, ResultExt};
use lvm;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::process::Command;
use std::time::{Duration, Instant};
use std::path::Path;
use std::thread;

pub fn run_check_output<I, S1, S2>(program: S1, args: I) -> Result<String>
where
    I: IntoIterator<Item = S2> + Debug + Clone,
    S1: AsRef<OsStr> + Debug + Clone,
    S2: AsRef<OsStr> + Debug + Clone,
{
    let output = Command::new(program.clone())
        .args(args.clone())
        .output()
        .chain_err(|| format!("Failed to start command: {:?} ({:?})", program, args))?;

    if !output.status.success() {
        Err(format!(
            "Command {:?} ({:?}) failed: {}",
            program,
            args,
            String::from_utf8_lossy(&output.stderr)
        ).into())
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

pub fn wait_for_path<P>(path: P, total_wait: Duration) -> Result<()>
where
    P: AsRef<Path>,
{
    let now = Instant::now();
    let wait = Duration::from_millis(50);

    while now.elapsed() < total_wait {
        if path.as_ref().exists() {
            return Ok(());
        }
        thread::sleep(wait);
    }
    Err(format!("timeout while waiting for {}", path.as_ref().display()).into())
}

pub fn next_available_drive_name(vg: &lvm::VolumeGroup) -> Result<String> {
    let volumes = vg.volumes()?;
    for num in 1.. {
        if volumes
            .iter()
            .all(|vol| vol.name != format!("Drive{}", num))
        {
            return Ok(format!("Drive{}", num));
        }
    }
    Err(ErrorKind::Msg("Failed to find valid drive number".into()).into())
}

pub fn translate_drive_name(name: &str, config: &config::Config) -> String {
    for drive in config.drive.as_ref().unwrap_or(&vec![]).iter() {
        if drive.name == name {
            return drive.newname.clone();
        } else if format!("{}-backup", drive.name) == name {
            return format!("{}-backup", drive.newname.clone());
        }
    }
    name.into()
}
