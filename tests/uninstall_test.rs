mod utils;

mod uninstall {
    use anyhow::Result;
    use std::path::Path;

    use crate::utils;

    fn setup_versions(temp_dir: &Path, versions: Vec<&str>) -> Result<()> {
        for version_str in versions.to_owned().into_iter() {
            utils::install_mock_version(temp_dir, version_str)?;
        }

        utils::create_shim(temp_dir, versions.get(0).unwrap())
    }

    #[test]
    fn can_uninstall_version_matching_range() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "12.18.3";
        setup_versions(&temp_dir, vec!["14.5.0", version_str])?;

        let result = cmd.arg("uninstall").arg("12").assert();

        utils::assert_outputs_contain(&result, "Uninstalled 12.18.3!", "")?;
        utils::assert_version_installed(&temp_dir, version_str, false)?;
        assert_eq!(
            utils::get_selected_version(&temp_dir),
            Some("14.5.0".to_string())
        );

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn can_uninstall_version_matching_exact_version() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "12.18.3";
        setup_versions(&temp_dir, vec!["14.5.0", version_str])?;

        let result = cmd.arg("uninstall").arg(version_str).assert();

        utils::assert_outputs_contain(&result, "Uninstalled 12.18.3!", "")?;
        utils::assert_version_installed(&temp_dir, version_str, false)?;
        assert_eq!(
            utils::get_selected_version(&temp_dir),
            Some("14.5.0".to_string())
        );

        temp_dir.close().map_err(anyhow::Error::from)
    }

    // #[test]
    // #[serial]
    // fn prompts_when_uninstalling_selected_version() -> Result<()> {
    //     let version_str = "12.18.3";
    //     setup_versions(vec![version_str])?;
    //
    //     let mut cmd = Command::cargo_bin("nvm-rust").unwrap();
    //
    //     let result = cmd.arg("uninstall").arg(version_str).assert();
    //     assert_outputs_contain(
    //         &result,
    //         "12.18.3 is currently selected.\nAre you sure you want to uninstall it? (y/N)",
    //         "",
    //     )?;
    //
    //     cmd.write_stdin("y\n");
    //
    //     let result = cmd.assert();
    //     assert_outputs_contain(
    //         &result,
    //         "12.18.3 is currently selected.\nAre you sure you want to uninstall it? (y/N)\nUninstalled 12.18.3!",
    //         "",
    //     )?;
    //
    //     assert_version_installed(version_str, false)?;
    //     assert_version_selected(version_str, false)?;
    //
    //     temp_dir.close().map_err(anyhow::Error::from)
    // }

    #[test]
    fn force_skips_prompt() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "12.18.3";
        setup_versions(&temp_dir, vec![version_str])?;

        let result = cmd
            .arg("uninstall")
            .arg(version_str)
            .arg("--force")
            .assert();

        utils::assert_outputs_contain(
            &result,
            "12.18.3 is currently selected.\nUninstalled 12.18.3!",
            "",
        )?;

        utils::assert_version_installed(&temp_dir, version_str, false)?;
        assert_eq!(utils::get_selected_version(&temp_dir), None);

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn exits_gracefully_if_no_version_is_found() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        setup_versions(&temp_dir, vec!["14.5.0"])?;

        let result = cmd.arg("uninstall").arg("12").assert();

        utils::assert_outputs_contain(&result, "", "Error: >=12.0.0 <13.0.0-0 is not installed.")?;

        temp_dir.close().map_err(anyhow::Error::from)
    }
}
