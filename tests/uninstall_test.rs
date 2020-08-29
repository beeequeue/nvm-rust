mod common;

mod uninstall {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{
        common,
        common::{assert_outputs, assert_version_not_installed},
    };

    fn setup_versions(versions: Vec<&str>) -> Result<()> {
        common::setup_integration_test()?;

        versions.to_owned().into_iter().for_each(|version_str| {
            common::install_mock_version(version_str).expect("Mock version");
        });

        common::create_shim(versions.get(0).unwrap()).expect("Select mock version");

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_uninstall_version_matching_range() -> Result<()> {
        let version_str = "12.18.3";
        setup_versions(vec![version_str, "14.5.0"])?;

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
        setup_versions(vec![version_str, "14.5.0"])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg(version_str).assert();

        assert_outputs(&result, "Uninstalled 12.18.3!", "")?;
        assert_version_not_installed(version_str)?;

        Result::Ok(())
    }
}
