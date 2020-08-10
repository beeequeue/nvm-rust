use std::path::PathBuf;

pub fn setup_version(version_stub: &str) {
    let path: PathBuf = ["test-data", "versions", version_stub].iter().collect();

    if !path.exists() {
        panic!(
            "Tried to set up mock version {} which doesn't exist.",
            version_stub
        );
    }

    println!("{:#?}", path);
}
