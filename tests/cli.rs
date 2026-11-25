use assert_cmd::Command;
use predicates::prelude::*;

fn dicho() -> Command {
    Command::cargo_bin("dicho").expect("binary should build")
}

#[test]
fn root_command_shows_a_daily_entry() {
    dicho()
        .assert()
        .success()
        .stdout(predicate::str::contains("Daily Spanish"))
        .stdout(predicate::str::contains("Category:"))
        .stdout(predicate::str::contains("Source:"))
        .stdout(predicate::str::contains("Featured vocabulary:"))
        .stdout(predicate::str::contains("Translation status: draft"));
}

#[test]
fn daily_selection_is_stable_across_runs() {
    let first = dicho().output().expect("command should run");
    let second = dicho().output().expect("command should run");
    assert!(first.status.success());
    assert_eq!(first.stdout, second.stdout);
}

#[test]
fn repeat_shows_todays_entry() {
    let daily = dicho().output().expect("command should run");
    let repeat = dicho().arg("repeat").output().expect("command should run");
    assert!(repeat.status.success());
    assert_eq!(daily.stdout, repeat.stdout);
}

#[test]
fn random_command_shows_an_entry() {
    dicho()
        .arg("random")
        .assert()
        .success()
        .stdout(predicate::str::contains("Random Spanish"))
        .stdout(predicate::str::contains("Category:"));
}

#[test]
fn output_is_plain_text() {
    let output = dicho().output().expect("command should run");
    assert!(!output.stdout.contains(&0x1b));
}

#[test]
fn version_flag_reports_package_version() {
    dicho()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("dicho 0.1.0"));
}

#[test]
fn help_flag_describes_the_project() {
    dicho()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Spanish language companion"));
}

#[test]
fn unknown_flag_is_rejected() {
    dicho().arg("--bogus").assert().failure();
}
