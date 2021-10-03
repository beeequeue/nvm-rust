use anyhow::Result;
use clap::ArgMatches;
use node_semver::Range;

use crate::{
    node_version::{InstalledNodeVersion, NodeVersion},
    old_config::OldConfig,
    subcommand::Subcommand,
    utils,
};

pub struct Uninstall {}

impl<'c> Subcommand<'c> for Uninstall {
    fn run(config: &'c OldConfig, matches: &ArgMatches) -> Result<()> {
        let force = matches.is_present("force");
        let input = matches.value_of("version").unwrap();
        let wanted_range = Range::parse(input).unwrap();

        if let Some(version) = InstalledNodeVersion::get_matching(config, &wanted_range) {
            if version.is_selected(config) {
                println!("{} is currently selected.", version.version());

                if !force
                    && !utils::confirm_choice(
                        String::from("Are you sure you want to uninstall it?"),
                        false,
                    )
                {
                    return Result::Ok(());
                }

                InstalledNodeVersion::deselect(config)?;
            }

            version.uninstall(config)
        } else {
            anyhow::bail!(
                "Did not find an installed version matching `{}`, (parsed as `{}`)",
                input,
                wanted_range
            )
        }
    }
}
