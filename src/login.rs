use std::fs::OpenOptions;
use std::io::{self, Write};

use anyhow::{anyhow, Result};
use dirs::home_dir;
use envfile::EnvFile;
use reqwest::multipart::Form;
use reqwest::Client;
use scraper::Html;

use crate::aur;

fn prompt_from_stdin(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}

/// Reads the user's AUR credentials from ~/.config/aurvote.
/// Note that the format is compatible with the `aurvote` tool.
fn get_credentials() -> Result<(String, String)> {
    match home_dir().map(|p| p.join(".config").join("aurvote")) {
        Some(config_path) => {
            let config_path_exists = config_path.exists();
            if config_path_exists {
                let map = EnvFile::new(&config_path)?.store;
                if let (Some(user), Some(pass)) = (map.get("user"), map.get("pass")) {
                    return Ok((user.to_owned(), pass.to_owned()));
                }
            }

            eprintln!("no config file found, asking for credentials");
            let user = prompt_from_stdin(Some("username: "));
            let pass = rpassword::read_password_from_tty(Some("password: "));
            let save = prompt_from_stdin(Some("save? (Y/n): "));
            if let (Ok(user), Ok(pass), Ok(save)) = (user, pass, save) {
                match &save.trim()[..] {
                    "" | "y" | "ye" | "yes" | "Y" | "YE" | "YES" => {
                        if !config_path_exists {
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(&config_path)?;
                        }

                        let mut conf = EnvFile::new(&config_path)?;
                        conf.update("user", &user);
                        conf.update("pass", &pass);
                        conf.write()?;
                        eprintln!("saved config to {}", config_path.display());
                    }
                    _ => {}
                }

                Ok((user, pass))
            } else {
                Err(anyhow!("failed getting credentials"))
            }
        }
        None => Err(anyhow!("failed to find home directory")),
    }
}

/// Logs in to the AUR with the given client, so future requests are authenticated with a cookie.
pub async fn do_login(client: &Client) -> Result<()> {
    let (user, pass) = get_credentials()?;
    let form = Form::new()
        .text("remember_me", "on")
        .text("user", user)
        .text("passwd", pass);

    eprintln!("Logging in...");
    let resp = client
        .post(&format!("{}/login", aur::AUR_BASE_URL))
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    if let Some(li) = document.select(&aur::LOGIN_ERROR_SELECTOR).next() {
        return Err(anyhow!(
            "failed to login to AUR: {}",
            li.text().collect::<String>()
        ));
    }

    Ok(())
}
