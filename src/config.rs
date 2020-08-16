use std::{fs::create_dir_all, path::PathBuf};

use clap::Arg;

#[derive(Copy, Clone)]
pub struct Config {
    dir: Option<&'static str>,
}

impl Config {
    pub fn from_env_and_args(_args: &[Arg]) -> Self {
        Config {
            dir: option_env!("NVM_DIR"),
        }
    }

    pub fn dir(self) -> PathBuf {
        let path = PathBuf::from(self.dir.unwrap_or(Self::get_default_dir()));

        if !path.exists() {
            create_dir_all(path.clone())
                .unwrap_or_else(|err| panic!("Could not create {:?} - {}", path, err));
        }

        if !path.is_dir() {
            panic!("{:?} is not a directory! Please rename it!", path)
        }

        path
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
