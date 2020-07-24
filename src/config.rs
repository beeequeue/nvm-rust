pub struct Config {
    dir: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Config {
            dir: option_env!("NVM_DIR").unwrap_or(Self::get_default_dir()),
        }
    }

    fn get_default_dir() -> &'static str {
        if cfg!(windows) {
            "C:\\Program Data\\nvm"
        } else {
            "$HOME/.nvm"
        }
    }
}
