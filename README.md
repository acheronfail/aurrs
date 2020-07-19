# `aurrs`

AUR + RS

An experimental wrapper for `pacman` which adds the following features:

* Voting on the AUR repository
* ... more to come!

## Usage

```txt
aurrs

USAGE:
    aurrs [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    -A, --vote, vote            Vote for a package on the AUR
    -D, --database, database    Alias for calling `pacman -D <args>...`
    -F, --files, files          Alias for calling `pacman -F <args>...`
    -Q, --query, query          Alias for calling `pacman -Q <args>...`
    -R, --remove, remove        Alias for calling `pacman -R <args>...`
    -S, --sync, sync            Alias for calling `pacman -S <args>...`
    -T, --deptest, deptest      Alias for calling `pacman -T <args>...`
    -U, --upgrade, upgrade      Alias for calling `pacman -U <args>...`
    help                        Prints this message or the help of the given subcommand(s)
```

## Installation

#### Precompiled binaries

See the [releases] page for pre-compiled binaries.

#### Via Cargo

```bash
cargo install aurrs
```

#### From Source (via Cargo)

```bash
git clone https://github.com/acheronfail/aurrs/
cd aurrs
cargo install --path .
```

[releases]: https://github.com/acheronfail/aurrs/releases
