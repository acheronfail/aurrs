use anyhow::{anyhow, Result};
use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;
use scraper::Html;

use crate::aur::constants::{AUR_BASE_URL, LOGIN_ERROR_SELECTOR};
use crate::credentials::get_credentials;

/// Logs in to the AUR with the given client, so future requests are authenticated with a cookie.
pub fn login_client_to_aur(client: &Client) -> Result<()> {
    let (user, pass) = get_credentials()?;
    let form = Form::new()
        .text("remember_me", "on")
        .text("user", user)
        .text("passwd", pass);

    eprintln!("Logging in...");
    let resp = client
        .post(&format!("{}/login", AUR_BASE_URL))
        .multipart(form)
        .send()?
        .text()?;

    let document = Html::parse_document(&resp);
    if let Some(li) = document.select(&LOGIN_ERROR_SELECTOR).next() {
        return Err(anyhow!(
            "failed to login to AUR: {}",
            li.text().collect::<String>()
        ));
    }

    Ok(())
}
