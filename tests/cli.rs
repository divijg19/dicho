use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn dicho() -> Command {
    Command::cargo_bin("dicho").expect("binary should build")
}

fn temp_data_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("dicho-cli-{name}-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("temp directory should be created");
    dir
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
        .stdout(predicate::str::contains("dicho 0.3.2"));
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
fn search_matches_accented_text_from_an_unaccented_query() {
    dicho()
        .args(["search", "abarca"])
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-010"));
}

#[test]
fn search_matches_translation_and_source() {
    dicho()
        .args(["search", "cobbler"])
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-035"));
    dicho()
        .args(["search", "sbarbi"])
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-021"));
}

#[test]
fn whole_word_search_excludes_substrings_unless_requested() {
    dicho()
        .args(["search", "clav"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
    dicho()
        .args(["search", "clav", "--substring"])
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-028"));
}

#[test]
fn category_filter_ignores_case() {
    let output = dicho()
        .args(["browse", "--category", "SAYING"])
        .output()
        .expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 5);
    assert!(text.lines().all(|line| line.contains("[saying]")));
}

#[test]
fn search_with_no_matches_is_empty() {
    let output = dicho()
        .args(["search", "notpresent"])
        .output()
        .expect("command should run");
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
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

#[test]
fn browse_filters_by_category_and_tag() {
    let output = dicho()
        .args(["browse", "--category", "saying"])
        .output()
        .expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 5);
    assert!(text.lines().all(|line| line.contains("[saying]")));

    let output = dicho()
        .args(["browse", "--tag", "optimism"])
        .output()
        .expect("command should run");
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 2);
}

#[test]
fn browse_filters_by_difficulty() {
    let output = dicho()
        .args(["browse", "--min-difficulty", "0.5"])
        .output()
        .expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 1);
    assert!(text.contains("refran-039"));
}

#[test]
fn invalid_filter_is_rejected() {
    dicho()
        .args(["browse", "--category", "unknown"])
        .assert()
        .failure();
    dicho()
        .args([
            "browse",
            "--min-difficulty",
            "0.9",
            "--max-difficulty",
            "0.1",
        ])
        .assert()
        .failure();
}

#[test]
fn search_accepts_filters() {
    dicho()
        .args(["search", "abarca", "--category", "proverb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-010"));
    dicho()
        .args(["search", "abarca", "--category", "saying"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn saved_entries_round_trip() {
    let dir = temp_data_dir("saved");
    let data = dir.join("saved.json");

    assert!(!data.exists());
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["save", "refran-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Saved refran-001"));
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .arg("saved")
        .assert()
        .success()
        .stdout(predicate::str::contains("refran-001"));
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["unsave", "refran-001"])
        .assert()
        .success();
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .arg("saved")
        .assert()
        .success()
        .stdout(predicate::str::contains("No saved entries."));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn history_records_daily_once_per_date() {
    let dir = temp_data_dir("history-daily");
    for _ in 0..2 {
        dicho().env("DICHO_DATA_DIR", &dir).assert().success();
    }
    let output = dicho()
        .env("DICHO_DATA_DIR", &dir)
        .arg("history")
        .output()
        .expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 1);
    assert!(text.contains("  daily  "));
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn history_records_save_and_unsave() {
    let dir = temp_data_dir("history-save");
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["save", "refran-001"])
        .assert()
        .success();
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["unsave", "refran-001"])
        .assert()
        .success();
    let output = dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["history", "--limit", "5"])
        .output()
        .expect("command should run");
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.lines().count(), 2);
    assert!(text.contains("  save  refran-001"));
    assert!(text.contains("  unsave  refran-001"));
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn history_is_empty_when_nothing_was_recorded() {
    let dir = temp_data_dir("history-empty");
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .arg("history")
        .assert()
        .success()
        .stdout(predicate::str::contains("No history yet."));
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn cannot_save_an_unknown_id() {
    let dir = temp_data_dir("unknown");
    dicho()
        .env("DICHO_DATA_DIR", &dir)
        .args(["save", "missing"])
        .assert()
        .failure();
    fs::remove_dir_all(&dir).ok();
}
