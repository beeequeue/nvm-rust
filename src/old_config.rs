use std::path::PathBuf;

#[deprecated]
#[derive(Debug)]
pub struct OldConfig {
    pub dir: PathBuf,
    pub shims_dir: PathBuf,
}
