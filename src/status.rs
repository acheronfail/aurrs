use anyhow::{anyhow, Result};
use futures::stream::futures_unordered::FuturesUnordered;
use reqwest::Client;
use scraper::Html;
use tokio::stream::StreamExt;

use crate::aur;
use crate::cli::VoteCommandOptions;

#[derive(Debug)]
pub struct PackageStatus {
    pub name: String,
    pub voted: bool,
    pub token: Option<String>,
}

pub async fn get_packages_statuses(
    client: &Client,
    options: &VoteCommandOptions,
) -> Result<Vec<PackageStatus>> {
    let mut futures = FuturesUnordered::new();

    for p in &options.packages {
        futures.push(get_package_status(&client, &p));
    }

    let mut results = Vec::new();
    while let Some(result) = futures.next().await {
        match result {
            Ok(value) => results.push(value),
            Err(e) => eprintln!("error querying package status: {}", e),
        }
    }

    Ok(results)
}

async fn get_package_status(client: &Client, pkg: &str) -> Result<PackageStatus> {
    let resp = client
        .get(&format!(
            "{}/packages/{}/",
            aur::AUR_BASE_URL,
            urlencoding::encode(pkg)
        ))
        .query(&[("setlang", "en")])
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let token = document
        .select(&aur::AUR_TOKEN_SELECTOR)
        .next()
        .and_then(|input| input.value().attr("value").map(|s| s.to_string()));

    if document.select(&aur::UNVOTE_SELECTOR).next().is_some() {
        Ok(PackageStatus {
            name: pkg.to_string(),
            voted: true,
            token,
        })
    } else if document.select(&aur::VOTE_SELECTOR).next().is_some() {
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
