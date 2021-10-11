#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;
use std::{fs, path::Path};

use anyhow::Result;
use assert_cmd::{assert::Assert, Command};
use assert_fs::TempDir;

pub fn integration_dir() -> TempDir {
    let dir = TempDir::new().expect("Could not create temp dir");

    println!("{:#?}", dir.path());
    dir
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

pub fn setup_integration_test() -> Result<(TempDir, Command)> {
    let temp_dir = integration_dir();

    let mut cmd = Command::cargo_bin("nvm-rust").expect("Could not create Command");
    cmd.args(&["--dir", &temp_dir.to_string_lossy()]);

    Result::Ok((temp_dir, cmd))
}

pub fn install_mock_version(path: &Path, version_str: &str) -> Result<()> {
    let to_dir = path.join("versions").join(version_str);
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
