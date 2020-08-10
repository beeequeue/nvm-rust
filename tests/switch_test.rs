use assert_cmd::Command;
use serial_test::serial;

mod common;

#[test]
#[serial]
fn switches_version() {
    common::setup_version("12");

    let mut cmd = Command::cargo_bin("nvm-rust").unwrap();

    let result = cmd
        .env("NVM_DIR", "C:\\dev\\test")
        .arg("-V")
        .arg("use")
        .arg("12")
        .assert();

    let output = result.get_output();

    println!("{:?}", String::from_utf8(output.stdout.clone()).unwrap());
}
