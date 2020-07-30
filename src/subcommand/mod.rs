use clap::ArgMatches;

pub mod list;

pub trait Subcommand {
    fn run(matches: &ArgMatches) -> Result<(), String>;
}
