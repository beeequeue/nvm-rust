use reqwest::Url;
use semver::Version;
use serde::Deserialize;
use std::borrow::Borrow;

pub trait NodeVersion {
    fn version(&self) -> Version;
    fn release_date(&self) -> String;
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct OnlineNodeVersion {
    #[serde(alias = "version")]
    version_str: String,
    #[serde(alias = "date")]
    release_date: String,
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
    // Constructor verifies that version_str is valid semver.
    fn version(&self) -> Version {
        // Required since the versions are prefixed with 'v' which `semver` can't handle
        let clean_version = if self.version_str.starts_with('v') {
            self.version_str.get(1..).unwrap()
        } else {
            self.version_str.borrow()
        };

        Version::parse(clean_version).unwrap()
    }

    fn release_date(&self) -> String {
        self.release_date.clone()
    }
}
