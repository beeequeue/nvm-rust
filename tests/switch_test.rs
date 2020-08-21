mod common;

mod switch {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{common, common::assert_version_installed};

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

        assert_version_installed(version_str)?;
        assert_eq!(output, "Switched to 12.18.3");

        Result::Ok(())
    }
}
