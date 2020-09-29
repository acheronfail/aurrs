use clap::AppSettings::{
    AllowExternalSubcommands, AllowLeadingHyphen, ColoredHelp, TrailingVarArg,
};
use clap::Clap;
#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp)]
pub struct Args {
    /// The subcommand to run
    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

/// A simple struct that swallows all arguments passed after it.
#[derive(Debug, Clap)]
#[clap(setting = ColoredHelp, setting = AllowExternalSubcommands, setting = TrailingVarArg, setting = AllowLeadingHyphen)]
pub struct ClapArgsSink;

#[derive(Debug, Clap)]
pub enum SubCommand {
    /// Alias for calling `pacman -D <args>...`.
    #[clap(name = "database", short_flag = 'D', long_flag = "database")]
    PacmanD(ClapArgsSink),
    /// Alias for calling `pacman -F <args>...`.
    #[clap(name = "files", short_flag = 'F', long_flag = "files")]
    PacmanF(ClapArgsSink),
    /// Alias for calling `pacman -Q <args>...`.
    #[clap(name = "query", short_flag = 'Q', long_flag = "query")]
    PacmanQ(ClapArgsSink),
    /// Alias for calling `pacman -R <args>...`.
    #[clap(name = "remove", short_flag = 'R', long_flag = "remove")]
    PacmanR(ClapArgsSink),
    /// Alias for calling `pacman -S <args>...`.
    #[clap(name = "sync", short_flag = 'S', long_flag = "sync")]
    PacmanS(ClapArgsSink),
    /// Alias for calling `pacman -T <args>...`.
    #[clap(name = "deptest", short_flag = 'T', long_flag = "deptest")]
    PacmanT(ClapArgsSink),
    /// Alias for calling `pacman -U <args>...`.
    #[clap(name = "upgrade", short_flag = 'U', long_flag = "upgrade")]
    PacmanU(ClapArgsSink),

    /// AUR commands (install AUR packages, vote for packages, etc)
    #[clap(short_flag = 'A', long_flag = "aur", setting = ColoredHelp)]
    Aur(AurCommandOptions),
}

impl SubCommand {
    pub fn needs_sudo(&self) -> bool {
        match self {
            SubCommand::PacmanD(_)
            | SubCommand::PacmanF(_)
            | SubCommand::PacmanQ(_)
            | SubCommand::PacmanR(_)
            | SubCommand::PacmanS(_)
            | SubCommand::PacmanT(_)
            | SubCommand::PacmanU(_) => true,
            SubCommand::Aur(_) => false,
        }
    }
}

#[derive(Debug, Clap)]
pub struct AurCommandOptions {
    // REQUIRED
    // --------
    /// The list of packages
    #[clap(required = true)]
    pub packages: Vec<String>,

    // OPERATIONS
    // --------
    /// View package information, including voted status
    #[clap(short = 'i', long = "info", conflicts_with_all = &["package", "vote", "remove"])]
    pub info: bool,

    /// Install/remove a package
    #[clap(short = 'p', long = "package")]
    pub package: bool,

    /// Add/remove a vote
    #[clap(short = 'v', long = "vote")]
    pub vote: bool,

    // BEHAVIOUR OPTIONS
    // --------
    /// Remove for the current operation (add is the default)
    #[clap(short = 'r', long = "remove")]
    pub remove: bool,
}

impl AurCommandOptions {
    pub fn longest_package_len(&self) -> usize {
        self.packages
            .iter()
            .fold(0, |acc, p| std::cmp::max(acc, p.len()))
    }
}
