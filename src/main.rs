#![feature(const_fn)]

use std::{num::ParseIntError, process::exit};

use clap::{clap_app, crate_version};

use config::Config;
use node_version::NodeVersion;
use subcommand::{install::Install, list::List, Subcommand};

mod config;
mod node_version;
mod subcommand;

fn validate_number(value: &str) -> Result<i32, String> {
    value.parse().map_err(|err: ParseIntError| err.to_string())
}

fn main() {
    let app = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@subcommand list =>
            (alias: "ls")
            (about: "List installed and released node versions")
            (@arg installed: -i --installed "Only display installed versions")
            (@arg online: -o --online --available "Only display available versions")
            (@arg filter: {NodeVersion::is_version_range} "Filter by semantic versions. e.g. `12`, `^10.9`, `>=8.10`, `>=8, <9`")
        )
        (@subcommand install =>
            (alias: "i")
            (about: "Install a new node version")
            (@arg force: -f --force "Install version even if it's already installed")
            (@arg version: +required {NodeVersion::is_version_range} "A semver range. The latest version matching this range will be installed.")
        )
    );

    let config = Config::from_env_and_args(app.get_arguments());
    let matches = app.get_matches();
    let result = match matches.subcommand_name() {
        Some("list") => List::run(matches.subcommand_matches("list").unwrap()),
        Some("install") => Install::run(matches.subcommand_matches("install").unwrap()),
        _ => Result::Ok(()),
    };

    println!(
        "{}",
        if result.is_err() {
            result.clone().unwrap_err()
        } else {
            String::from("OK")
        }
    );

    if result.is_err() {
        exit(1)
    }
}
