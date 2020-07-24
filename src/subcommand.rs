use crate::config::Config;
use clap::ArgMatches;

pub mod ls;

pub trait Subcommand {
    fn run(matches: ArgMatches, config: Config) -> ();
}
