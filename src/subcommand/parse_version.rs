use anyhow::Result;
use clap::Parser;
use node_semver::Range;

use crate::{files, node_version::parse_range, subcommand::Action, Config};

#[derive(Parser, Clone, Debug)]
#[command(
    about = "Echo what a version string will be parsed to",
    alias = "pv",
    hide(true),
)]
pub struct ParseVersionCommand {
    /// The semver range to echo the parsed result of
    #[arg(value_parser = parse_range)]
    pub version: Option<String>,
}

impl Action<ParseVersionCommand> for ParseVersionCommand {
    fn run(_: &Config, options: &ParseVersionCommand) -> Result<()> {
        let version = options.version.clone();

        if version.is_none() {
            if let Some(version_from_files) = files::get_version_file() {
                println!("{}", version_from_files.range());

                return Ok(());
            }
        }

        if version.is_none() {
            anyhow::bail!("Did not get a version");
        }
        let version = version.unwrap();

        match Range::parse(&version) {
            Ok(result) => {
                println!(
                    "{:^pad$}\n{:^pad$}\n{}",
                    version,
                    "⬇",
                    result,
                    pad = result.to_string().len()
                );
                Ok(())
            },
            Err(err) => {
                println!("Failed to parse `{}`", err.input());
                Ok(())
            },
        }
    }
}
