use anyhow::{anyhow, Result};
use clap::crate_name;
use reqwest::multipart::Form;
use reqwest::Client;
use scraper::Html;

use crate::aur::constants::{AUR_BASE_URL, AUR_TOKEN_SELECTOR, UNVOTE_SELECTOR, VOTE_SELECTOR};
use crate::model::{AurRpcInfo, AurRpcInfoResult, PackageStatus};

pub async fn change_package_vote(
    client: &Client,
    pkg: &str,
    token: String,
    vote: bool,
) -> Result<()> {
    let resp = client
        .get(&format!("{}/rpc.php", AUR_BASE_URL))
        .query(&[("type", "info"), ("arg", pkg)])
        .send()
        .await?
        .json::<AurRpcInfo>()
        .await?;

    let AurRpcInfoResult { package_base, .. } = resp.results;

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
            urlencoding::encode(&package_base),
            urlencoding::encode(url_path)
        ))
        .multipart(form)
        .send()
        .await?;

    Ok(())
}

pub async fn get_package_status(client: &Client, pkg: &str) -> Result<PackageStatus> {
    let resp = client
        .get(&format!(
            "{}/packages/{}/",
            AUR_BASE_URL,
            urlencoding::encode(pkg)
        ))
        .query(&[("setlang", "en")])
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let token = document
        .select(&AUR_TOKEN_SELECTOR)
        .next()
        .and_then(|input| input.value().attr("value").map(|s| s.to_string()));

    if document.select(&UNVOTE_SELECTOR).next().is_some() {
        Ok(PackageStatus {
            name: pkg.to_string(),
            voted: true,
            token,
        })
    } else if document.select(&VOTE_SELECTOR).next().is_some() {
        Ok(PackageStatus {
            name: pkg.to_string(),
            voted: false,
            token,
        })
    } else {
        Err(anyhow!(
            "[{}] failed to load package page, does it exist?",
            pkg
        ))
    }
}
