use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use itertools::Itertools;
use node_semver::Range;

pub mod package_json;

const PACKAGE_JSON_FILE_NAME: PathBuf = "package.json".into();
const NVMRC_FILE_NAME: PathBuf = ".nvmrc".into();
const NODE_VERSION_FILE_NAME: PathBuf = ".node-version".into();
const ASDF_FILE_NAME: PathBuf = ".tool-versions".into();

pub enum VersionFile {
    Nvmrc(Range),
    PackageJson(package_json::PackageJson),
    Asdf(Range),
}

pub fn get_version_file() -> Option<VersionFile> {
    if PACKAGE_JSON_FILE_NAME.exists() {
        let parse_result = package_json::PackageJson::try_from(PACKAGE_JSON_FILE_NAME);

        if parse_result.is_ok() {
            return Some(VersionFile::PackageJson(parse_result.unwrap()));
        } else {
            println!(
                "Failed to parse package.json: {}",
                parse_result.unwrap_err()
            );
        }
    }

    if let Some(existingFile) = [NVMRC_FILE_NAME, NODE_VERSION_FILE_NAME]
        .iter()
        .find_or_first(|&path| path.exists())
    {
        let contents = fs::read_to_string(existingFile);

        if let Ok(contents) = contents {
            let parse_result = Range::parse(&contents);

            if parse_result.is_ok() {
                return Some(VersionFile::Nvmrc(parse_result.unwrap()));
            } else {
                println!(
                    "Failed to parse {}: '{}'",
                    existingFile.display(),
                    parse_result.unwrap_err().input(),
                );
            }
        }
    }

    if ASDF_FILE_NAME.exists() {
        let contents = fs::read_to_string(ASDF_FILE_NAME);

        if let Ok(contents) = contents {
            let version_string = contents
                .lines()
                .find(|line| line.starts_with("nodejs"))
                .map(|line| line.split(' ').nth(1))
                .flatten();

            if let Some(version_string) = version_string {
                let parse_result = Range::parse(&version_string);

                if parse_result.is_ok() {
                    return Some(VersionFile::Asdf(parse_result.unwrap()));
                } else {
                    println!(
                        "Failed to parse {}: '{}'",
                        ASDF_FILE_NAME.display(),
                        parse_result.unwrap_err().input(),
                    );
                }
            }
        }
    }

    None
}
