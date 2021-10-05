use anyhow::Result;
use clap::{AppSettings, Clap};
use node_semver::Range;

use crate::{
    actions::Action,
    node_version,
    node_version::{InstalledNodeVersion, NodeVersion},
    Config,
};

#[derive(Clap, Clone, Debug)]
#[clap(
about = "Uninstall a version",
alias = "r",
alias = "remove",
setting = AppSettings::ColoredHelp
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
                return Result::Ok(());
            }

            InstalledNodeVersion::deselect(config)?;
        }

        version.uninstall(config)
    }
}
