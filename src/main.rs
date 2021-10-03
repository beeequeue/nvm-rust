use anyhow::Result;
use clap::{AppSettings, Clap};

use config::OldConfig;
use subcommand::{install::Install, list::{List,ListCommand}, switch::Switch, uninstall::Uninstall};

mod config;
mod node_version;
mod subcommand;
mod utils;

// fn validate_number(value: &str) -> Result<i32> {
//     value.parse().context(format!("{} is not a number!", value))
// }

#[derive(Clap)]
enum Subcommands {
    List(ListCommand),
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Config {
    /// Level of verbosity, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// Accept any prompts needed for command to complete
    #[clap(short, long)]
    force: bool,

    #[clap(subcommand)]
    command: Subcommands,
}

fn main() -> Result<()> {
    let matches: Config = Config::parse();

    let config = OldConfig::from_env_and_args();

    if matches.verbose > 0 {
        println!("{:#?}\n", config);
    }

    match matches.command {
        Subcommands::List(options) => {
            println!("{:?}", options);
        }
    }

    Result::Ok(())
}
