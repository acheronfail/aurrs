use std::ffi::OsStr;
use std::process::Command;

use anyhow::Result;

pub fn pacman<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new("sudo").arg("pacman").args(args).spawn()?;
    let status = child.wait()?;
    if !status.success() {
        if let Some(code) = status.code() {
            eprintln!("pacman exited with code: {}", code);
        } else {
            eprintln!("an unknown error occurred");
        }

        std::process::exit(1);
    }

    Ok(())
}

pub fn sync<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    todo!()
}

pub fn remove<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    todo!()
}
