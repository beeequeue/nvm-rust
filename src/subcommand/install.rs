use std::{borrow::Borrow, path::Path};

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use node_semver::Range;

use crate::{
    actions::Action,
    archives, node_version,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    Config,
};

/// List installed and released node versions
#[derive(Clap, Debug)]
#[clap(
about = "Install a new node version",
alias = "i",
alias = "add",
setting = AppSettings::ColoredHelp
)]
pub struct InstallCommand {
    /// Filter by semantic versions.
    #[clap(validator = node_version::is_version_range)]
    pub version: Range,
}

impl Action<InstallCommand> for InstallCommand {
    fn run(config: &Config, options: &InstallCommand) -> Result<()> {
        let online_versions = OnlineNodeVersion::fetch_all()?;
        let filtered_versions = node_version::filter_version_req(online_versions, &options.version);

        if let Some(latest_version) = filtered_versions.first() {
            if InstalledNodeVersion::is_installed(config, &latest_version.version()) {
                println!(
                    "{} is already installed - skipping...",
                    latest_version.version()
                );

                return Result::Ok(());
            }

            download_and_extract_to(
                latest_version.borrow(),
                &config.get_versions_dir().join(latest_version.to_string()),
            )?;
        } else {
            anyhow::bail!("Did not find a version matching `{}`!", options.version);
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
