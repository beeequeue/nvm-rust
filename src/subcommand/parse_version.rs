use anyhow::Result;
use clap::ArgMatches;
use semver::{Compat, VersionReq};

use crate::{old_config::OldConfig, subcommand::Subcommand};

pub struct ParseVersion;

impl<'c> Subcommand<'c> for ParseVersion {
    fn run(_config: &'c OldConfig, matches: &ArgMatches) -> Result<()> {
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
