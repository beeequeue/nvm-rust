use crate::config::Config;
use crate::subcommand::Subcommand;
use clap::ArgMatches;
use semver::VersionReq;
use serde::Deserialize;
use std::borrow::Borrow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct ResponseVersion {
    // version: Version,
    #[serde(rename = "version")]
    version_string: String,
    files: Vec<String>,
    lts: bool,
    // date: Instant,
}

pub struct Ls;

impl Ls {
    pub fn validate_filter(value: &str) -> Result<(), String> {
        match value {
            val if (val.to_lowercase() == "lts") => Result::Ok(()),
            val => {
                let parse_result = VersionReq::parse(val);

                if parse_result.is_err() {
                    return Result::Err(String::from("Invalid version."));
                }

                Result::Ok(())
            }
        }
    }

    fn fetch_versions() -> Result<Vec<ResponseVersion>, String> {
        let response = reqwest::blocking::get("https://nodejs.org/dist/inde.json");

        if response.is_err() {
            return Result::Err(response.unwrap_err().to_string());
        }

        let body = response.unwrap().text().unwrap();
        let versions: Vec<ResponseVersion> = serde_json::from_str(body.borrow()).unwrap();

        println!("{:?}", versions);

        Result::Ok(versions)
    }
}

impl Subcommand for Ls {
    fn run(matches: ArgMatches, _: Config) -> Result<(), String> {
        let versions = Self::fetch_versions();

        if versions.is_err() {
            return Result::Err(versions.unwrap_err());
        }

        Result::Ok(())
    }
}
