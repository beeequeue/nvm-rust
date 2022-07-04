use std::{borrow::Borrow, path::Path};

use anyhow::{Context, Result};
use clap::{AppSettings, Parser};
use node_semver::Range;

use crate::{
    archives, files, node_version,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::{switch::SwitchCommand, Action},
    Config,
};

#[derive(Parser, Clone, Debug)]
#[clap(
about = "Install a new node version",
alias = "i",
alias = "add",
setting = AppSettings::ColoredHelp
)]
pub struct InstallCommand {
    /// A semver range. The latest version matching this range will be installed
    #[clap(validator = node_version::is_version_range)]
    pub version: Option<Range>,
    /// Switch to the new version after installing it
    #[clap(long, short, default_value("false"))]
    pub switch: bool,
}

impl Action<InstallCommand> for InstallCommand {
    fn run(config: &Config, options: &InstallCommand) -> Result<()> {
        let version_filter = options
            .version
            .xor(files::get_version_file().map(|version_file| version_file.range()));

        if version_filter.is_none() {
            anyhow::bail!("You did not pass a version and we did not find any version files (package.json#engines, .nvmrc) in the current directory.");
        }
        let version_filter = version_filter.unwrap();

        let online_versions = OnlineNodeVersion::fetch_all()?;
        let filtered_versions = node_version::filter_version_req(online_versions, &version_filter);

        let version_to_install = filtered_versions.first().context(format!(
            "Did not find a version matching `{}`!",
            &version_filter
        ))?;

        if !config.force && InstalledNodeVersion::is_installed(config, version_to_install.version())
        {
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
            || (options.switch
                && dialoguer::Confirm::new()
                    .with_prompt(format!("Switch to {}?", version_to_install.to_string()))
                    .default(true)
                    .interact()?)
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
