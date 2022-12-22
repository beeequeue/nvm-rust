use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{read_link, remove_dir_all},
    path::PathBuf,
};

use anyhow::{Context, Result};
use node_semver::{Range, Version};
use serde::Deserialize;

use crate::{utils, Config};

#[cfg(target_os = "windows")]
const PLATFORM: &str = "win";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "darwin";
#[cfg(target_os = "linux")]
const PLATFORM: &str = "linux";

#[cfg(target_os = "windows")]
const EXT: &str = ".zip";
#[cfg(target_os = "macos")]
const EXT: &str = ".tar.gz";
#[cfg(target_os = "linux")]
const EXT: &str = ".tar.gz";

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x64";
#[cfg(target_arch = "x86")]
const ARCH: &str = "x86";
#[cfg(target_arch = "aarch64")]
const ARCH: &str = "arm64";

pub trait NodeVersion {
    fn version(&self) -> &Version;
}

impl PartialEq<Self> for dyn NodeVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version().eq(other.version())
    }
}

impl Eq for dyn NodeVersion {}

impl PartialOrd<Self> for dyn NodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.version().cmp(other.version()))
    }
}

impl Ord for dyn NodeVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version().cmp(other.version())
    }
}

pub fn parse_range(value: &str) -> Result<Range> {
    Range::parse(value).context(value.to_string())
}

pub fn filter_version_req<V: NodeVersion>(versions: Vec<V>, version_range: &Range) -> Vec<V> {
    versions
        .into_iter()
        .filter(|version| version_range.satisfies(version.version()))
        .collect()
}

pub fn get_latest_of_each_major<V: NodeVersion>(versions: &[V]) -> Vec<&V> {
    let mut map: HashMap<u64, &V> = HashMap::new();

    for version in versions.iter() {
        let entry = map.get_mut(&version.version().major);
        if entry.is_some() && version.version().lt(entry.unwrap().version()) {
            continue;
        }

        map.insert(version.version().major, version);
    }

    map.values().cloned().collect()
}

/// Handles `vX.X.X` prefixes
fn parse_version_str(version_str: &str) -> Result<Version> {
    // Required since the versions are prefixed with 'v' which `semver` can't handle
    let clean_version = if version_str.starts_with('v') {
        version_str.get(1..).unwrap()
    } else {
        version_str
    };

    Version::parse(clean_version).context(version_str.to_owned())
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct OnlineNodeVersion {
    #[serde()]
    version: Version,
    #[serde(alias = "date")]
    pub release_date: String,

    files: Vec<String>,
}

impl OnlineNodeVersion {
    pub fn fetch_all() -> Result<Vec<Self>> {
        let response = ureq::get("https://nodejs.org/dist/index.json").call()?;

        response
            .into_json()
            .context("Failed to parse versions list from nodejs.org")
    }

    pub fn install_path(&self, config: &Config) -> PathBuf {
        config.get_versions_dir().join(self.to_string())
    }

    pub fn download_url(&self) -> String {
        let file_name = self.file();

        format!("https://nodejs.org/dist/v{}/{}", self.version, file_name)
    }

    fn file(&self) -> String {
        format!("node-v{}-{PLATFORM}-{ARCH}{EXT}", self.version())
    }
}

impl ToString for OnlineNodeVersion {
    fn to_string(&self) -> String {
        self.version.to_string()
    }
}

impl NodeVersion for OnlineNodeVersion {
    fn version(&self) -> &Version {
        &self.version
    }
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct InstalledNodeVersion {
    version: Version,
    path: PathBuf,
}

impl InstalledNodeVersion {
    // Properties

    pub fn get_dir_path(&self, config: &Config) -> PathBuf {
        config.get_versions_dir().join(self.version().to_string())
    }

    pub fn is_installed(config: &Config, version: &Version) -> bool {
        Self::list(config).iter().any(|v| v.version().eq(version))
    }

    pub fn is_selected(&self, config: &Config) -> bool {
        let path = config.get_shims_dir();
        let real_path = read_link(path);

        if real_path.is_err() {
            return false;
        }

        let real_path = real_path.unwrap();

        real_path
            .to_string_lossy()
            .contains(&self.version().to_string())
    }

    // Functions

    pub fn uninstall(self, config: &Config) -> Result<()> {
        remove_dir_all(self.get_dir_path(config))?;

        println!("Uninstalled {}!", self.version());
        Ok(())
    }

    /// Checks that all the required files are present in the installation dir
    #[allow(dead_code)]
    pub fn validate(&self, config: &Config) -> Result<()> {
        let version_dir =
            read_link(config.get_shims_dir()).expect("Could not read installation dir");

        let mut required_files = vec![version_dir; 2];
        required_files[0].set_file_name(format!("node{}", utils::exec_ext()));
        required_files[1].set_file_name(format!("npm{}", utils::exec_ext()));

        if let Some(missing_file) = required_files.iter().find(|file| !file.exists()) {
            anyhow::bail!(
                "{:?} is not preset for {:?}",
                missing_file,
                self.version.to_string()
            );
        }

        Ok(())
    }

    // Static functions

    pub fn deselect(config: &Config) -> Result<()> {
        remove_dir_all(config.get_shims_dir()).map_err(anyhow::Error::from)
    }

    pub fn list(config: &Config) -> Vec<InstalledNodeVersion> {
        let mut version_dirs: Vec<Version> = vec![];

        for entry in config
            .get_versions_dir()
            .read_dir()
            .expect("Failed to read nvm dir")
        {
            if entry.is_err() {
                println!("Could not read {entry:?}");
                continue;
            }

            let entry = entry.unwrap();
            let result = parse_version_str(entry.file_name().to_string_lossy().as_ref());

            if let Ok(version) = result {
                version_dirs.push(version);
            }
        }

        version_dirs.sort();
        version_dirs.reverse();

        version_dirs
            .iter()
            .map(|version| {
                let version_str = version.to_string();

                InstalledNodeVersion {
                    version: parse_version_str(&version_str)
                        .expect("Got bad version into InstalledNodeVersion."),
                    path: config.get_versions_dir().join(&version_str),
                }
            })
            .collect()
    }

    /// Returns the latest, installed version matching the version range
    pub fn find_matching(config: &Config, range: &Range) -> Option<InstalledNodeVersion> {
        Self::list(config)
            .iter()
            .find(|inv| range.satisfies(inv.version()))
            .map(|inv| inv.to_owned())
    }
}

impl ToString for InstalledNodeVersion {
    fn to_string(&self) -> String {
        self.version.to_string()
    }
}

impl NodeVersion for InstalledNodeVersion {
    fn version(&self) -> &Version {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    mod online_version {
        use anyhow::Result;
        use node_semver::Version;

        use spectral::prelude::*;

        use crate::node_version::OnlineNodeVersion;

        #[test]
        fn formats_file_name_correctly() -> Result<()> {
            let version = OnlineNodeVersion {
                version: Version::from((18, 12, 1)),
                release_date: "".to_string(),
                files: vec![],
            };

            assert_that!(version.file())
                .is_equal_to("node-v18.12.1-darwin-arm64.tar.gz".to_string());

            Ok(())
        }

        #[test]
        fn can_parse_version_data() -> Result<()> {
            let expected = OnlineNodeVersion {
                version: Version {
                    major: 14,
                    minor: 18,
                    patch: 0,
                    build: vec![],
                    pre_release: vec![],
                },
                release_date: "2021-09-28".to_string(),
                files: vec![
                    "aix-ppc64".to_string(),
                    "headers".to_string(),
                    "linux-arm64".to_string(),
                    "linux-armv7l".to_string(),
                    "linux-ppc64le".to_string(),
                    "linux-s390x".to_string(),
                    "linux-x64".to_string(),
                    "osx-x64-pkg".to_string(),
                    "osx-x64-tar".to_string(),
                    "src".to_string(),
                    "win-x64-7z".to_string(),
                    "win-x64-exe".to_string(),
                    "win-x64-msi".to_string(),
                    "win-x64-zip".to_string(),
                    "win-x86-7z".to_string(),
                    "win-x86-exe".to_string(),
                    "win-x86-msi".to_string(),
                    "win-x86-zip".to_string(),
                ],
            };

            let json_str = r#"
{
    "version": "v14.18.0",
    "date": "2021-09-28",
    "files": [
      "aix-ppc64",
      "headers",
      "linux-arm64",
      "linux-armv7l",
      "linux-ppc64le",
      "linux-s390x",
      "linux-x64",
      "osx-x64-pkg",
      "osx-x64-tar",
      "src",
      "win-x64-7z",
      "win-x64-exe",
      "win-x64-msi",
      "win-x64-zip",
      "win-x86-7z",
      "win-x86-exe",
      "win-x86-msi",
      "win-x86-zip"
    ],
    "npm": "6.14.15",
    "v8": "8.4.371.23",
    "uv": "1.42.0",
    "zlib": "1.2.11",
    "openssl": "1.1.1l",
    "modules": "83",
    "lts": "Fermium",
    "security": false
}
"#
            .trim();

            let result: OnlineNodeVersion = serde_json::from_str(json_str)
                .expect("Failed to parse version data from nodejs.org");

            assert_that!(expected).is_equal_to(result);

            Ok(())
        }
    }
}
