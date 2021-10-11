#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use assert_cmd::assert::Assert;

pub fn integration_dir() -> PathBuf {
    let path = PathBuf::from("./integration");

    ensure_dir_exists(&path).expect("integration dir exists");

    fs::canonicalize(path).expect("canonicalize integration dir path")
}

// TODO: Rework unix shims
#[cfg(unix)]
pub fn required_files<'a>() -> [&'a str; 3] {
    ["bin/node", "bin/npm", "bin/npx"]
}

#[cfg(windows)]
pub fn required_files<'a>() -> [&'a str; 5] {
    ["node.exe", "npm", "npm.cmd", "npx", "npx.cmd"]
}

fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).context(format!("Could not create {:?}", path))?
    }

    Result::Ok(())
}

pub fn setup_integration_test() -> Result<()> {
    env::set_var("NVM_DIR", integration_dir());

    let path = integration_dir();

    for entry in fs::read_dir(path.to_owned())? {
        let name = entry?.file_name();
        let entry_path = path.join(name.to_owned());

        if entry_path.is_dir() || name == "shims" {
            fs::remove_dir_all(entry_path)?
        } else {
            fs::remove_file(entry_path)?
        }
    }

    Result::Ok(())
}

pub fn install_mock_version(version_str: &str) -> Result<()> {
    let to_dir = integration_dir().join("versions").join(version_str);
    fs::create_dir_all(to_dir.to_owned())?;

    for file_name in required_files() {
        let file_path = to_dir.join(file_name);
        fs::write(&file_path, version_str)
            .unwrap_or_else(|_| panic!("Failed to write to {:#?}", &file_path))
    }

    Result::Ok(())
}

#[allow(dead_code)]
#[cfg(windows)]
pub fn create_shim(version_str: &str) -> Result<()> {
    symlink_dir(
        integration_dir().join(version_str),
        integration_dir().join("shims"),
    )
    .map_err(anyhow::Error::from)
}

#[cfg(unix)]
pub fn create_shim(version_str: &str) -> Result<()> {
    symlink(
        integration_dir().join(version_str),
        integration_dir().join("shims"),
    )
    .map_err(anyhow::Error::from)
}

#[derive(PartialEq, Eq)]
struct OutputResult(bool, bool);

pub fn assert_outputs_contain(result: &Assert, stdout: &str, stderr: &str) -> Result<()> {
    let output = result.get_output().to_owned();
    let output_stderr = String::from_utf8(output.stderr)?;
    let output_stdout = String::from_utf8(output.stdout)?;
    let result = OutputResult(
        output_stdout.trim().contains(stdout),
        output_stderr.trim().contains(stderr),
    );

    if result != OutputResult(true, true) {
        panic!(
            r#"Got incorrect command output:
stdout expected:
"{}"
stdout output:
"{}"

stderr expected:
"{}"
stderr output:
"{}"
"#,
            stdout,
            output_stdout.trim(),
            stderr,
            output_stderr.trim()
        )
    }

    Result::Ok(())
}

pub fn assert_version_installed(version_str: &str, installed: bool) -> Result<()> {
    let path = integration_dir();

    for filename in required_files().iter() {
        let file_path = path.join("versions").join(version_str).join(filename);

        assert_eq!(
            file_path.exists(),
            installed,
            "{:#?} does{}exist",
            file_path,
            if !installed { " " } else { " not " }
        );
    }

    Result::Ok(())
}

pub fn assert_version_selected(version_str: &str, selected: bool) -> Result<()> {
    let path = integration_dir().join("shims");

    // symlink_metadata errors if the path doesn't exist
    if fs::symlink_metadata(&path).is_ok() {
        let real_path = fs::read_link(path).unwrap();

        assert_eq!(
            real_path.to_str().unwrap().contains(version_str),
            selected,
            "{} is{}selected (Expected it{}to be).",
            version_str,
            if selected { " not " } else { " " },
            if !selected { " not " } else { " " },
        );
    } else if selected {
        panic!(
            "{} should have been selected but no version is.",
            version_str
        )
    }

    Result::Ok(())
}
