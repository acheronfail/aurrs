use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

use anyhow::{anyhow, Result};
use clap::crate_name;
use dirs::home_dir;

const KEYTAR_SERVICE_NAME: &str = crate_name!();

fn prompt_from_stdin(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}

fn get_password(user: &str) -> Result<String> {
    match keytar::find_password(KEYTAR_SERVICE_NAME) {
        Ok(result) if result.success => Ok(result.password),
        Ok(_) => {
            // No password was returned, prompt for one and then set it.
            if let Ok(pass) = rpassword::read_password_from_tty(Some("password: ")) {
                keytar::set_password(KEYTAR_SERVICE_NAME, user, &pass)?;
                Ok(pass)
            } else {
                Err(anyhow!("failed prompting user for password"))
            }
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn get_credentials() -> Result<(String, String)> {
    let config_path = home_dir()
        .map(|p| p.join(".config").join(crate_name!()))
        .expect("failed to find home directory");

    // Create config file if it doesn't exist.
    if !config_path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&config_path)?;
    }

    // Read config file to get credentials.
    let config_string = fs::read_to_string(&config_path)?;
    match config_string.parse::<toml_edit::Document>() {
        Ok(mut doc) => {
            let user = {
                let mut user = None;
                // Read username from config file.
                if let toml_edit::Item::Value(value) = &doc["aur"]["username"] {
                    user = value.as_str().map(|s| s.to_owned());
                }

                // No username in config file, prompt for one.
                if user.is_none() {
                    let env_user = env::var("USER").ok();
                    let prompt = env_user
                        .as_ref()
                        .map(|u| format!("username ({}): ", u))
                        .unwrap_or_else(|| "username: ".to_owned());

                    let mut value = prompt_from_stdin(Some(prompt.as_str()))?;
                    if value.is_empty() {
                        if let Some(username) = env_user {
                            value = username;
                        } else {
                            return Err(anyhow!("failed to get username"));
                        }
                    }

                    // Save username in config file.
                    doc["aur"]["username"] = toml_edit::value(value.as_ref());
                    fs::write(&config_path, doc.to_string())?;
                    user = Some(value);
                }

                user.unwrap()
            };

            let pass = get_password(&user)?;
            Ok((user, pass))
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}
