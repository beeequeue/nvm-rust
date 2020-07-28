use std::borrow::Borrow;
use std::path::PathBuf;

use reqwest::Url;
use semver::{SemVerError, Version};
use serde::Deserialize;

use crate::CONFIG;

pub trait NodeVersion {
    fn version(&self) -> Version;
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
    pub fn download_url(&self) -> Result<Url, String> {
        let file_name = self.get_file();

        let url = format!("https://nodejs.org/dist/{}/{}", self.version_str, file_name);

        Url::parse(url.borrow())
            .map_err(|_| format!("Could not create a valid download url. [{}]", url))
    }

    /// Returns the correct file name based on OS
    fn get_file(&self) -> String {
        let mut platform = "linux";
        let mut arch = "x64";
        let mut ext = ".tar.gz";

        if cfg!(target_os = "windows") {
            platform = "win";
            ext = ".zip";

            if cfg!(target_arch = "x86") {
                arch = "x86";
            }
        }

        if cfg!(target_os = "macos") {
            platform = "darwin";
        }

        format!(
            "node-v{version}-{platform}-{arch}.{ext}",
            version = self.version_str,
            platform = platform,
            arch = arch,
            ext = ext,
        )
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
