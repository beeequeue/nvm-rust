use std::{
    env::set_var,
    fs::{canonicalize, copy, create_dir_all, read_dir, remove_file},
    path::PathBuf,
};

use anyhow::{Context, Result};
use std::fs::remove_dir_all;

pub const INTEGRATION_DIR: &str = "./integration";

pub fn integration_dir() -> PathBuf {
    PathBuf::from(INTEGRATION_DIR)
}

pub fn required_files<'a>() -> [&'a str; 4] {
    ["node", "node.cmd", "npm", "npm.cmd"]
}

fn ensure_dir_exists(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        create_dir_all(path).context(format!("Could not create {:?}", path))?
    }

    Result::Ok(())
}

pub fn setup_integration_test() -> Result<()> {
    set_var("NVM_DIR", INTEGRATION_DIR);

    let path = integration_dir();
    ensure_dir_exists(&path)?;

    for entry in read_dir(path.to_owned())? {
        let name = entry?.file_name();
        let entry_path = path.join(name.to_owned());

        if entry_path.is_dir() || name == "shims" {
            remove_dir_all(entry_path)?
        } else {
            remove_file(entry_path)?
        }
    }

    Result::Ok(())
}

pub fn install_mock_version(version_str: &str) -> Result<()> {
    let test_data_dir = PathBuf::from("test-data")
        .join("versions")
        .join(version_str);
    let test_data_dir = canonicalize(test_data_dir).expect("Could not resolve stub version path");

    if !test_data_dir.exists() {
        panic!(
            "Tried to set up mock version {} which doesn't exist.",
            version_str
        );
    }

    let to_dir = PathBuf::from(INTEGRATION_DIR).join(version_str);
    create_dir_all(to_dir.to_owned())?;

    for entry in read_dir(test_data_dir.to_owned())? {
        let name = entry?.file_name();

        copy(test_data_dir.join(name.to_owned()), to_dir.join(name))?;
    }

    Result::Ok(())
}

pub fn assert_version_installed(version_str: &str) -> Result<()> {
    let path = integration_dir();

    for filename in required_files().iter() {
        let file_path = path.join(version_str).join(filename);

        assert!(file_path.exists(), "{:#?} was not created", file_path);
    }

    Result::Ok(())
}
