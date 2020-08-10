use std::borrow::Borrow;
#[cfg(windows)]
use std::fs::remove_dir;
#[cfg(unix)]
use std::fs::remove_file;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;

use clap::ArgMatches;
use semver::{Version, VersionReq};

use crate::{config::Config, node_version::InstalledNodeVersion, subcommand::Subcommand};

pub struct Switch<'c> {
    config: &'c Config,
}

impl<'c> Switch<'c> {
    #[cfg(windows)]
    fn set_shims(self, version: &Version) -> Result<(), String> {
        let shims_dir = self.config.shims_dir();

        if !InstalledNodeVersion::is_installed(self.config, version) {
            return Result::Err(format!("{} is not installed", version));
        }

        if shims_dir.exists() {
            if let Result::Err(err) = remove_dir(shims_dir.to_owned()) {
                return Result::Err(format!(
                    "Could not remove old symlink at {:?}: {}",
                    shims_dir,
                    err.to_string()
                ));
            }
        }

        symlink_dir(self.config.dir.join(version.to_string()), shims_dir)
            .map_err(|err| err.to_string())
    }

    #[cfg(unix)]
    fn set_shims(version: &Version) -> Result<(), String> {
        let shims_dir = CONFIG.shims_dir();

        if shims_dir.exists() {
            if let Result::Err(err) = remove_file(shims_dir.to_owned()) {
                return Result::Err(format!(
                    "Could not remove old symlink at {:?}: {}",
                    shims_dir,
                    err.to_string()
                ));
            }
        }

        symlink(CONFIG.dir().join(version.to_string()), shims_dir).map_err(|err| err.to_string())
    }
}

impl<'c> Subcommand<'c> for Switch<'c> {
    fn run(config: &'c Config, matches: &ArgMatches) -> Result<(), String> {
        let command = Self { config };

        let range: Option<VersionReq>;

        if let Some(arg) = matches.value_of("version") {
            // The argument is checked by clap in main.rs
            range = VersionReq::parse(arg).ok();
        } else {
            // TODO: Check for .nvmrc, parse it, etc...
            return Result::Err(String::from(".nvmrc files are not supported yet."));
        }

        if range.is_none() {
            return Result::Err(String::from("Did not get a version to switch to."));
        }

        if let Some(version) = InstalledNodeVersion::get_matching(config, range.unwrap().borrow()) {
            if !InstalledNodeVersion::is_installed(config, version.borrow()) {
                return Result::Err(format!("{} is not installed", version));
            }

            return command.set_shims(version.borrow());
        }

        Result::Err(String::from(
            "No version matching the version range was found.",
        ))
    }
}
