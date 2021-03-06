use std::ffi::OsStr;
use std::process::Command;

use anyhow::Result;

pub fn pacman<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new("sudo")
        .arg("pacman")
        .args(args)
        .spawn()?
        .wait()?;
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
