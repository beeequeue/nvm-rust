use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{AppSettings, Clap, ValueHint};

use crate::{
    actions::Action,
    subcommand::{list::ListCommand, parse_version::ParseVersionCommand},
};

mod actions;
mod node_version;
mod old_config;
mod subcommand;
mod utils;

// fn validate_number(value: &str) -> Result<i32> {
//     value.parse().context(format!("{} is not a number!", value))
// }

#[derive(Clap, Debug)]
enum Subcommands {
    List(ListCommand),
    ParseVersion(ParseVersionCommand),
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
        self.dir
            .as_ref()
            .map_or_else(Config::default_dir, |r| r.clone())
    }

    pub fn get_shims_dir(&self) -> PathBuf {
        self.shims_dir
            .as_ref()
            .map_or_else(|| self.get_dir().join("shims"), |r| r.clone())
    }

    /// Path to directory containing node versions
    fn get_versions_dir(&self) -> PathBuf {
        self.get_dir().join("versions")
    }

    #[cfg(windows)]
    fn default_dir() -> PathBuf {
        dirs::data_local_dir().unwrap().join("nvm-rust")
    }

    #[cfg(unix)]
    fn default_dir() -> String {
        dirs::home_dir().unwrap().join("nvm-rust")
    }
}

fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        create_dir_all(path.to_path_buf())
            .unwrap_or_else(|err| panic!("Could not create {:?} - {}", path, err));

        println!("Created nvm dir at {:?}", path);
    }

    if !path.is_dir() {
        panic!("{:?} is not a directory! Please rename it.", path)
    }
}

fn main() -> Result<()> {
    let config: Config = Config::parse();

    ensure_dir_exists(&config.get_dir());
    ensure_dir_exists(&config.get_shims_dir());
    ensure_dir_exists(&config.get_versions_dir());

    match config.command {
        Subcommands::List(ref options) => ListCommand::run(&config, options),
        Subcommands::ParseVersion(ref options) => ParseVersionCommand::run(&config, options),
        #[allow(unreachable_patterns)]
        _ => Result::Ok(()),
    }
}
