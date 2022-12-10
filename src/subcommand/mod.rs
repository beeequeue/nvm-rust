use anyhow::Result;

use crate::Config;

pub mod install;
pub mod is_installed;
pub mod list;
pub mod parse_version;
pub mod switch;
pub mod uninstall;

pub trait Action<T: clap::Parser> {
    fn run(config: &Config, options: &T) -> Result<()>;
}
