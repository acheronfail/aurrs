mod aur;
mod cli;
mod command;
mod credentials;
mod model;
mod sudo;

use std::env::args_os;

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
                aur::login_client_to_aur(&client).await?;
                command::vote(&client, options).await?
            }
            // Commands are proxied directly to pacman.
            SubCommand::PacmanD(_)
            | SubCommand::PacmanF(_)
            | SubCommand::PacmanQ(_)
            | SubCommand::PacmanR(_)
            | SubCommand::PacmanS(_)
            | SubCommand::PacmanT(_)
            | SubCommand::PacmanU(_) => {
                sudo::use_sudo()?;
                command::pacman(args_os().into_iter().skip(1))?;
            }
        },
        // Default to `-Syu`
        None => {
            sudo::use_sudo()?;
            command::pacman(&["-Syu"])?;
        }
    }

    Ok(())
}
