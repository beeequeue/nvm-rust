mod utils;

mod switch {
    use anyhow::Result;

    use crate::utils;

    #[test]
    fn can_switch_version_with_no_previous_one() -> Result<()> {
        let (temp_dir, mut cmd) = utils::setup_integration_test()?;

        let version_str = "12.18.3";
        utils::install_mock_version(&temp_dir, version_str)?;
        let result = cmd.arg("pv").assert();

        let output = String::from_utf8(result.get_output().to_owned().stdout)?;
        let err = String::from_utf8(result.get_output().to_owned().stderr)?;

        println!("{}", output.trim());
        println!("{}", err.trim());

        temp_dir.close().map_err(anyhow::Error::from)
    }
}
