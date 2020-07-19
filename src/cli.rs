use clap::AppSettings::{AllowLeadingHyphen, ColoredHelp, TrailingVarArg};
use clap::Clap;
#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct Args {
    /// The subcommand to run
    #[clap(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Debug, Clap)]
pub enum SubCommand {
    /// Alias for calling `pacman -D <args>...`.
    #[clap(name = "database", short_flag = 'D', long_flag = "database", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanD { args: Vec<String> },
    /// Alias for calling `pacman -F <args>...`.
    #[clap(name = "files", short_flag = 'F', long_flag = "files", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanF { args: Vec<String> },
    /// Alias for calling `pacman -Q <args>...`.
    #[clap(name = "query", short_flag = 'Q', long_flag = "query", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanQ { args: Vec<String> },
    /// Alias for calling `pacman -R <args>...`.
    #[clap(name = "remove", short_flag = 'R', long_flag = "remove", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanR { args: Vec<String> },
    /// Alias for calling `pacman -S <args>...`.
    #[clap(name = "sync", short_flag = 'S', long_flag = "sync", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanS { args: Vec<String> },
    /// Alias for calling `pacman -T <args>...`.
    #[clap(name = "deptest", short_flag = 'T', long_flag = "deptest", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanT { args: Vec<String> },
    /// Alias for calling `pacman -U <args>...`.
    #[clap(name = "upgrade", short_flag = 'U', long_flag = "upgrade", setting = ColoredHelp, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
    PacmanU { args: Vec<String> },

    /// Vote for a package on the AUR
    #[clap(short_flag = 'A', long_flag = "vote", setting = ColoredHelp)]
    Vote(VoteCommandOptions),
}

impl SubCommand {
    pub fn pacman_operation(&self) -> Option<&str> {
        match self {
            Self::PacmanD { .. } => Some("-D"),
            Self::PacmanF { .. } => Some("-F"),
            Self::PacmanQ { .. } => Some("-Q"),
            Self::PacmanR { .. } => Some("-R"),
            Self::PacmanS { .. } => Some("-S"),
            Self::PacmanT { .. } => Some("-T"),
            Self::PacmanU { .. } => Some("-U"),
            _ => None,
        }
    }
}

#[derive(Debug, Clap)]
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
