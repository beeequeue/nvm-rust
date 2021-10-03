use std::path::PathBuf;

#[derive(Debug)]
pub struct OldConfig {
    pub dir: PathBuf,
    pub shims_dir: PathBuf,
}
