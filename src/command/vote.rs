use anyhow::{anyhow, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;

use crate::aur::{change_package_vote, get_package_status};
use crate::cli::VoteCommandOptions;
use crate::model::PackageStatus;

pub async fn vote(client: &Client, options: &VoteCommandOptions) -> Result<()> {
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
