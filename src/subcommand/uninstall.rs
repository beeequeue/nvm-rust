use anyhow::Result;
use clap::ArgMatches;
use semver::VersionReq;

use crate::{config::Config, node_version::InstalledNodeVersion, subcommand::Subcommand};

pub struct Uninstall {}

impl Uninstall {}

impl<'c> Subcommand<'c> for Uninstall {
    fn run(config: &'c Config, matches: &ArgMatches) -> Result<()> {
        let wanted_range = VersionReq::parse(matches.value_of("version").unwrap()).unwrap();

        if let Some(version) = InstalledNodeVersion::get_matching(config, &wanted_range) {
            version.uninstall(config)
        } else {
            anyhow::bail!(
                "Did not find an installed version matching {}",
                wanted_range
            )
        }
    }
}
