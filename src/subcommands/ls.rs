use clap::ArgMatches;
use semver::VersionReq;

pub fn run(matches: ArgMatches) {}

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
