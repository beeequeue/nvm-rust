use anyhow::Result;
use clap::ArgMatches;

use crate::config::OldConfig;

pub mod install;
pub mod list;
pub mod parse_version;
pub mod switch;
pub mod uninstall;

pub trait Subcommand<'c> {
    fn run(config: &'c OldConfig, matches: &ArgMatches) -> Result<()>;
}
