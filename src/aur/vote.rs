use anyhow::{anyhow, Result};
use clap::crate_name;
use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;
use scraper::Html;

use crate::aur::constants::{AUR_BASE_URL, AUR_TOKEN_SELECTOR, UNVOTE_SELECTOR, VOTE_SELECTOR};
use crate::model::AurPackage;

pub fn change_package_vote(
    client: &Client,
    pkg_name: &str,
    token: String,
    vote: bool,
) -> Result<()> {
    let pkgs = raur::info(&[pkg_name])?;
    let pkg = match pkgs.first() {
        Some(pkg) => pkg,
        None => return Err(anyhow!("Failed to find package with name: {}", pkg_name)),
    };

    let (action, url_path) = if vote {
        ("do_Vote", "vote")
    } else {
        ("do_UnVote", "unvote")
    };

    let form = Form::new().text("token", token).text(action, crate_name!());
    client
        .post(&format!(
            "{}/pkgbase/{}/{}/",
            AUR_BASE_URL,
            urlencoding::encode(&pkg.package_base),
            urlencoding::encode(url_path)
        ))
        .multipart(form)
        .send()?;

    Ok(())
}

pub fn get_vote_package_status(client: &Client, pkg: &str, fetch_meta: bool) -> Result<AurPackage> {
    let resp = client
        .get(&format!(
            "{}/packages/{}/",
            AUR_BASE_URL,
            urlencoding::encode(pkg)
        ))
        .query(&[("setlang", "en")])
        .send()?
        .text()?;

    let document = Html::parse_document(&resp);
    let token = document
        .select(&AUR_TOKEN_SELECTOR)
        .next()
        .and_then(|input| input.value().attr("value").map(|s| s.to_string()));

    if document.select(&UNVOTE_SELECTOR).next().is_some() {
        AurPackage::new(pkg, token, true, fetch_meta)
    } else if document.select(&VOTE_SELECTOR).next().is_some() {
        AurPackage::new(pkg, token, false, fetch_meta)
    } else {
        Err(anyhow!(
            "[{}] failed to load package page, does it exist?",
            pkg
        ))
    }
}
