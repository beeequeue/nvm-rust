use anyhow::{Context, Result};
use clap::{clap_app, crate_version};

use crate::subcommand::parse_version::ParseVersion;
use config::Config;
use subcommand::{install::Install, list::List, switch::Switch, uninstall::Uninstall, Subcommand};

mod config;
mod node_version;
mod subcommand;
mod utils;

fn validate_number(value: &str) -> Result<i32> {
    value.parse().context(format!("{} is not a number!", value))
}

fn main() -> Result<()> {
    let app = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@arg verbose: -V --verbose "Print debugging information")
        (@subcommand list =>
            (alias: "ls")
            (about: "List installed and released node versions")
            (@arg installed: -i --installed "Only display installed versions")
            (@arg online: -o --online --available "Only display available versions")
            (@arg filter: {node_version::is_version_range} "Filter by semantic versions. e.g. `12`, `^10.9`, `>=8.10`, `>=8, <9`")
        )
        (@subcommand install =>
            (alias: "i")
            (about: "Install a new node version")
            (@arg force: -f --force "Install version even if it's already installed")
            (@arg version: +required {node_version::is_version_range} "A semver range. The latest version matching this range will be installed.")
        )
        (@subcommand uninstall =>
            (alias: "u")
            (alias: "r")
            (about: "Uninstall an installed node version")
            (@arg force: -f --force "Skip prompt if uninstalling selected version.")
            (@arg version: +required {node_version::is_version_range} "A semver range. The latest installed version matching this range will be removed.")
        )
        (@subcommand use =>
            (alias: "switch")
            (alias: "u")
            (about: "Switch to an installed node version")
            (@arg version: {node_version::is_version_range} "A semver range. The latest version matching this range will be switched to.\nRespects `.nvmrc` files.")
        )
        (@subcommand parse_version =>
            (alias: "parse-version")
            (alias: "pv")
            (about: "Echo what a version string will be parsed to.")
            (@arg version: {node_version::is_version_range} "The semver range to echo the parsed result of.")
        )
    );

    let config = Config::from_env_and_args(app.get_arguments());
    let matches = app.get_matches();

    if matches.is_present("verbose") {
        println!("{:#?}\n", config);
    }

    let result = match matches.subcommand_name() {
        Some("list") => List::run(&config, matches.subcommand_matches("list").unwrap()),
        Some("install") => Install::run(&config, matches.subcommand_matches("install").unwrap()),
        Some("uninstall") => {
            Uninstall::run(&config, matches.subcommand_matches("uninstall").unwrap())
        },
        Some("use") => Switch::run(&config, matches.subcommand_matches("use").unwrap()),
        Some("parse_version") => ParseVersion::run(
            &config,
            matches.subcommand_matches("parse_version").unwrap(),
        ),
        _ => Result::Ok(()),
    };

    result
}
