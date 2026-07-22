//! CLI integration tests for the standalone validator gate (A5b).
//!
//! These assert the exact exit-code contract from the Go tool's
//! `cmd/exitcode.go` (`0` clean, `1` error, `2` warning, `3` CLI usage error;
//! `--strict` promotes warnings to `1`) and the hook-parity invocations
//! (`validate structure --allow-dirs=evals` and `check --per-file`) against the
//! frozen golden-corpus fixtures. The binary is located via the
//! `CARGO_BIN_EXE_<name>` variable Cargo sets for integration tests.

use std::path::PathBuf;
use std::process::{Command, Output};

/// Absolute path to a golden-corpus testdata fixture directory.
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden-corpus/fixtures/testdata")
        .join(name)
}

/// Run the built binary with `args` and return its output.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_skill-validator-rs"))
        .args(args)
        .output()
        .expect("spawn skill-validator-rs")
}

/// The process exit code (tests never trigger signals).
fn code(output: &Output) -> i32 {
    output.status.code().expect("exit code")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn clean_skill_exits_zero() {
    let dir = fixture("valid-skill");
    let out = run(&["validate", "structure", dir.to_str().unwrap()]);
    assert_eq!(code(&out), 0, "stdout: {}", stdout(&out));
}

#[test]
fn error_skill_exits_one() {
    let dir = fixture("invalid-skill");
    let out = run(&["validate", "structure", dir.to_str().unwrap()]);
    assert_eq!(code(&out), 1, "stdout: {}", stdout(&out));
}

#[test]
fn warning_only_skill_exits_two() {
    let dir = fixture("warnings-only-skill");
    let out = run(&["validate", "structure", dir.to_str().unwrap()]);
    assert_eq!(code(&out), 2, "stdout: {}", stdout(&out));
}

#[test]
fn warning_only_skill_exits_one_under_strict() {
    let dir = fixture("warnings-only-skill");
    let out = run(&["validate", "structure", "--strict", dir.to_str().unwrap()]);
    assert_eq!(code(&out), 1, "stdout: {}", stdout(&out));
}

#[test]
fn cli_misuse_exits_three() {
    // Missing the required <dir> positional.
    assert_eq!(code(&run(&["validate", "structure"])), 3);
    // Unknown flag.
    let dir = fixture("valid-skill");
    assert_eq!(
        code(&run(&[
            "validate",
            "structure",
            "--no-such-flag",
            dir.to_str().unwrap()
        ])),
        3
    );
    // Unknown subcommand.
    assert_eq!(code(&run(&["totally-bogus"])), 3);
}

#[test]
fn help_and_version_exit_zero() {
    assert_eq!(code(&run(&["--help"])), 0);
    assert_eq!(code(&run(&["--version"])), 0);
}

#[test]
fn validate_structure_allow_dirs_suppresses_the_named_dir() {
    // The pre-commit hook parity invocation.
    let dir = fixture("allowed-dirs-skill");
    let path = dir.to_str().unwrap();

    let without = run(&["validate", "structure", path]);
    assert!(
        stdout(&without).contains("evals/"),
        "expected an unknown-dir warning for evals/ without the flag; stdout: {}",
        stdout(&without)
    );

    let with = run(&["validate", "structure", "--allow-dirs=evals", path]);
    assert!(
        !stdout(&with).contains("unknown directory: evals/"),
        "--allow-dirs=evals should suppress the evals/ warning; stdout: {}",
        stdout(&with)
    );
    // The fixture still has an unrecognised `testing/` directory, so the run is
    // still gated on a warning (exit 2), proving the flag is scoped to evals.
    assert_eq!(code(&with), 2, "stdout: {}", stdout(&with));
}

#[test]
fn check_per_file_runs_all_groups_on_a_clean_skill() {
    // The pre-push hook parity invocation.
    let dir = fixture("valid-skill");
    let out = run(&[
        "check",
        "--allow-dirs=evals",
        "--per-file",
        "-o",
        "json",
        dir.to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0, "stdout: {}", stdout(&out));

    let parsed: serde_json::Value =
        serde_json::from_str(&stdout(&out)).expect("check --per-file emits valid JSON");
    assert!(parsed.get("content").is_some(), "content group ran");
    assert!(
        parsed.get("contamination").is_some(),
        "contamination group ran"
    );
    assert!(
        parsed
            .get("reference_reports")
            .and_then(|v| v.as_array())
            .is_some_and(|a| !a.is_empty()),
        "per-file reference analysis present: {}",
        stdout(&out)
    );
}

#[test]
fn json_output_is_stable_and_parseable() {
    let dir = fixture("invalid-skill");
    let out = run(&["validate", "structure", "-o", "json", dir.to_str().unwrap()]);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout(&out)).expect("valid JSON on stdout");
    assert!(parsed.get("errors").and_then(|v| v.as_u64()).unwrap() > 0);
    assert!(parsed.get("results").and_then(|v| v.as_array()).is_some());
}

#[test]
fn annotations_do_not_pollute_json_stdout() {
    let dir = fixture("invalid-skill");
    let out = run(&[
        "validate",
        "structure",
        "-o",
        "json",
        "--emit-annotations",
        dir.to_str().unwrap(),
    ]);
    // Annotations go to stderr under JSON so stdout stays machine-parseable.
    serde_json::from_str::<serde_json::Value>(&stdout(&out))
        .expect("stdout is pure JSON even with --emit-annotations");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("::error"),
        "annotations emitted to stderr: {err}"
    );
}
