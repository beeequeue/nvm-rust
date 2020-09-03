mod common;

mod uninstall {
    use anyhow::Result;
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{
        common,
        common::{assert_outputs, assert_version_installed, assert_version_selected},
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
        setup_versions(vec!["14.5.0", version_str])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg("12").assert();

        assert_outputs(&result, "Uninstalled 12.18.3!", "")?;
        assert_version_installed(version_str, false)?;
        assert_version_selected(version_str, false)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn can_uninstall_version_matching_exact_version() -> Result<()> {
        let version_str = "12.18.3";
        setup_versions(vec!["14.5.0", version_str])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg(version_str).assert();

        assert_outputs(&result, "Uninstalled 12.18.3!", "")?;
        assert_version_installed(version_str, false)?;
        assert_version_selected(version_str, false)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn prompts_when_uninstalling_selected_version() -> Result<()> {
        let version_str = "12.18.3";
        setup_versions(vec![version_str])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("uninstall").arg(version_str).assert();
        assert_outputs(
            &result,
            "12.18.3 is currently selected.\nAre you sure you want to uninstall it? (y/N)",
            "",
        )?;

        cmd.write_stdin("y\n");

        let result = cmd.assert();
        assert_outputs(
            &result,
            "12.18.3 is currently selected.\nAre you sure you want to uninstall it? (y/N)\nUninstalled 12.18.3!",
            "",
        )?;

        assert_version_installed(version_str, false)?;
        assert_version_selected(version_str, false)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn force_skips_prompt() -> Result<()> {
        let version_str = "12.18.3";
        setup_versions(vec![version_str])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("uninstall").arg(version_str).arg("-f").assert();
        assert_outputs(
            &result,
            "12.18.3 is currently selected.\nUninstalled 12.18.3!",
            "",
        )?;

        assert_version_installed(version_str, false)?;
        assert_version_selected(version_str, false)?;

        Result::Ok(())
    }

    #[test]
    #[serial]
    fn exits_gracefully_if_no_version_is_found() -> Result<()> {
        setup_versions(vec!["14.5.0"])?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
        let result = cmd.arg("uninstall").arg("12").assert();

        assert_outputs(
            &result,
            "",
            "Error: Did not find an installed version matching ^12",
        )?;

        Result::Ok(())
    }
}
