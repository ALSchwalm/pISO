use error::{Result, ResultExt};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::process::Command;

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
