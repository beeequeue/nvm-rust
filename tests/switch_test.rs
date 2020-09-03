mod common;

mod switch {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{
        common,
        common::{assert_outputs, assert_version_selected},
    };

    #[test]
    #[serial]
    fn can_switch_version_with_no_previous_one() -> Result<()> {
        let version_str = "12.18.3";
        common::setup_integration_test()?;
        common::install_mock_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("use").arg("12").assert();

        let output = String::from_utf8(result.get_output().to_owned().stdout)?;
        let output = output.trim();

        assert_version_selected(version_str, true)?;
        assert_eq!(output, "Switched to 12.18.3");

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_switch_version_with_previous_version() -> Result<()> {
        let old_version = "12.18.3";
        let new_version = "14.5.0";

        common::setup_integration_test()?;
        common::install_mock_version(old_version)?;
        common::install_mock_version(new_version)?;
        common::create_shim(old_version)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("use").arg("14").assert();

        assert_version_selected(new_version, true)?;
        assert_outputs(&result, "Switched to 14.5.0", "")?;

        Result::Ok(())
    }
}
