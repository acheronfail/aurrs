use std::process::Command;
use std::thread::{sleep, spawn};
use std::time::Duration;

use anyhow::{anyhow, Result};

pub fn use_sudo() -> Result<()> {
    // Run thread in background.
    let _sudo_loop = spawn(|| loop_sudo());
    Ok(())
}

fn loop_sudo() -> Result<()> {
    loop {
        let output = Command::new("sudo").arg("-v").output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to use sudo: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        sleep(Duration::from_secs(290));
    }
}
