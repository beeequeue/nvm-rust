use std::{collections::HashMap, ops::Deref};

use anyhow::Result;
use clap::{AppSettings, Clap};
use node_semver::Range;

use crate::{
    node_version,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::Action,
    Config,
};

enum VersionStatus {
    Outdated(OnlineNodeVersion),
    Latest,
    Unknown,
}

fn emoji_from(status: &VersionStatus) -> char {
    match status {
        VersionStatus::Outdated(_) => '⏫',
        _ => '✅',
    }
}

fn latest_version_string_from(status: &VersionStatus) -> String {
    match status {
        VersionStatus::Outdated(version) => format!("-> {}", version.to_string()),
        VersionStatus::Latest => "".to_string(),
        _ => "-> unknown".to_string(),
    }
}

#[derive(Clap, Clone, Debug)]
#[clap(
about = "List installed and released node versions",
alias = "ls",
setting = AppSettings::ColoredHelp
)]
pub struct ListCommand {
    /// Only display installed versions
    #[clap(short, long)]
    pub installed: Option<bool>,
    /// Only display available versions
    #[clap(short, long, takes_value(false))]
    pub online: Option<bool>,
    /// Filter by semantic versions.
    ///
    /// `12`, `^10.9`, `>=8.10`, `>=8, <9`
    #[clap(short, long, validator = node_version::is_version_range)]
    pub filter: Option<Range>,
}

impl Action<ListCommand> for ListCommand {
    fn run(config: &Config, options: &ListCommand) -> Result<()> {
        let mut installed_versions = InstalledNodeVersion::list(config);

        // Use filter option if it was passed
        if let Some(filter) = &options.filter {
            installed_versions = node_version::filter_version_req(installed_versions, filter);
        }

        let mut latest_per_major: HashMap<u64, &OnlineNodeVersion> = HashMap::new();
        let online_versions = OnlineNodeVersion::fetch_all()?;
        if !online_versions.is_empty() {
            latest_per_major = node_version::get_latest_of_each_major(&online_versions);
        }

        let lines: Vec<String> = installed_versions
            .iter()
            .map(|version| {
                let version_status = match latest_per_major.get(&version.version().major) {
                    Some(latest) if latest.version().gt(&version.version()) => {
                        VersionStatus::Outdated(latest.deref().clone())
                    },
                    Some(_) => VersionStatus::Latest,
                    None => VersionStatus::Unknown,
                };

                format!(
                    "{} {} {}",
                    emoji_from(&version_status),
                    version.to_string(),
                    latest_version_string_from(&version_status)
                )
            })
            .collect();

        println!("{}", lines.join("\n"));
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod filter_default {
        use std::{borrow::Borrow, fs};

        use super::super::{node_version, OnlineNodeVersion};

        #[test]
        fn filters_correctly() {
            let test_data = fs::read_to_string("test-data/node-versions.json").unwrap();
            let test_data: Vec<OnlineNodeVersion> =
                serde_json::from_str(test_data.borrow()).unwrap();

            assert_eq!(
                node_version::filter_default(test_data),
                vec![
                    OnlineNodeVersion::new(
                        String::from("14.6.0"),
                        String::from("2020-07-15"),
                        vec![],
                    ),
                    OnlineNodeVersion::new(
                        String::from("13.14.0"),
                        String::from("2020-04-28"),
                        vec![],
                    ),
                    OnlineNodeVersion::new(
                        String::from("12.18.3"),
                        String::from("2020-07-22"),
                        vec![],
                    ),
                    OnlineNodeVersion::new(
                        String::from("11.15.0"),
                        String::from("2019-04-30"),
                        vec![],
                    ),
                ]
            );
        }
    }
}
