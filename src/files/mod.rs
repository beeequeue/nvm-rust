use std::{fs, path::PathBuf};

use itertools::Itertools;
use node_semver::Range;

pub mod package_json;

const PACKAGE_JSON_FILE_NAME: &str = "package.json";
const NVMRC_FILE_NAME: &str = ".nvmrc";
const NODE_VERSION_FILE_NAME: &str = ".node-version";
const ASDF_FILE_NAME: &str = ".tool-versions";

pub enum VersionFile {
    Nvmrc(Range),
    PackageJson(Range),
    Asdf(Range),
}

impl VersionFile {
    pub fn range(self) -> Range {
        match self {
            VersionFile::Nvmrc(range) => range,
            VersionFile::PackageJson(range) => range,
            VersionFile::Asdf(range) => range,
        }
    }
}

pub fn get_version_file() -> Option<VersionFile> {
    if PathBuf::from(PACKAGE_JSON_FILE_NAME).exists() {
        let parse_result =
            package_json::PackageJson::try_from(PathBuf::from(PACKAGE_JSON_FILE_NAME));

        if let Ok(parse_result) = parse_result {
            return parse_result
                .engines
                .and_then(|engines| engines.node)
                .map(VersionFile::PackageJson);
        } else {
            println!(
                "Failed to parse package.json: {}",
                parse_result.unwrap_err()
            );
        }
    }

    if let Some(existing_file) = [NVMRC_FILE_NAME, NODE_VERSION_FILE_NAME]
        .iter()
        .find_or_first(|&path| PathBuf::from(path).exists())
    {
        let contents = fs::read_to_string(existing_file);

        if let Ok(contents) = contents {
            let parse_result = Range::parse(&contents);

            if let Ok(parse_result) = parse_result {
                return Some(VersionFile::Nvmrc(parse_result));
            } else {
                println!(
                    "Failed to parse {}: '{}'",
                    existing_file,
                    parse_result.unwrap_err().input(),
                );
            }
        }
    }

    if PathBuf::from(ASDF_FILE_NAME).exists() {
        let contents = fs::read_to_string(ASDF_FILE_NAME);

        if let Ok(contents) = contents {
            let version_string = contents
                .lines()
                .find(|line| line.starts_with("nodejs"))
                .and_then(|line| line.split(' ').nth(1));

            if let Some(version_string) = version_string {
                let parse_result = Range::parse(&version_string);

                if let Ok(parse_result) = parse_result {
                    return Some(VersionFile::Asdf(parse_result));
                } else {
                    println!(
                        "Failed to parse {}: '{}'",
                        ASDF_FILE_NAME,
                        parse_result.unwrap_err().input(),
                    );
                }
            }
        }
    }

    None
}
