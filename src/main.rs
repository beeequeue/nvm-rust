#![feature(const_fn)]

use clap::{clap_app, crate_version};

use config::Config;
use subcommand::ls::Ls;
use subcommand::Subcommand;

mod config;
mod node_version;
mod subcommand;

static CONFIG: Config = Config::new();

fn main() {
    let matches = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@subcommand ls =>
            (alias: "list")
            (about: "List installed and released node versions")
            (@arg FILTER: {Ls::validate_filter} "Filter by semantic versions. e.g. `12`, `LTS`, `^10.9`, `>=8.10`")
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
