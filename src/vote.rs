use anyhow::{anyhow, Result};
use clap::crate_name;
use reqwest::multipart::Form;
use reqwest::Client;

use crate::aur;
use crate::cli::CommandOptions;
use crate::status::{get_packages_statuses, PackageStatus};

pub async fn do_vote(client: &Client, options: CommandOptions, should_vote: bool) -> Result<()> {
    let padding = options.longest_package_len();
    let packages_and_statuses = get_packages_statuses(client, &options).await?;
    for (pkg, PackageStatus { voted, token }) in packages_and_statuses {
        if voted == should_vote {
            println!(
                "{:width$}: {}",
                pkg,
                if should_vote {
                    "Already Voted!"
                } else {
                    "Not voted."
                },
                width = padding
            );
        } else {
            match token {
                Some(token) => match change_package_vote(client, pkg, token, should_vote).await {
                    Ok(_) => println!(
                        "{:width$}: {}",
                        pkg,
                        if should_vote {
                            "Voted!"
                        } else {
                            "Removed Vote!"
                        },
                        width = padding
                    ),
                    Err(e) => println!("{:width$}: Failed to update vote - {}", e, width = padding),
                },
                None => return Err(anyhow!("Failed to find token for: {}", pkg)),
            }
        }
    }

    Ok(())
}

pub async fn change_package_vote(
    client: &Client,
    pkg: &str,
    token: String,
    vote: bool,
) -> Result<()> {
    let resp = client
        .get(&format!("{}/rpc.php", aur::AUR_BASE_URL))
        .query(&[("type", "info"), ("arg", pkg)])
        .send()
        .await?
        .json::<aur::AurRpcInfo>()
        .await?;

    let aur::AurRpcInfoResult { package_base, .. } = resp.results;

    let (action, url_path) = if vote {
        ("do_Vote", "vote")
    } else {
        ("do_UnVote", "unvote")
    };

    let form = Form::new().text("token", token).text(action, crate_name!());
    client
        .post(&format!(
            "{}/pkgbase/{}/{}/",
            aur::AUR_BASE_URL,
            urlencoding::encode(&package_base),
            urlencoding::encode(url_path)
        ))
        .multipart(form)
        .send()
        .await?;

    Ok(())
}
