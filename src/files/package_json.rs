use std::{fs, path::PathBuf};

use node_semver::Range;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct PackageJson {
    #[serde()]
    pub name: Option<String>,
    #[serde()]
    pub version: Option<String>,
    #[serde()]
    pub engines: Option<PackageJsonEngines>,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct PackageJsonEngines {
    #[serde()]
    pub node: Option<Range>,
}

impl TryFrom<PathBuf> for PackageJson {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, anyhow::Error> {
        let contents = fs::read_to_string(path)?;
        let package_json: PackageJson = serde_json::from_str(&contents)?;

        Ok(package_json)
    }
}
