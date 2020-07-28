use std::borrow::Borrow;

use clap::ArgMatches;
use semver::VersionReq;

use crate::{
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::Subcommand,
};

pub struct Ls;

impl Ls {
    pub fn validate_filter(value: &str) -> Result<VersionReq, String> {
        VersionReq::parse(value).map_err(|_| String::from("Invalid semver range."))
    }
}

impl Subcommand for Ls {
    fn run(self, matches: &ArgMatches) -> Result<(), String> {
        let show_installed = !matches.is_present("online");
        let show_online = !matches.is_present("installed");

        let filter = matches
            .value_of("filter")
            .map(|version_str| VersionReq::parse(version_str).unwrap());

        let installed_versions = InstalledNodeVersion::get_all();
        let mut installed_versions_str = String::new();

        if show_installed {
            installed_versions_str = String::from("Installed versions:\n");

            installed_versions_str.push_str(
                installed_versions
                    .into_iter()
                    .map(|version| format!("{:15}", version.version()))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .borrow(),
            );

            // For formatting
            if show_installed && show_online {
                installed_versions_str.push('\n');
            }
        }

        let online_versions = OnlineNodeVersion::fetch_all();
        let mut online_versions_str = String::new();

        if show_online {
            online_versions_str = String::from("Available for download:\n");

            if online_versions.is_ok() {
                let versions = online_versions.unwrap();
                let versions = NodeVersion::filter_major_versions(versions);

                online_versions_str.push_str(
                    versions
                        .into_iter()
                        .map(|version| format!("{:15}{}", version.version(), version.release_date))
                        .collect::<Vec<String>>()
                        .join("\n")
                        .borrow(),
                );
            } else {
                online_versions_str = String::from("Could not fetch versions...");
            }

            online_versions_str.push('\n');
        }

        let hint = if filter.is_none() {
            String::from("Specify a version range to show more results.\ne.g. `nvm ls 12`")
        } else {
            String::new()
        };

        let output_str = format!(
            "{}\n{}\n{}",
            installed_versions_str, online_versions_str, hint
        );

        println!("{}", output_str.trim());

        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod filter_major_versions {
        use std::{borrow::Borrow, fs};

        use super::super::{NodeVersion, OnlineNodeVersion};

        #[test]
        fn filters_correctly() {
            let test_data = fs::read_to_string("test-data/node-versions.json").unwrap();
            let test_data: Vec<OnlineNodeVersion> =
                serde_json::from_str(test_data.borrow()).unwrap();

            assert_eq!(
                NodeVersion::filter_major_versions(test_data),
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
