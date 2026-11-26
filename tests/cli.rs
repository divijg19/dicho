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
fn random_does_not_repeat_todays_entry() {
    let daily = dicho().output().expect("command should run");
    let random = dicho().arg("random").output().expect("command should run");
    assert!(random.status.success());
    assert_ne!(spanish_line(&daily.stdout), spanish_line(&random.stdout));
}

fn spanish_line(output: &[u8]) -> String {
    String::from_utf8_lossy(output)
        .lines()
        .find(|line| line.starts_with("  ") && !line.contains(':'))
        .unwrap_or_default()
        .to_owned()
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
        .stdout(predicate::str::contains("dicho 0.2.0"));
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

#[test]
fn browse_lists_all_entries() {
    let output = dicho().arg("browse").output().expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 40);
    assert!(text.contains("refran-001  "));
    assert!(text.contains("refran-040  "));
}

#[test]
fn show_finds_a_known_entry() {
    dicho()
        .args(["show", "refran-021"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Entry: refran-021"))
        .stdout(predicate::str::contains("A buen hambre no hay pan duro."))
        .stdout(predicate::str::contains("Difficulty: 0.25"));
}

#[test]
fn show_rejects_an_unknown_id() {
    dicho().args(["show", "missing"]).assert().failure();
}

#[test]
fn categories_and_tags_show_counts() {
    dicho()
        .arg("categories")
        .assert()
        .success()
        .stdout(predicate::str::contains("proverb"))
        .stdout(predicate::str::contains("saying"));
    dicho()
        .arg("tags")
        .assert()
        .success()
        .stdout(predicate::str::contains("optimism  2"));
}
