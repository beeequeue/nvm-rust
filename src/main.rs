mod config;
mod subcommand;

use clap::{clap_app, crate_version};
use config::Config;
use subcommand::ls::Ls;
use subcommand::Subcommand;

fn main() {
    let config = Config::new();

    let matches = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@subcommand ls =>
            (alias: "list")
            (about: "List installed and released node versions")
            (@arg FILTER: {Ls::validate_filter} "Filter by semantic versions. e.g. `12`, `LTS`, `^10.9`, `>=8.10`")
        )
    ).get_matches();

    match matches.subcommand_name() {
        Some("ls") => Ls::run(matches, config),
        _ => {}
    }

    // println!("{:?}", matches);
}
