mod subcommands;

use clap::{clap_app, crate_version};
use subcommands::ls;

fn main() {
    let matches = clap_app!("nvm(-rust)" =>
        (version: crate_version!())
        (about: "Node Version Manager (but in Rust)")
        (@subcommand ls =>
            (alias: "list")
            (about: "List installed and released node versions")
            (@arg FILTER: {ls::validate_filter} "Filter by semantic versions. e.g. `12`, `LTS`, `^10.9`, `>=8.10`")
        )
    ).get_matches();

    match matches.subcommand_name() {
        Some("ls") => ls::run(matches),
        _ => {}
    }

    // println!("{:?}", matches);
}
