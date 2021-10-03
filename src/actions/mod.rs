use anyhow::Result;

use crate::Config;

pub trait Action<T: clap::Clap> {
    fn run(config: &Config, options: &T) -> Result<()>;
}
