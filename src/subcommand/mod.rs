use clap::ArgMatches;

pub mod ls;

pub trait Subcommand {
    fn run(self, matches: &ArgMatches) -> Result<(), String>;
}
