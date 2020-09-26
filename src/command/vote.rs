use anyhow::{anyhow, Result};
use reqwest::blocking::Client;

use crate::aur::{change_package_vote, get_vote_package_status};
use crate::cli::AurCommandOptions;
use crate::model::AurPackage;

pub fn vote(client: &Client, options: &AurCommandOptions) -> Result<()> {
    let padding = options.longest_package_len();
    let packages_and_statuses = get_vote_packages_statuses(client, &options)?;

    let should_vote = !options.remove;
    for s in packages_and_statuses {
        if s.voted == should_vote {
            println!(
                "{:width$}: {}",
                &s.name,
                if should_vote {
                    "Already voted!"
                } else {
                    "Not voted!"
                },
                width = padding
            );
            continue;
        }

        match s.token {
            Some(token) => match change_package_vote(client, &s.name, token, should_vote) {
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
                Err(e) => println!("{:width$}: Failed to update vote - {}", e, width = padding),
            },
            None => return Err(anyhow!("Failed to find token for: {}", &s.name)),
        }
    }

    Ok(())
}

pub fn get_vote_packages_statuses(
    client: &Client,
    options: &AurCommandOptions,
) -> Result<Vec<AurPackage>> {
    // TODO: running this in parallel would save time
    options
        .packages
        .iter()
        .map(|pkg_name| get_vote_package_status(&client, pkg_name, options.info))
        .collect()
}
