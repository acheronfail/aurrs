mod aur;
mod cli;
mod command;
mod credentials;
mod model;
mod sudo;

use std::env::args_os;

use anyhow::Result;
use clap::{crate_name, Clap};
use reqwest::blocking::Client;

use cli::{Args, SubCommand};

fn main() -> Result<()> {
    let args = Args::parse();

    unsafe {
        if libc::geteuid() == 0 {
            eprintln!("Please avoid running {} as root!", crate_name!());
        }
    }

    match args.subcommand {
        Some(subcommand) => {
            if subcommand.needs_sudo() {
                sudo::use_sudo()?;
            }

            let cli_arguments = args_os().into_iter().skip(1);

            match &subcommand {
                // AUR.
                SubCommand::Aur(options) => {
                    let should_install = options.package || (!options.vote && !options.info);
                    if should_install {
                        command::install(options)?;
                    } else {
                        // TODO: load the cookie from disk, currently not possible with reqwest
                        // See: https://github.com/seanmonstar/reqwest/issues/14#issuecomment-481414056
                        let client = Client::builder().cookie_store(true).build()?;
                        aur::login_client_to_aur(&client)?;

                        if options.vote {
                            command::vote(&client, options)?
                        } else if options.info {
                            let info_string =
                                command::get_vote_packages_statuses(&client, options)?
                                    .iter()
                                    .map(|p| format!("{}", p))
                                    .collect::<Vec<_>>()
                                    .join("\n---\n");

                            println!("{}", info_string);
                        }
                    }
                }
                // These commands are proxied directly to pacman
                SubCommand::PacmanS(_)
                | SubCommand::PacmanR(_)
                | SubCommand::PacmanD(_)
                | SubCommand::PacmanQ(_)
                | SubCommand::PacmanF(_)
                | SubCommand::PacmanT(_)
                | SubCommand::PacmanU(_) => command::pacman(cli_arguments)?,
            }
        }
        // Default to `-Syu`
        None => {
            sudo::use_sudo()?;
            command::pacman(&["-Syu"])?;
        }
    }

    Ok(())
}
