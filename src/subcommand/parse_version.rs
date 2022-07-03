use anyhow::Result;
use clap::{AppSettings, Parser};
use node_semver::Range;

use crate::{node_version::is_version_range, subcommand::Action, Config};

#[derive(Parser, Clone, Debug)]
#[clap(
about = "Echo what a version string will be parsed to",
alias = "pv",
setting = AppSettings::ColoredHelp,
setting = AppSettings::Hidden
)]
pub struct ParseVersionCommand {
    /// The semver range to echo the parsed result of
    #[clap(validator = is_version_range)]
    pub version: String,
}

impl Action<ParseVersionCommand> for ParseVersionCommand {
    fn run(_: &Config, options: &ParseVersionCommand) -> Result<()> {
        match Range::parse(&options.version) {
            Ok(result) => {
                println!(
                    "{:^pad$}\n{:^pad$}\n{}",
                    options.version,
                    "⬇",
                    result,
                    pad = result.to_string().len()
                );
                Ok(())
            },
            Err(err) => {
                println!(
                    "Failed to parse `{}`: `{}`",
                    options.version,
                    err
                );
                Ok(())
            },
        }
    }
}
