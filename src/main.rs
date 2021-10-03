use std::path::PathBuf;

use anyhow::Result;
use clap::{AppSettings, Clap};

use subcommand::{install::Install, list::{List,ListCommand}, switch::Switch, uninstall::Uninstall};

mod old_config;
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
#[clap(
    name = "nvm(-rust)",
    about = "Node Version Manager (but better, and in Rust)",
    setting = AppSettings::ColoredHelp
)]
pub struct Config {
    /// Installation directory
    #[clap(short, long, env("NVM_DIR"))]
    dir: PathBuf,
    /// bin directory
    #[clap(short, long, env("NVM_SHIMS_DIR"))]
    shims_dir: PathBuf,
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


    }

    match matches.command {
        Subcommands::List(options) => {
            println!("{:?}", options);
        }
    }

    Result::Ok(())
}
