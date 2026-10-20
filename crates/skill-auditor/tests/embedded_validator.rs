//! The auditor embeds the offline validator as `validate`, `check`, `probes`
//! and `analyze`. These tests pin the contract that matters to callers: the
//! validator's exit codes and output pass through unchanged, and its help is
//! reachable under the auditor's name.

use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-skill-auditor");

fn auditor(args: &[&str]) -> Output {
    Command::new(BIN).args(args).output().expect("run auditor")
}

fn code(out: &Output) -> i32 {
    out.status.code().expect("exit code")
}

fn text(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn validate_help_lists_groups_under_the_auditor_name() {
    let out = auditor(&["validate", "--help"]);
    assert_eq!(code(&out), 0);
    let help = text(&out);
    assert!(help.contains("pantheon-skill-auditor validate"), "{help}");
    assert!(help.contains("structure"), "{help}");
    assert!(help.contains("artifacts"), "{help}");
}

#[test]
fn usage_errors_exit_3_like_the_standalone_validator() {
    let out = auditor(&["validate", "no-such-group"]);
    assert_eq!(code(&out), 3);
}

#[test]
fn missing_skill_directory_exits_1() {
    let dir = tempfile::tempdir().expect("tempdir");
    let missing = dir.path().join("absent");
    let out = auditor(&["validate", "structure", missing.to_str().expect("utf8")]);
    assert_eq!(code(&out), 1, "{}", text(&out));
}

#[test]
fn analyze_listing_is_advisory_and_exits_0() {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = auditor(&[
        "analyze",
        "listing",
        dir.path().to_str().expect("utf8"),
        "-o",
        "json",
    ]);
    assert_eq!(code(&out), 0);
    assert!(text(&out).contains("budget_chars"), "{}", text(&out));
}

#[test]
fn probes_validate_runs_through_the_embedded_validator() {
    let dir = tempfile::tempdir().expect("tempdir");
    let out = auditor(&["probes", "validate", dir.path().to_str().expect("utf8")]);
    // An empty probe directory is an error (exit 1) with the validator's own
    // message, not a usage error (exit 3): the subcommand reached it.
    assert_eq!(code(&out), 1);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("no probe files"), "{stderr}");
}

#[test]
fn validator_global_flags_work_before_the_subcommand() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_str().expect("utf8");

    // `-o json` precedes `analyze`, as it can on the standalone validator.
    let out = auditor(&["-o", "json", "analyze", "listing", root]);
    assert_eq!(code(&out), 0, "{}", String::from_utf8_lossy(&out.stderr));
    assert!(text(&out).contains("budget_chars"), "{}", text(&out));

    // `--strict` and an inline-value flag precede `validate`.
    let missing = dir.path().join("absent");
    let missing = missing.to_str().expect("utf8");
    let out = auditor(&[
        "--strict",
        "--allow-dirs=evals",
        "validate",
        "structure",
        missing,
    ]);
    assert_eq!(code(&out), 1, "{}", text(&out));
}

#[test]
fn auditor_commands_and_help_are_not_swallowed_by_the_validator() {
    let out = auditor(&["version"]);
    assert_eq!(code(&out), 0);
    assert!(
        text(&out).contains("pantheon-skill-auditor v"),
        "{}",
        text(&out)
    );

    let out = auditor(&["help", "validate"]);
    assert_eq!(code(&out), 0);
}
