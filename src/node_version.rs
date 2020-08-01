use std::{borrow::Borrow, collections::HashSet, path::PathBuf};

use reqwest::Url;
use semver::{SemVerError, Version, VersionReq};
use serde::Deserialize;

use crate::CONFIG;

pub trait NodeVersion {
    fn version(&self) -> Version;
}

impl dyn NodeVersion {
    pub fn is_version_range(value: &str) -> Result<VersionReq, String> {
        VersionReq::parse(value).map_err(|_| String::from("Invalid semver range."))
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

                found_major_versions.insert(major.clone());

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
fn parse_version_str(version_str: String) -> Result<Version, SemVerError> {
    // Required since the versions are prefixed with 'v' which `semver` can't handle
    let clean_version = if version_str.starts_with('v') {
        version_str.get(1..).unwrap()
    } else {
        version_str.borrow()
    };

    Version::parse(clean_version)
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
    pub fn fetch_all() -> Result<Vec<Self>, String> {
        let response = reqwest::blocking::get("https://nodejs.org/dist/index.json");

        if response.is_err() {
            return Result::Err(response.unwrap_err().to_string());
        }

        let body = response.unwrap().text().unwrap();

        serde_json::from_str(body.borrow()).map_err(|err| {
            println!("{}", err);
            err.to_string()
        })
    }

    pub fn download_url(&self) -> Result<Url, String> {
        let file_name = self.get_file();

        let url = format!("https://nodejs.org/dist/{}/{}", self.version_str, file_name);

        Url::parse(url.borrow())
            .map_err(|_| format!("Could not create a valid download url. [{}]", url))
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
    pub fn new(version_str: String, path: PathBuf) -> Self {
        Self { version_str, path }
    }

    pub fn is_installed(version: &Version) -> bool {
        Self::get_all().iter().any(|v| v.version().eq(version))
    }

    /// Returns all the installed, valid node versions in `Config.dir`
    pub fn get_all() -> Vec<InstalledNodeVersion> {
        let base_path = CONFIG.dir().clone();
        let mut version_dirs: Vec<Version> = vec![];

        for entry in base_path.read_dir().unwrap() {
            if entry.is_err() {
                println!("Could not read {:?}", entry);
                continue;
            }

            let entry = entry.unwrap();
            let version = parse_version_str(String::from(entry.file_name().to_string_lossy()));

            if entry.metadata().unwrap().is_dir() && version.is_ok() {
                version_dirs.push(version.unwrap());
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

    /// Checks that all the required files are present in the installation dir
    pub fn validate(&self) -> Result<(), String> {
        let base_path = CONFIG.dir().clone();
        let version_dir: PathBuf = [base_path.to_str().unwrap(), ""].iter().collect();

        let mut required_files = vec![version_dir.clone(); 2];
        required_files[0].set_file_name(format!("node{}", Self::get_ext()));
        required_files[1].set_file_name(format!("npm{}", Self::get_ext()));

        if let Some(missing_file) = required_files.iter().find(|file| !file.exists()) {
            return Result::Err(format!(
                "{:#?} is missing in {}",
                missing_file.file_name().unwrap(),
                self.version()
            ));
        }

        Result::Ok(())
    }

    fn get_ext() -> String {
        String::from(if cfg!(windows) { ".exe" } else { "" })
    }
}

impl NodeVersion for InstalledNodeVersion {
    fn version(&self) -> Version {
        parse_version_str(self.version_str.clone())
            .expect("Got bad version into InstalledNodeVersion.")
    }
}
