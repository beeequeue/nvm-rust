use anyhow::Result;
use clap::Parser;
use node_semver::Range;

use crate::{
    files,
    node_version::{parse_range, InstalledNodeVersion, NodeVersion},
    subcommand::Action,
    Config,
};

#[derive(Parser, Clone, Debug)]
#[command(
    about = "Check if a version is installed",
    alias = "isi",
    alias = "installed"
)]
pub struct IsInstalledCommand {
    /// A semver range. Will be matched against installed all installed versions.
    #[arg(value_parser = parse_range)]
    pub version: Option<Range>,
    /// Which exit code to use when a version is not installed.
    #[arg(long, short = 'e', default_value = "1")]
    pub exit_code: i32,
    /// Silence output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

impl Action<IsInstalledCommand> for IsInstalledCommand {
    fn run(config: &Config, options: &IsInstalledCommand) -> Result<()> {
        let version_filter = options
            .version
            .clone()
            .or_else(|| files::get_version_file().map(|version_file| version_file.range()));

        if version_filter.is_none() {
            anyhow::bail!("You did not pass a version and we did not find any version files (package.json#engines, .nvmrc) in the current directory.");
        }
        let version_filter = version_filter.unwrap();

        let installed_versions = InstalledNodeVersion::list(config);
        for installed_version in installed_versions {
            if !version_filter.satisfies(installed_version.version()) {
                continue;
            }

            if !options.quiet {
                println!(
                    "✅ A version matching {version_filter} is installed ({})!",
                    installed_version.to_string()
                );
            }
            return Ok(());
        }

        if !options.quiet {
            println!("❌ A version matching {version_filter} is not installed.");
        }

        std::process::exit(options.exit_code)
    }
}
