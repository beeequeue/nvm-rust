mod utils;

mod switch {
    use anyhow::Result;

    use crate::utils;

    #[test]
    fn can_switch_version_with_no_previous_one() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "12.18.3";
        utils::install_mock_version(&temp_dir, version_str)?;
        let result = cmd.arg("use").arg("12").assert();

        let output = String::from_utf8(result.get_output().to_owned().stdout)?;
        let output = output.trim();

        assert_eq!(output, "Switched to 12.18.3");
        assert_eq!(
            utils::get_selected_version(&temp_dir),
            Some(version_str.to_string())
        );

        temp_dir.close().map_err(anyhow::Error::from)
    }

    #[test]
    fn can_switch_version_with_previous_version() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;
        let old_version = "12.18.3";
        let new_version = "14.5.0";

        utils::install_mock_version(&temp_dir, old_version)?;
        utils::install_mock_version(&temp_dir, new_version)?;
        utils::create_shim(&temp_dir, old_version)?;

        let result = cmd.arg("use").arg("14").assert();

        assert_eq!(
            utils::get_selected_version(&temp_dir),
            Some(new_version.to_string())
        );
        utils::assert_outputs_contain(&result, "Switched to 14.5.0", "")?;

        temp_dir.close().map_err(anyhow::Error::from)
    }
}
