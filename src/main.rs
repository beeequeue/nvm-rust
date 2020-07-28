#![feature(const_fn)]

use clap::{clap_app, crate_version};

use config::Config;
use std::num::ParseIntError;
use subcommand::ls::Ls;

mod config;
mod node_version;
mod subcommand;

static CONFIG: Config = Config::new();

fn validate_number(value: &str) -> Result<i32, String> {
    value.parse().map_err(|err: ParseIntError| err.to_string())
}

fn main() {
    let matches = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@subcommand ls =>
            (alias: "list")
            (about: "List installed and released node versions")
            (@arg installed: -i --installed "Only display installed versions")
            (@arg online: -o --online --available "Only display available versions")
            (@arg filter: {Ls::validate_filter} "Filter by semantic versions. e.g. `12`, `^10.9`, `>=8.10`, `>=8, <9`")
        )
    ).get_matches();

    let result = match matches.subcommand_name() {
        Some("ls") => Ls::init().run(matches.subcommand_matches("ls").unwrap()),
        _ => Result::Ok(()),
    };

    println!(
        "{}",
        if result.is_err() {
            result.unwrap_err()
        } else {
            String::from("OK")
        }
    );
}
