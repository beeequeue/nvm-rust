mod common;

mod switch {
    use assert_cmd::Command;
    use serial_test::serial;

    use crate::{common, common::assert_version_installed};

    #[test]
    #[serial]
    fn can_switch_version_with_no_previous_one() -> Result<(), anyhow::Error> {
        let version_str = "12.18.3";
        common::setup_integration_test()?;
        common::install_mock_version(version_str)?;

        let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

        let result = cmd.arg("use").arg("12").assert();

        let output = result.get_output();

        println!(
            "{}",
            String::from_utf8(output.stdout.clone()).unwrap().trim()
        );

        assert_version_installed(version_str)?;

        Result::Ok(())
    }
}
