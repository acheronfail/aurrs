use clap::AppSettings::ColoredHelp;
use clap::Clap;
#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct Args {
    /// The subcommand to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clap)]
pub enum Command {
    /// Vote (if not voted)
    Vote(CommandOptions),
    /// Remove votes (if voted)
    Unvote(CommandOptions),
    /// Print the current vote status
    Status(CommandOptions),
}

#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct CommandOptions {
    /// The list of packages
    #[clap(required = true)]
    pub packages: Vec<String>,
}

impl CommandOptions {
    pub fn longest_package_len(&self) -> usize {
        self.packages
            .iter()
            .fold(0, |acc, p| std::cmp::max(acc, p.len()))
    }
}
