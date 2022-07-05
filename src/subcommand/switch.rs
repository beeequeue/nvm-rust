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
use clap::{AppSettings, Parser};
use node_semver::{Range, Version};

use crate::{
    files, node_version,
    node_version::{InstalledNodeVersion, NodeVersion},
    subcommand::Action,
    Config,
};

#[derive(Parser, Clone, Debug)]
#[clap(
about = "Switch to an installed node version",
alias = "switch",
alias = "use",
setting = AppSettings::ColoredHelp
)]
pub struct SwitchCommand {
    /// A semver range. The latest version matching this range will be switched to.
    #[clap(validator = node_version::is_version_range)]
    pub version: Option<Range>,
}

impl Action<SwitchCommand> for SwitchCommand {
    fn run(config: &Config, options: &SwitchCommand) -> Result<()> {
        let version_filter = options
            .clone()
            .version
            .xor(files::get_version_file().map(|version_file| version_file.range()));

        if version_filter.is_none() {
            anyhow::bail!("You did not pass a version and we did not find any version files (package.json#engines, .nvmrc) in the current directory.");
        }
        let version_filter = version_filter.unwrap();

        let version = InstalledNodeVersion::find_matching(config, &version_filter);
        if version.is_none() {
            anyhow::bail!("No version matching the version range was found.")
        }

        let version = version.unwrap();

        if !InstalledNodeVersion::is_installed(config, version.version()) {
            anyhow::bail!("{} is not installed", version.to_string());
        }

        let result = set_shims(config, version.version());
        if let Ok(()) = result {
            println!("Switched to {}", version.to_string());
        }

        result
    }
}

#[cfg(windows)]
fn set_shims(config: &Config, version: &Version) -> Result<()> {
    let shims_dir = config.get_shims_dir();

    if !InstalledNodeVersion::is_installed(config, version) {
        anyhow::bail!("{version} is not installed");
    }

    if read_link(&shims_dir).is_ok() {
        if let Err(err) = remove_dir(&shims_dir) {
            anyhow::bail!("Could not remove old symlink at {shims_dir:?}: {err}",);
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
