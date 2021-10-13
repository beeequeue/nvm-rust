use std::fs::read_link;
#[cfg(windows)]
use std::fs::remove_dir;
#[cfg(unix)]
use std::fs::remove_file;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;

use anyhow::Result;
use clap::{AppSettings, Clap};
use node_semver::{Range, Version};

use crate::{
    node_version,
    node_version::{InstalledNodeVersion, NodeVersion},
    subcommand::Action,
    Config,
};

#[derive(Clap, Clone, Debug)]
#[clap(
about = "Switch to an installed node version",
alias = "switch",
alias = "use",
setting = AppSettings::ColoredHelp
)]
pub struct SwitchCommand {
    /// A semver range. The latest version matching this range will be switched to.
    #[clap(validator = node_version::is_version_range)]
    pub version: Range,
}

impl Action<SwitchCommand> for SwitchCommand {
    fn run(config: &Config, options: &SwitchCommand) -> Result<()> {
        let version = InstalledNodeVersion::find_matching(config, &options.version);

        if version.is_none() {
            anyhow::bail!("No version matching the version range was found.")
        }

        let version = version.unwrap();

        if !InstalledNodeVersion::is_installed(config, version.version()) {
            anyhow::bail!("{} is not installed", version.to_string());
        }

        let result = set_shims(config, version.version());
        if let Result::Ok(()) = result {
            println!("Switched to {}", version.to_string());
        }

        result
    }
}

#[cfg(windows)]
fn set_shims(config: &Config, version: &Version) -> Result<()> {
    let shims_dir = config.get_shims_dir();

    if !InstalledNodeVersion::is_installed(config, version) {
        anyhow::bail!("{} is not installed", version);
    }

    if read_link(&shims_dir).is_ok() {
        if let Result::Err(err) = remove_dir(shims_dir.to_owned()) {
            anyhow::bail!(
                "Could not remove old symlink at {:?}: {}",
                shims_dir,
                err.to_string()
            );
        }
    }

    symlink_dir(
        config.get_versions_dir().join(version.to_string()),
        shims_dir,
    )
    .map_err(anyhow::Error::from)
}

#[cfg(unix)]
fn set_shims(config: &Config, version: &Version) -> Result<()> {
    let shims_dir = config.get_shims_dir();

    if read_link(&shims_dir).is_ok() {
        remove_file(&shims_dir)?;
    }

    symlink(
        config
            .get_versions_dir()
            .join(version.to_string())
            .join("bin"),
        shims_dir,
    )
    .map_err(anyhow::Error::from)
}
