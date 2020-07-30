use clap::ArgMatches;

pub mod install;
pub mod list;

pub trait Subcommand {
    fn run(matches: &ArgMatches) -> Result<(), String>;
}
