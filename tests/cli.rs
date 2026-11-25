use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn root_command_prints_foundation_message() {
    Command::cargo_bin("dicho")
        .expect("binary should build")
        .assert()
        .success()
        .stdout(predicate::str::contains("dicho"))
        .stdout(predicate::str::contains("v0.0.2"));
}

#[test]
fn version_flag_reports_package_version() {
    Command::cargo_bin("dicho")
        .expect("binary should build")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("dicho 0.0.2"));
}

#[test]
fn help_flag_describes_the_project() {
    Command::cargo_bin("dicho")
        .expect("binary should build")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Spanish language companion"));
}

#[test]
fn unknown_flag_is_rejected() {
    Command::cargo_bin("dicho")
        .expect("binary should build")
        .arg("--bogus")
        .assert()
        .failure();
}
