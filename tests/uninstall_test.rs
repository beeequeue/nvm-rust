mod common;

mod uninstall {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{
        common,
        common::{assert_outputs, assert_version_not_installed},
    };

    fn setup_installed_version(version_str: &str) -> Result<()> {
        common::setup_integration_test()?;
        common::install_mock_version(version_str)?;
        common::create_shim(version_str)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_uninstall_version_matching_range() -> Result<()> {
        let version_str = "12.18.3";
        setup_installed_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg("12").assert();

        assert_outputs(&result, "Uninstalled 12.18.3!", "")?;
        assert_version_not_installed(version_str)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_uninstall_version_matching_exact_version() -> Result<()> {
        let version_str = "12.18.3";
        setup_installed_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg("12.18.3").assert();

        assert_outputs(&result, "Uninstalled 12.18.3!", "")?;
        assert_version_not_installed(version_str)?;

        Result::Ok(())
    }
}
