use anyhow::Result;
use clap::Clap;
use reqwest::Client;

use cli::{Args, Command};
use login::do_login;
use vote::do_vote;

mod aur;
mod cli;
mod login;
mod status;
mod vote;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // TODO: load the cookie from disk, currently not possible with reqwest
    // See: https://github.com/seanmonstar/reqwest/issues/14#issuecomment-481414056
    let client = Client::builder().cookie_store(true).build()?;
    do_login(&client).await?;

    match args.command {
        Command::Vote(options) => do_vote(&client, options).await?,
    }

    Ok(())
}
