#[cfg(windows)]
use std::os::windows;
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Parser, ValueHint};

use crate::subcommand::{
    install::InstallCommand, list::ListCommand, parse_version::ParseVersionCommand,
    switch::SwitchCommand, uninstall::UninstallCommand, Action,
};

mod archives;
mod files;
mod node_version;
mod subcommand;

#[derive(Parser, Clone, Debug)]
enum Subcommands {
    List(ListCommand),
    Install(InstallCommand),
    Uninstall(UninstallCommand),
    Use(SwitchCommand),
    ParseVersion(ParseVersionCommand),
}

#[derive(Parser, Debug)]
#[command(
    name = "nvm(-rust)",
    author,
    about,
    about = "Node Version Manager (but better, and in Rust)"
)]
pub struct Config {
    /// Installation directory
    #[clap(global(true), long, value_hint(ValueHint::DirPath), env("NVM_DIR"))]
    dir: Option<PathBuf>,
    /// bin directory
    #[clap(
        global(true),
        long,
        value_hint(ValueHint::DirPath),
        env("NVM_SHIMS_DIR")
    )]
    shims_dir: Option<PathBuf>,
    /// Accept any prompts needed for the command to complete
    #[clap(global(true), short, long)]
    force: bool,

    #[clap(subcommand)]
    command: Subcommands,
}

impl Config {
    pub fn get_dir(&self) -> PathBuf {
        self.dir
            .as_ref()
            .map_or_else(Config::default_dir, |r| r.clone())
    }

    pub fn get_shims_dir(&self) -> PathBuf {
        self.shims_dir
            .as_ref()
            .map_or_else(|| self.get_dir().join("shims"), |r| r.clone())
    }

    /// Path to directory containing node versions
    fn get_versions_dir(&self) -> PathBuf {
        self.get_dir().join("versions")
    }

    fn with_force(&self) -> Self {
        Self {
            force: true,
            dir: Some(self.get_dir()),
            shims_dir: Some(self.get_shims_dir()),
            command: self.command.clone(),
        }
    }

    #[cfg(windows)]
    fn default_dir() -> PathBuf {
        dirs::data_local_dir().unwrap().join("nvm-rust")
    }

    #[cfg(unix)]
    fn default_dir() -> PathBuf {
        dirs::home_dir().unwrap().join("nvm-rust")
    }
}

fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path)
            .unwrap_or_else(|err| panic!("Could not create {:?} - {}", path, err));

        println!("Created nvm dir at {:?}", path);
    }

    if !path.is_dir() {
        panic!("{:?} is not a directory! Please rename it.", path)
    }
}

#[cfg(windows)]
const SYMLINK_ERROR: &str = "You do not seem to have permissions to create symlinks.
This is most likely due to Windows requiring Admin access for it unless you enable Developer Mode.

Either run the program as Administrator or enable Developer Mode:
https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development#active-developer-mode

Read more:
https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10";

#[cfg(windows)]
fn ensure_symlinks_work(config: &Config) -> Result<()> {
    let target_path = &config.get_dir().join("test");

    if windows::fs::symlink_dir(&config.get_shims_dir(), target_path).is_err() {
        bail!("{SYMLINK_ERROR}");
    }

    fs::remove_dir(target_path).expect("Could not remove test symlink...");

    Ok(())
}

fn main() -> Result<()> {
    let config: Config = Config::parse();
    #[cfg(windows)]
    let is_initial_run = !config.get_dir().exists();

    ensure_dir_exists(&config.get_dir());
    ensure_dir_exists(&config.get_versions_dir());

    #[cfg(windows)]
    if is_initial_run {
        let result = ensure_symlinks_work(&config);
        result?;
    }

    match config.command {
        Subcommands::List(ref options) => ListCommand::run(&config, options),
        Subcommands::Install(ref options) => InstallCommand::run(&config, options),
        Subcommands::Uninstall(ref options) => UninstallCommand::run(&config, options),
        Subcommands::Use(ref options) => SwitchCommand::run(&config, options),
        Subcommands::ParseVersion(ref options) => ParseVersionCommand::run(&config, options),
        #[allow(unreachable_patterns)]
        _ => Ok(()),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;

    Config::command().debug_assert()
}
