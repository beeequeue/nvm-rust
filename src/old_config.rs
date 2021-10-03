use std::{
    borrow::Borrow,
    env,
    fs::{canonicalize, create_dir_all},
    path::PathBuf,
};

#[derive(Debug)]
pub struct OldConfig {
    pub dir: PathBuf,
    pub shims_dir: PathBuf,
}

impl OldConfig {
    pub fn from_env_and_args() -> Self {
        let dir = env::var("NVM_DIR").ok();
        let dir = PathBuf::from(dir.unwrap_or_else(|| Self::get_default_dir().to_string()));

        Self::ensure_dir_exists(dir.borrow());

        let dir = canonicalize(dir).expect("Could not resolve nvm dir path");

        OldConfig {
            shims_dir: dir.join("shims"),
            dir,
        }
    }

    fn ensure_dir_exists(path: &PathBuf) {
        if !path.exists() {
            create_dir_all(path.clone())
                .unwrap_or_else(|err| panic!("Could not create {:?} - {}", path, err));

            println!("Created nvm dir at {:?}", path);
        }

        if !path.is_dir() {
            panic!("{:?} is not a directory! Please rename it.", path)
        }
    }

    #[cfg(windows)]
    fn get_default_dir() -> String {
        if cfg!(target_arch = "x86") {
            return String::from("C:\\Program Files (x86)\\nvm");
        }

        String::from("C:\\Program Files\\nvm")
    }

    #[cfg(unix)]
    fn get_default_dir() -> String {
        format!("{}/.nvm", env::var("HOME").unwrap())
    }
}
