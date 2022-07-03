use anyhow::Result;
use clap::{AppSettings, Parser};
use node_semver::Range;
use std::fmt;

use crate::{files::package_json, node_version::is_version_range, subcommand::Action, Config};

#[derive(Debug)]
enum Source {
    Input,
    PackageJson,
    Nvmrc,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
    pub version: Option<String>,
}

impl Action<ParseVersionCommand> for ParseVersionCommand {
    fn run(_: &Config, options: &ParseVersionCommand) -> Result<()> {
        let mut source = Source::Input;
        let mut version = options.version.clone();

        if version.is_none() {
            source = Source::PackageJson;
            let data = package_json::from_current_dir();

            match data {
                Some(data) if data.engines.is_some() => {
                    version = Some(data.engines.unwrap().node.unwrap().to_string());
                },
                _ => (),
            }
        }

        if version.is_none() {
            anyhow::bail!("Did not get a version");
        }
        let version = version.unwrap();

        match Range::parse(&version) {
            Ok(result) => {
                println!(
                    "from: {source}\n{:^pad$}\n{:^pad$}\n{}",
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
