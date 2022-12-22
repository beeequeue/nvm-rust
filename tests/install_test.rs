mod utils;

mod install {
    use crate::utils;
    use anyhow::Result;

    #[test]
    fn can_install_version_matching_range() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_range = ">=14, <14.21";
        let result = cmd
            .arg("install")
            .arg("--force")
            .arg(version_range)
            .assert();

        utils::assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v14.20.1/node-v14.20.1-",
            "",
        )?;
        utils::assert_version_installed(&temp_dir, "14.20.1", true)?;

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn can_install_version_matching_exact_version() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "14.21.2";
        let result = cmd.arg("install").arg("--force").arg(version_str).assert();

        utils::assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v14.21.2/node-v14.21.2-",
            "",
        )?;
        utils::assert_version_installed(&temp_dir, version_str, true)?;

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn stops_when_installing_installed_version() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "14.21.2";
        utils::install_mock_version(&temp_dir, version_str)?;

        let result = cmd.arg("install").arg(version_str).assert();

        utils::assert_outputs_contain(&result, "14.21.2 is already installed - skipping...", "")?;

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn force_forces_install_of_installed_version() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "14.21.2";
        let result = cmd.arg("install").arg("--force").arg(version_str).assert();

        utils::assert_outputs_contain(
            &result,
            "Downloading from https://nodejs.org/dist/v14.21.2/node-v14.21.2-",
            "",
        )?;
        utils::assert_outputs_contain(&result, "Extracting...", "")?;
        utils::assert_version_installed(&temp_dir, version_str, true)?;

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn exits_gracefully_if_no_version_is_found() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let result = cmd.arg("install").arg("--force").arg("12.99.99").assert();

        utils::assert_outputs_contain(
            &result,
            "",
            "Error: Did not find a version matching `12.99.99`!",
        )?;

        temp_dir.close().map_err(anyhow::Error::from)
    }
}
