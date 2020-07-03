use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::Html;

use crate::aur;
use crate::cli::CommandOptions;

#[derive(Debug)]
pub struct PackageStatus {
    pub voted: bool,
    pub token: Option<String>,
}

pub async fn do_status(client: &Client, options: CommandOptions) -> Result<()> {
    let padding = options.longest_package_len();

    let packages_and_statuses = get_packages_statuses(client, &options).await?;
    for (pkg, s) in packages_and_statuses {
        println!(
            "{:width$}: {}",
            pkg,
            if s.voted { "Voted!" } else { "Not voted." },
            width = padding
        );
    }

    Ok(())
}

pub async fn get_packages_statuses<'a, 'c>(
    client: &'c Client,
    options: &'a CommandOptions,
) -> Result<Vec<(&'a str, PackageStatus)>> {
    // TODO: can we run these all concurrently?
    let packages_and_futures = options
        .packages
        .iter()
        .map(|p| (p.as_ref(), get_package_status(&client, &p)))
        .collect::<Vec<_>>();

    let mut results = Vec::new();
    for (p, f) in packages_and_futures {
        results.push((p, f.await?));
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
        Ok(PackageStatus { voted: true, token })
    } else if document.select(&aur::VOTE_SELECTOR).next().is_some() {
        Ok(PackageStatus {
            voted: false,
            token,
        })
    } else {
        Err(anyhow!("{}: Unknown vote state!", pkg))
    }
}
