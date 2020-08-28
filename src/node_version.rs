use std::{borrow::Borrow, collections::HashSet, fs::remove_dir_all, path::PathBuf};

use anyhow::{Context, Result};
use reqwest::Url;
use semver::{Version, VersionReq};
use serde::Deserialize;

use crate::config::Config;

pub trait NodeVersion {
    fn version(&self) -> Version;
}

impl dyn NodeVersion {
    pub fn is_version_range(value: &str) -> Result<VersionReq> {
        VersionReq::parse(value).context(value.to_string())
    }

    // Filters out relevant major versions. Relevant meaning anything >=10
    pub fn filter_default<V: NodeVersion>(versions: Vec<V>) -> Vec<V> {
        let relevant_versions = VersionReq::parse(">=10").unwrap();
        let mut found_major_versions: HashSet<u64> = HashSet::new();

        let major_versions = versions
            .into_iter()
            .filter(|version| {
                let version = version.version();
                let major = version.major;

                if found_major_versions.contains(major.borrow()) {
                    return false;
                }

                found_major_versions.insert(major);

                true
            })
            .collect();

        Self::filter_version_req(major_versions, relevant_versions)
    }

    pub fn filter_version_req<V: NodeVersion>(
        versions: Vec<V>,
        version_range: VersionReq,
    ) -> Vec<V> {
        versions
            .into_iter()
            .filter(|version| version_range.matches(version.version().borrow()))
            .collect()
    }
}

/// Handles `vX.X.X` prefixes
fn parse_version_str(version_str: String) -> Result<Version> {
    // Required since the versions are prefixed with 'v' which `semver` can't handle
    let clean_version = if version_str.starts_with('v') {
        version_str.get(1..).unwrap()
    } else {
        version_str.borrow()
    };

    Version::parse(clean_version).context(version_str)
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct OnlineNodeVersion {
    #[serde(alias = "version")]
    version_str: String,
    #[serde(alias = "date")]
    pub release_date: String,
    files: Vec<String>,
}

impl OnlineNodeVersion {
    pub fn fetch_all() -> Result<Vec<Self>> {
        let response = reqwest::blocking::get("https://nodejs.org/dist/index.json")?;

        let body = response.text().unwrap();

        serde_json::from_str(body.borrow()).context("Failed to fetch versions from nodejs.org")
    }

    pub fn get_download_url(&self) -> Result<Url> {
        let file_name = self.get_file();

        let url = format!("https://nodejs.org/dist/{}/{}", self.version_str, file_name);

        Url::parse(url.borrow())
            .context(format!("Could not create a valid download url. [{}]", url))
    }

    #[cfg(target_os = "windows")]
    fn get_file(&self) -> String {
        format!(
            "node-v{version}-win-{arch}.zip",
            version = self.version(),
            arch = if cfg!(target_arch = "x86") {
                "x86"
            } else {
                "x64"
            },
        )
    }

    #[cfg(target_os = "macos")]
    fn get_file(&self) -> String {
        format!(
            "node-v{version}-darwin-x64.tar.gz",
            version = self.version()
        )
    }

    #[cfg(target_os = "linux")]
    fn get_file(&self) -> String {
        format!("node-v{version}-linux-x64.tar.gz", version = self.version())
    }

    #[cfg(test)]
    pub fn new(version_str: String, release_date: String, files: Vec<String>) -> Self {
        Self {
            version_str,
            release_date,
            files,
        }
    }
}

impl NodeVersion for OnlineNodeVersion {
    fn version(&self) -> Version {
        parse_version_str(self.version_str.clone())
            .expect("Got bad version into OnlineNodeVersion.")
    }
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct InstalledNodeVersion {
    version_str: String,
    path: PathBuf,
}

impl InstalledNodeVersion {
    // Properties

    pub fn get_dir_path(self, config: &Config) -> PathBuf {
        config.shims_dir.join(self.version().to_string())
    }

    fn get_ext() -> String {
        String::from(if cfg!(windows) { ".cmd" } else { "" })
    }

    // Functions

    pub fn uninstall(self, config: &Config) -> Result<()> {
        remove_dir_all(self.get_dir_path(config))?;

        Result::Ok(())
    }

    pub fn is_installed(config: &Config, version: &Version) -> bool {
        Self::get_all(config)
            .iter()
            .any(|v| v.version().eq(version))
    }

    /// Returns all the installed, valid node versions in `Config.dir`
    pub fn get_all(config: &Config) -> Vec<InstalledNodeVersion> {
        let base_path = config.dir.to_owned();
        let mut version_dirs: Vec<Version> = vec![];

        for entry in base_path.read_dir().unwrap() {
            if entry.is_err() {
                println!("Could not read {:?}", entry);
                continue;
            }

            let entry = entry.unwrap();
            let result = parse_version_str(String::from(entry.file_name().to_string_lossy()));

            if let Result::Ok(version) = result {
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
                    version_str: version_str.clone(),
                    path: [base_path.to_str().unwrap(), version_str.borrow()]
                        .iter()
                        .collect(),
                }
            })
            .collect()
    }

    /// Returns the latest, installed version matching the version range
    pub fn get_matching(config: &Config, range: &VersionReq) -> Option<InstalledNodeVersion> {
        Self::get_all(config)
            .iter()
            .find(|inv| range.matches(inv.version().borrow()))
            .map(|inv| inv.to_owned())
    }

    /// Checks that all the required files are present in the installation dir
    pub fn validate(&self, config: &Config) -> Result<()> {
        let base_path = config.dir.to_owned();
        let version_dir: PathBuf = [base_path.to_str().unwrap(), ""].iter().collect();

        let mut required_files = vec![version_dir; 2];
        required_files[0].set_file_name(format!("node{}", Self::get_ext()));
        required_files[1].set_file_name(format!("npm{}", Self::get_ext()));

        if let Some(missing_file) = required_files.iter().find(|file| !file.exists()) {
            anyhow::bail!(
                "{:?} is not preset for {:?}",
                missing_file,
                self.version_str
            );
        }

        Result::Ok(())
    }
}

impl NodeVersion for InstalledNodeVersion {
    fn version(&self) -> Version {
        parse_version_str(self.version_str.clone())
            .expect("Got bad version into InstalledNodeVersion.")
    }
}
