use std::borrow::Borrow;
#[cfg(windows)]
use std::fs::remove_dir;
#[cfg(unix)]
use std::fs::remove_file;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;

use anyhow::Result;
use clap::ArgMatches;
use node_semver::{Range, Version};

use crate::{
    node_version::{InstalledNodeVersion, NodeVersion},
    old_config::OldConfig,
    subcommand::Subcommand,
};

pub struct Switch<'c> {
    config: &'c OldConfig,
}

impl<'c> Switch<'c> {
    #[cfg(windows)]
    fn set_shims(self, version: &Version) -> Result<()> {
        let shims_dir = self.config.shims_dir.to_owned();

        if !InstalledNodeVersion::is_installed_old(self.config, version) {
            anyhow::bail!("{} is not installed", version);
        }

        if shims_dir.exists() {
            if let Result::Err(err) = remove_dir(shims_dir.to_owned()) {
                anyhow::bail!(
                    "Could not remove old symlink at {:?}: {}",
                    shims_dir,
                    err.to_string()
                );
            }
        }

        symlink_dir(self.config.dir.join(version.to_string()), shims_dir)
            .map_err(anyhow::Error::from)
    }

    #[cfg(unix)]
    fn set_shims(self, version: &Version) -> Result<()> {
        let shims_dir = self.config.shims_dir.to_owned();

        if shims_dir.exists() {
            if let Result::Err(err) = remove_file(shims_dir.to_owned()) {
                anyhow::bail!(
                    "Could not remove old symlink at {:?}: {}",
                    shims_dir,
                    err.to_string()
                );
            }
        }

        symlink(
            self.config.dir.join(version.to_string()).join("bin"),
            shims_dir,
        )
        .map_err(anyhow::Error::from)
    }
}

impl<'c> Subcommand<'c> for Switch<'c> {
    fn run(config: &'c OldConfig, matches: &ArgMatches) -> Result<()> {
        let command = Self { config };

        let range: Option<Range>;

        if let Some(arg) = matches.value_of("version") {
            // The argument is checked by clap in main.rs
            range = Range::parse(arg).ok();
        } else {
            // TODO: Check for .nvmrc, parse it, etc...
            anyhow::bail!(".nvmrc files are not supported yet.");
        }

        if range.is_none() {
            anyhow::bail!("Did not get a version to switch to.");
        }

        if let Some(inv) = InstalledNodeVersion::get_matching(config, range.unwrap().borrow()) {
            if !InstalledNodeVersion::is_installed_old(config, &inv.version()) {
                anyhow::bail!("{} is not installed", inv.version());
            }

            let result = command.set_shims(&inv.version());
            if let Result::Ok(()) = result {
                println!("Switched to {}", inv.version());
            }

            return result;
        }

        Result::Err(anyhow::anyhow!(
            "No version matching the version range was found."
        ))
    }
}
