mod common;

mod install {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{
        common,
        common::{assert_outputs_contain, assert_version_installed, install_mock_version},
    };

    #[test]
    #[serial]
    fn can_install_version_matching_range() -> Result<()> {
        common::setup_integration_test()?;
        let version_range = ">=12, <12.8";
        let version_str = "12.7.0";

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("install").arg("--force").arg(version_range).assert();

        assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v12.7.0/node-v12.7.0-",
            "",
        )?;
        assert_version_installed(version_str, true)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_install_version_matching_exact_version() -> Result<()> {
        common::setup_integration_test()?;
        let version_str = "12.18.3";

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("install").arg("--force").arg(version_str).assert();

        assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v12.18.3/node-v12.18.3-",
            "",
        )?;
        assert_version_installed(version_str, true)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn stops_when_installing_installed_version() -> Result<()> {
        let version_str = "12.18.3";
        install_mock_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("install").arg(version_str).assert();
        assert_outputs_contain(&result, "12.18.3 is already installed - skipping...", "")?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn force_forces_install_of_installed_version() -> Result<()> {
        let version_str = "12.18.3";
        install_mock_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("install").arg("--force").arg(version_str).assert();
        assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v12.18.3/node-v12.18.3-",
            "",
        )?;
        assert_outputs_contain(&result, "Extracting...", "")?;

        assert_version_installed(version_str, true)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn exits_gracefully_if_no_version_is_found() -> Result<()> {
        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("install").arg("--force").arg("12.99.99").assert();

        assert_outputs_contain(
            &result,
            "",
            "Error: Did not find a version matching `12.99.99`!",
        )?;

        Result::Ok(())
    }
}
