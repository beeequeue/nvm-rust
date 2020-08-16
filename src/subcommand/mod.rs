use clap::ArgMatches;

use crate::config::Config;

pub mod install;
pub mod list;

pub trait Subcommand {
    fn run(config: Config, matches: &ArgMatches) -> Result<(), String>;
}
