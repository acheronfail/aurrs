use std::process::Command;
use std::thread::{sleep, spawn};
use std::time::Duration;

use anyhow::{anyhow, Result};

const SUDO_REFRESH_INTERVAL: Duration = Duration::from_secs(290);

pub fn use_sudo() -> Result<()> {
    // Run thread in background.
    spawn(|| loop_sudo());
    Ok(())
}

/// Calls `sudo -v` in a loop.
fn loop_sudo() -> Result<()> {
    loop {
        let output = Command::new("sudo").arg("-v").output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "Failed to use sudo: {}",
                if stderr.is_empty() {
                    String::from_utf8_lossy(&output.stdout)
                } else {
                    stderr
                }
            ));
        }

        sleep(SUDO_REFRESH_INTERVAL);
    }
}
