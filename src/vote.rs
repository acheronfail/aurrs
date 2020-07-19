use anyhow::{anyhow, Result};
use clap::crate_name;
use reqwest::multipart::Form;
use reqwest::Client;

use crate::aur;
use crate::cli::VoteCommandOptions;
use crate::status::get_packages_statuses;

pub async fn do_vote(client: &Client, options: VoteCommandOptions) -> Result<()> {
    let padding = options.longest_package_len();
    let packages_and_statuses = get_packages_statuses(client, &options).await?;

    let should_vote = !options.unvote;
    for s in packages_and_statuses {
        if options.info || s.voted == should_vote {
            println!(
                "{:width$}: {}",
                s.name,
                if s.voted {
                    "Already Voted!"
                } else {
                    "Not voted."
                },
                width = padding
            );
        } else {
            match s.token {
                Some(token) => {
                    match change_package_vote(client, &s.name, token, should_vote).await {
                        Ok(_) => println!(
                            "{:width$}: {}",
                            &s.name,
                            if should_vote {
                                "Voted!"
                            } else {
                                "Removed Vote!"
                            },
                            width = padding
                        ),
                        Err(e) => {
                            println!("{:width$}: Failed to update vote - {}", e, width = padding)
                        }
                    }
                }
                None => return Err(anyhow!("Failed to find token for: {}", &s.name)),
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
