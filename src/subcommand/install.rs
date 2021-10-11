use std::{borrow::Borrow, path::Path};

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use node_semver::Range;

use crate::{
    archives, node_version,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::{switch::SwitchCommand, Action},
    Config,
};

#[derive(Clap, Clone, Debug)]
#[clap(
about = "Install a new node version",
alias = "i",
alias = "add",
setting = AppSettings::ColoredHelp
)]
pub struct InstallCommand {
    /// A semver range. The latest version matching this range will be installed
    #[clap(validator = node_version::is_version_range)]
    pub version: Range,
    /// Switch to the new version after installing it
    #[clap(long, short)]
    pub switch: Option<bool>,
}

impl Action<InstallCommand> for InstallCommand {
    fn run(config: &Config, options: &InstallCommand) -> Result<()> {
        let online_versions = OnlineNodeVersion::fetch_all()?;
        let filtered_versions = node_version::filter_version_req(online_versions, &options.version);

        let version_to_install = filtered_versions.first().context(format!(
            "Did not find a version matching `{}`!",
            options.version
        ))?;

        if InstalledNodeVersion::is_installed(config, version_to_install.version()) {
            println!(
                "{} is already installed - skipping...",
                version_to_install.version()
            );

            return Result::Ok(());
        }

        download_and_extract_to(
            version_to_install.borrow(),
            &config
                .get_versions_dir()
                .join(version_to_install.to_string()),
        )?;

        if config.force
            || (options.switch.is_none()
                && dialoguer::Confirm::new()
                    .with_prompt(format!("Switch to {}?", version_to_install.to_string()))
                    .default(true)
                    .interact()?)
            || options.switch.unwrap()
        {
            SwitchCommand::run(
                &config.with_force(),
                &SwitchCommand {
                    version: Range::parse(version_to_install.to_string())?,
                },
            )?;
        }

        Result::Ok(())
    }
}

fn download_and_extract_to(version: &OnlineNodeVersion, path: &Path) -> Result<()> {
    let url = version.get_download_url().unwrap();

    println!("Downloading from {}...", url);
    let response = reqwest::blocking::get(url)
        .context(format!("Failed to download version: {}", version.version()))?;

    archives::extract_archive(response, path)
}
