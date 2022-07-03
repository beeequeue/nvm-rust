#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;
use std::{fs, path::Path};

use anyhow::Result;
use assert_cmd::{assert::Assert, Command};
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;

#[cfg(unix)]
pub fn required_files<'a>() -> [&'a str; 3] {
    ["node", "npm", "npx"]
}

#[cfg(windows)]
pub fn required_files<'a>() -> [&'a str; 5] {
    ["node.exe", "npm", "npm.cmd", "npx", "npx.cmd"]
}

fn integration_dir() -> TempDir {
    let dir = TempDir::new().expect("Could not create temp dir");

    println!("{:#?}", dir.path());
    dir
}

pub fn setup_integration_test() -> Result<(TempDir, Command)> {
    let temp_dir = integration_dir();

    let mut cmd = Command::cargo_bin("nvm-rust").expect("Could not create Command");
    cmd.args(&["--dir", &temp_dir.to_string_lossy()]);

    Result::Ok((temp_dir, cmd))
}

pub fn install_mock_version(path: &Path, version_str: &str) -> Result<()> {
    let mut to_dir = path.join("versions");

    to_dir = to_dir.join(version_str);
    // Unix shims are under `bin/xxx`
    #[cfg(unix)]
    {
        to_dir = to_dir.join("bin");
    }

    fs::create_dir_all(&to_dir)?;

    for file_name in required_files() {
        let file_path = to_dir.join(file_name);

        fs::write(&file_path, version_str)
            .unwrap_or_else(|err| panic!("Failed to write to {:#?}: {}", &file_path, err))
    }

    Result::Ok(())
}

#[allow(dead_code)]
#[cfg(windows)]
pub fn create_shim(temp_dir: &Path, version_str: &str) -> Result<()> {
    symlink_dir(
        temp_dir.join("versions").join(version_str),
        temp_dir.join("shims"),
    )
    .map_err(anyhow::Error::from)
}

#[cfg(unix)]
pub fn create_shim(temp_dir: &Path, version_str: &str) -> Result<()> {
    let mut shims_path = temp_dir.join("versions").join(version_str);

    // Unix shims are under `bin/xxx`
    #[cfg(unix)]
    {
        shims_path = shims_path.join("bin");
    }

    symlink(&shims_path, temp_dir.join("shims")).map_err(anyhow::Error::from)
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

#[allow(dead_code)]
pub fn assert_version_installed(
    temp_dir: &TempDir,
    version_str: &str,
    expect_installed: bool,
) -> Result<()> {
    let versions_dir = temp_dir.child("versions");

    for filename in required_files().iter() {
        let mut file_path = versions_dir.child(version_str);

        // Unix shims are under `bin/xxx`
        #[cfg(unix)]
        {
            file_path = file_path.child("bin");
        }

        file_path = file_path.child(filename);

        if expect_installed {
            file_path.assert(predicates::path::exists());
        } else {
            file_path.assert(predicates::path::exists().not());
        }
    }

    Result::Ok(())
}

#[allow(dead_code)]
pub fn get_selected_version(temp_dir: &TempDir) -> Option<String> {
    let symlink_path = temp_dir.child("shims");

    match fs::read_link(&symlink_path) {
        Result::Ok(shims_dir) => {
            let file_path = shims_dir.join(required_files()[0]);

            Some(fs::read_to_string(&file_path).unwrap())
        },
        Result::Err(_) => None,
    }
}
