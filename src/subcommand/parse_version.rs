use anyhow::Result;
use clap::ArgMatches;
use semver::{Compat, VersionReq};

use crate::{config::Config, subcommand::Subcommand};

pub struct ParseVersion;

impl<'c> Subcommand<'c> for ParseVersion {
    fn run(_config: &'c Config, matches: &ArgMatches) -> Result<()> {
        let input = matches.value_of("version").unwrap();

        match VersionReq::parse_compat(input, Compat::Npm) {
            Ok(result) => {
                println!(
                    "{:^pad$}\n{:^pad$}\n{}",
                    input,
                    "⬇",
                    result.to_string(),
                    pad = result.to_string().len()
                );
                Ok(())
            },
            Err(err) => {
                println!("Failed to parse `{}`: `{}`", input, err.to_string());
                Ok(())
            },
        }
    }
}
