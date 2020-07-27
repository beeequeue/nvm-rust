pub struct Config {
    pub dir: Option<&'static str>,
}

impl Config {
    pub const fn new() -> Self {
        Config {
            dir: option_env!("NVM_DIR"),
        }
    }

    pub fn dir(self) -> &'static str {
        self.dir.unwrap_or(Self::get_default_dir())
    }

    fn get_default_dir() -> &'static str {
        if cfg!(windows) {
            if cfg!(target_arch = "x86") {
                return "C:\\Program Files (x86)\\nvm";
            }

            return "C:\\Program Files\\nvm";
        }

        "$HOME/.nvm"
    }
}
