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
    /// Vote for a package on the AUR
    #[clap(short_flag = 'A', long_flag = "vote")]
    Vote(VoteCommandOptions),
}

#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct VoteCommandOptions {
    /// The list of packages
    #[clap(required = true)]
    pub packages: Vec<String>,

    /// View vote status
    #[clap(short = 'i', long = "info", conflicts_with = "unvote")]
    pub info: bool,

    /// Remove your vote
    #[clap(short = 'r', long = "remove")]
    pub unvote: bool,
}

impl VoteCommandOptions {
    pub fn longest_package_len(&self) -> usize {
        self.packages
            .iter()
            .fold(0, |acc, p| std::cmp::max(acc, p.len()))
    }
}
