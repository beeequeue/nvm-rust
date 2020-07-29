use clap::ArgMatches;

pub mod ls;

pub trait Subcommand {
    fn run(matches: &ArgMatches) -> Result<(), String>;
}
