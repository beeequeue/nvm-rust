use std::{
    borrow::Borrow,
    env,
    fs::{canonicalize, create_dir_all},
    path::PathBuf,
};

use clap::Arg;

#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
    pub shims_dir: PathBuf,
}

impl Config {
    pub fn from_env_and_args(_args: &[Arg]) -> Self {
        let dir = env::var("NVM_DIR").ok();
        let dir = PathBuf::from(dir.unwrap_or_else(|| Self::get_default_dir().to_string()));
        let dir = canonicalize(dir).expect("Could not resolve nvm dir path");

        Self::ensure_dir_exists(dir.borrow());

        Config {
            shims_dir: dir.join("shims"),
            dir,
        }
    }

    fn ensure_dir_exists(path: &PathBuf) {
        if !path.exists() {
            create_dir_all(path.clone())
                .unwrap_or_else(|err| panic!("Could not create {:?} - {}", path, err));
        }

        if !path.is_dir() {
            panic!("{:?} is not a directory! Please rename it.", path)
        }
    }

    #[cfg(windows)]
    fn get_default_dir() -> &'static str {
        if cfg!(target_arch = "x86") {
            return "C:\\Program Files (x86)\\nvm";
        }

        "C:\\Program Files\\nvm"
    }

    #[cfg(unix)]
    fn get_default_dir() -> &'static str {
        "$HOME/.nvm"
    }
}
