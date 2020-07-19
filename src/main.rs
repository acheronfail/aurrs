mod aur;
mod cli;
mod login;
mod pacman;
mod status;
mod sudo;
mod vote;

use anyhow::Result;
use clap::Clap;
use reqwest::Client;

use cli::{Args, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    unsafe {
        if libc::geteuid() == 0 {
            eprintln!("Please avoid running aurrs as root!");
        }
    }

    match args.command {
        // AUR voting.
        Some(SubCommand::Vote(options)) => {
            // TODO: load the cookie from disk, currently not possible with reqwest
            // See: https://github.com/seanmonstar/reqwest/issues/14#issuecomment-481414056
            let client = Client::builder().cookie_store(true).build()?;
            login::do_login(&client).await?;
            vote::do_vote(&client, options).await?
        }
        // Commands are proxied directly to pacman.
        Some(SubCommand::PacmanD { args })
        | Some(SubCommand::PacmanF { args })
        | Some(SubCommand::PacmanQ { args })
        | Some(SubCommand::PacmanR { args })
        | Some(SubCommand::PacmanS { args })
        | Some(SubCommand::PacmanT { args })
        | Some(SubCommand::PacmanU { args }) => {
            sudo::use_sudo()?;
            pacman::do_pacman(args)?;
        }
        // Default to `-Syu`
        None => {
            sudo::use_sudo()?;
            pacman::do_pacman(&["-Syu"])?;
        }
    }

    Ok(())
}
