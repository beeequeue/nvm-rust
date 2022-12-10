use anyhow::Result;
use clap::Parser;
use node_semver::Range;

use crate::{
    node_version,
    node_version::{InstalledNodeVersion, NodeVersion},
    subcommand::Action,
    Config,
};

#[derive(Parser, Clone, Debug)]
#[command(
    about = "Uninstall a version",
    alias = "r",
    alias = "rm",
    alias = "remove"
)]
pub struct UninstallCommand {
    /// A semver range. The latest version matching this range will be installed
    #[clap(validator = node_version::is_version_range)]
    pub version: Range,
}

impl Action<UninstallCommand> for UninstallCommand {
    fn run(config: &Config, options: &UninstallCommand) -> Result<()> {
        let version = InstalledNodeVersion::find_matching(config, &options.version);
        if version.is_none() {
            anyhow::bail!("{} is not installed.", &options.version.to_string())
        }

        let version = version.unwrap();
        if version.is_selected(config) {
            println!("{} is currently selected.", version.version());

            if !config.force
                && !(dialoguer::Confirm::new()
                    .with_prompt("Are you sure you want to uninstall it?")
                    .interact()?)
            {
                return Ok(());
            }

            InstalledNodeVersion::deselect(config)?;
        }

        version.uninstall(config)
    }
}
