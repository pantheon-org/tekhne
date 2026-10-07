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

/// Write a minimal skill under `root/rel` for the listing-budget tests.
fn write_listing_skill(root: &std::path::Path, rel: &str, frontmatter: &str) {
    let dir = root.join(rel);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("SKILL.md"),
        format!("---\n{frontmatter}---\n\nBody\n"),
    )
    .unwrap();
}

#[test]
fn analyze_listing_is_advisory_and_reports_overflow() {
    let tmp = tempfile::tempdir().unwrap();
    write_listing_skill(
        tmp.path(),
        "dev/alpha",
        "name: alpha\ndescription: Does a thing\n",
    );
    write_listing_skill(
        tmp.path(),
        "dev/hidden",
        "name: hidden\ndescription: x\ndisable-model-invocation: true\n",
    );
    let root = tmp.path().to_str().unwrap();

    let within = run(&["analyze", "listing", root]);
    assert_eq!(code(&within), 0, "stdout: {}", stdout(&within));
    assert!(stdout(&within).contains("1 skills"));
    assert!(stdout(&within).contains("1 excluded"));

    // Over budget is still advisory: exit 0.
    let over = run(&["analyze", "listing", root, "--budget-chars", "5"]);
    assert_eq!(code(&over), 0);
    assert!(stdout(&over).contains("OVER"));

    let json = run(&["analyze", "listing", root, "-o", "json"]);
    assert!(stdout(&json).contains("\"total_chars\""));
}

#[test]
fn analyze_listing_missing_root_is_an_error() {
    let out = run(&["analyze", "listing", "/nonexistent/skills-root"]);
    assert_eq!(code(&out), 1);
}

#[test]
fn analyze_listing_context_window_and_band() {
    let tmp = tempfile::tempdir().unwrap();
    write_listing_skill(
        tmp.path(),
        "dev/alpha",
        "name: alpha\ndescription: Does a thing\n",
    );
    let root = tmp.path().to_str().unwrap();

    // 1000 tokens x 4 chars x 0.01 = 40 chars: the one skill (20 chars) fits.
    let win = run(&["analyze", "listing", root, "--context-tokens", "1000"]);
    assert_eq!(code(&win), 0);
    assert!(
        stdout(&win).contains("of 40 budget (within)"),
        "{}",
        stdout(&win)
    );

    let band = run(&["analyze", "listing", root, "--fraction", "0.01"]);
    assert_eq!(code(&band), 0);
    assert!(stdout(&band).contains("band: 200000 tokens -> 8000 chars"));
    assert!(stdout(&band).contains("band: 1000000 tokens -> 40000 chars"));

    let bad = run(&["analyze", "listing", root, "--fraction", "2"]);
    assert_eq!(code(&bad), 1);
}

#[test]
fn analyze_listing_from_settings_reads_env_and_project_file() {
    let tmp = tempfile::tempdir().unwrap();
    let skills = tmp.path().join("skills");
    write_listing_skill(
        &skills,
        "dev/alpha",
        "name: alpha\ndescription: Does a thing\n",
    );
    let project = tmp.path().join("proj");
    std::fs::create_dir_all(project.join(".claude")).unwrap();
    std::fs::write(
        project.join(".claude/settings.json"),
        r#"{"skillListingBudgetFraction": 0.02}"#,
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_skill-validator-rs");
    let from_file = Command::new(bin)
        .args([
            "analyze",
            "listing",
            skills.to_str().unwrap(),
            "--from-settings",
        ])
        .env("CLAUDE_PROJECT_DIR", &project)
        .env("CLAUDE_CONFIG_DIR", tmp.path().join("nocfg"))
        .env_remove("SLASH_COMMAND_TOOL_CHAR_BUDGET")
        .output()
        .unwrap();
    let text = stdout(&from_file);
    assert!(
        text.contains("band: 200000 tokens -> 16000 chars"),
        "{text}"
    );
    assert!(text.contains("skillListingBudgetFraction=0.02"), "{text}");

    let from_env = Command::new(bin)
        .args([
            "analyze",
            "listing",
            skills.to_str().unwrap(),
            "--from-settings",
        ])
        .env("CLAUDE_PROJECT_DIR", &project)
        .env("CLAUDE_CONFIG_DIR", tmp.path().join("nocfg"))
        .env("SLASH_COMMAND_TOOL_CHAR_BUDGET", "10")
        .output()
        .unwrap();
    assert!(
        stdout(&from_env).contains("of 10 budget (OVER)"),
        "{}",
        stdout(&from_env)
    );
}
