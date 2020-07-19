use std::process::Command;

use anyhow::Result;

pub fn do_pacman(args: Vec<String>) -> Result<()> {
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
