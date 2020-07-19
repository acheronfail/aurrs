mod aur;
mod cli;
mod login;
mod pacman;
mod status;
mod sudo;
mod vote;

use anyhow::Result;
use clap::{crate_name, Clap};
use reqwest::Client;

use cli::{Args, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    unsafe {
        if libc::geteuid() == 0 {
            eprintln!("Please avoid running {} as root!", crate_name!());
        }
    }

    match args.command {
        Some(subcommand) => match &subcommand {
            // AUR voting.
            SubCommand::Vote(options) => {
                // TODO: load the cookie from disk, currently not possible with reqwest
                // See: https://github.com/seanmonstar/reqwest/issues/14#issuecomment-481414056
                let client = Client::builder().cookie_store(true).build()?;
                login::do_login(&client).await?;
                vote::do_vote(&client, options).await?
            }
            // Commands are proxied directly to pacman.
            SubCommand::PacmanD { args }
            | SubCommand::PacmanF { args }
            | SubCommand::PacmanQ { args }
            | SubCommand::PacmanR { args }
            | SubCommand::PacmanS { args }
            | SubCommand::PacmanT { args }
            | SubCommand::PacmanU { args } => {
                sudo::use_sudo()?;
                pacman::do_pacman(
                    vec![subcommand.pacman_operation().unwrap()]
                        .into_iter()
                        .chain(args.into_iter().map(|s| s.as_str())),
                )?;
            }
        },
        // Default to `-Syu`
        None => {
            sudo::use_sudo()?;
            pacman::do_pacman(&["-Syu"])?;
        }
    }

    Ok(())
}
