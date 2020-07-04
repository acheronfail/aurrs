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
    Vote(VoteCommandOptions),
    /// Remove votes (if voted)
    Unvote(VoteCommandOptions),
    /// Print the current vote status
    Status(VoteCommandOptions),
}

#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct VoteCommandOptions {
    /// The list of packages
    #[clap(required = true)]
    pub packages: Vec<String>,
}

impl VoteCommandOptions {
    pub fn longest_package_len(&self) -> usize {
        self.packages
            .iter()
            .fold(0, |acc, p| std::cmp::max(acc, p.len()))
    }
}
