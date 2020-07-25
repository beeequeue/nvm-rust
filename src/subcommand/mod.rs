use clap::ArgMatches;

use crate::config::Config;

pub mod ls;

pub trait Subcommand {
    fn run(self, matches: &ArgMatches, config: Config) -> Result<(), String>;
}
