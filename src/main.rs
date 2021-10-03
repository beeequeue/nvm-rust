use std::path::PathBuf;

use anyhow::Result;
use clap::{AppSettings, Clap, ValueHint};

use subcommand::list::ListCommand;

use crate::actions::Action;

mod actions;
mod old_config;
mod node_version;
mod subcommand;
mod utils;

// fn validate_number(value: &str) -> Result<i32> {
//     value.parse().context(format!("{} is not a number!", value))
// }

#[derive(Clap, Debug)]
enum Subcommands {
    List(ListCommand),
}

#[derive(Clap, Debug)]
#[clap(
name = "nvm(-rust)",
about = "Node Version Manager (but better, and in Rust)",
setting = AppSettings::ColoredHelp
)]
pub struct Config {
    /// Installation directory
    #[clap(short, long, value_hint(ValueHint::DirPath), env("NVM_DIR"))]
    dir: Option<PathBuf>,
    /// bin directory
    #[clap(short, long, value_hint(ValueHint::DirPath), env("NVM_SHIMS_DIR"))]
    shims_dir: Option<PathBuf>,
    /// Level of verbosity, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    /// Accept any prompts needed for command to complete
    #[clap(short, long)]
    pub force: bool,

    #[clap(subcommand)]
    command: Subcommands,
}

impl Config {
    pub fn get_dir(&self) -> PathBuf {
        self.dir.as_ref().map_or_else(Config::default_dir, |r| r.clone())
    }

    pub fn get_shims_dir(&self) -> PathBuf {
        self.shims_dir.as_ref().map_or_else(|| self.get_dir().join("shims"), |r| r.clone())
    }

    /// Path to directory containing node versions
    fn get_versions_dir(&self) -> PathBuf {
        // self.get_dir().join("versions")
        self.get_dir()
    }

    #[cfg(windows)]
    fn default_dir() -> PathBuf {
        if cfg!(target_arch = "x86") {
            return "C:\\Program Files (x86)\\nvm".into();
        }

        "C:\\Program Files\\nvm".into()
    }

    #[cfg(unix)]
    fn default_dir() -> String {
        format!("{}/.nvm", env::var("HOME").unwrap())
    }
}

fn main() -> Result<()> {
    let mut matches: Config = Config {
        ..Config::parse()
    };
    println!("{:?}", matches);

    match matches.command {
        Subcommands::List(ref options) => ListCommand::run(&matches, options),
    }
}
