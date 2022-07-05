use anyhow::Result;
use clap::{AppSettings, Parser};
use itertools::Itertools;
use node_semver::Range;

use crate::{
    node_version,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::Action,
    Config,
};

enum VersionStatus<'p> {
    Latest,
    NotInstalled,
    Outdated(&'p OnlineNodeVersion),
}

impl<'p> VersionStatus<'p> {
    fn from<T: NodeVersion>(versions: &[&T], latest: &'p OnlineNodeVersion) -> VersionStatus<'p> {
        if versions.is_empty() {
            VersionStatus::NotInstalled
        } else if versions
            .iter()
            .all(|version| version.version() < latest.version())
        {
            VersionStatus::Outdated(latest)
        } else {
            VersionStatus::Latest
        }
    }

    fn to_emoji(&self) -> char {
        match self {
            VersionStatus::Latest => '✅',
            VersionStatus::NotInstalled => '〰',
            VersionStatus::Outdated(_) => '⏫',
        }
    }

    fn to_version_string(&self) -> String {
        match self {
            VersionStatus::Outdated(version) => format!("-> {}", version.to_string()),
            _ => "".to_string(),
        }
    }
}

#[derive(Parser, Clone, Debug)]
#[clap(
about = "List installed and released node versions",
alias = "ls",
setting = AppSettings::ColoredHelp
)]
pub struct ListCommand {
    /// Only display installed versions
    #[clap(short, long, alias = "installed")]
    pub local: bool,
    /// Filter by semantic versions.
    ///
    /// `12`, `^10.9`, `>=8.10`, `>=8, <9`
    #[clap(short('F'), long, validator = node_version::is_version_range)]
    pub filter: Option<Range>,
}

impl Action<ListCommand> for ListCommand {
    fn run(config: &Config, options: &ListCommand) -> Result<()> {
        let mut installed_versions = InstalledNodeVersion::list(config);

        // Use filter option if it was passed
        if let Some(filter) = &options.filter {
            installed_versions = node_version::filter_version_req(installed_versions, filter);
        }

        if options.local {
            println!(
                "{}",
                installed_versions
                    .iter()
                    .map(|version| version.to_string())
                    .join("\n")
            );

            return Ok(());
        }

        // Get available versions, extract only the latest for each major version
        let mut latest_per_major = Vec::<&OnlineNodeVersion>::new();
        let online_versions = OnlineNodeVersion::fetch_all()?;
        if !online_versions.is_empty() {
            latest_per_major = node_version::get_latest_of_each_major(&online_versions);
            latest_per_major.sort();
            latest_per_major.reverse();
        }

        let majors_and_installed_versions: Vec<(&OnlineNodeVersion, Vec<&InstalledNodeVersion>)> =
            latest_per_major
                .into_iter()
                .map(|latest| {
                    (
                        latest,
                        installed_versions
                            .iter()
                            .filter(|installed| installed.version().major == latest.version().major)
                            .collect(),
                    )
                })
                .collect();

        // Show the latest X major versions by default
        // and show any older, installed versions as well
        let mut versions_to_show = Vec::<(&OnlineNodeVersion, &Vec<&InstalledNodeVersion>)>::new();
        for (i, (latest, installed)) in majors_and_installed_versions.iter().enumerate() {
            if i < 5 || !installed.is_empty() {
                versions_to_show.push((latest, installed));
            }
        }

        let output = versions_to_show
            .iter()
            .map(|(online_version, installed_versions)| {
                let version_status = VersionStatus::from(installed_versions, online_version);

                let version_to_show = if installed_versions.is_empty() {
                    online_version.to_string()
                } else {
                    installed_versions[0].to_string()
                };

                format!(
                    "{} {} {}",
                    &version_status.to_emoji(),
                    version_to_show.to_string(),
                    &version_status.to_version_string(),
                )
            })
            .join("\n");

        println!("{output}");
        Ok(())
    }
}
