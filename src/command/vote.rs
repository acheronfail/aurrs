use anyhow::{anyhow, Result};
use reqwest::blocking::Client;

use crate::aur::{change_package_vote, get_package_status};
use crate::cli::VoteCommandOptions;
use crate::model::PackageStatus;

pub fn vote(client: &Client, options: &VoteCommandOptions) -> Result<()> {
    let padding = options.longest_package_len();
    let packages_and_statuses = get_packages_statuses(client, &options)?;

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
    }

    Ok(())
}

pub fn get_packages_statuses(
    client: &Client,
    options: &VoteCommandOptions,
) -> Result<Vec<PackageStatus>> {
    // TODO: running this in parallel would save time
    options
        .packages
        .iter()
        .map(|pkg_name| get_package_status(&client, pkg_name))
        .collect()
}
