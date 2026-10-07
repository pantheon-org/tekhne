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

fn write_trigger_skill(dir: &std::path::Path, description: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: s\ndescription: {description}\n---\n\nBody\n"),
    )
    .unwrap();
}

#[test]
fn analyze_triggers_names_dropped_phrases_and_stays_advisory() {
    let tmp = tempfile::tempdir().unwrap();
    let (base, head) = (tmp.path().join("base"), tmp.path().join("head"));
    write_trigger_skill(&base, "'X. Use when running lint, fixing formatter drift'");
    write_trigger_skill(&head, "'X. Use when running lint'");

    let out = run(&[
        "analyze",
        "triggers",
        head.to_str().unwrap(),
        "--base",
        base.to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0);
    assert!(
        stdout(&out).contains("fixing formatter drift"),
        "{}",
        stdout(&out)
    );

    let json = run(&[
        "analyze",
        "triggers",
        head.to_str().unwrap(),
        "--base",
        base.to_str().unwrap(),
        "-o",
        "json",
    ]);
    assert!(stdout(&json).contains("\"dropped\""));

    let new = run(&[
        "analyze",
        "triggers",
        head.to_str().unwrap(),
        "--base",
        "/nonexistent/base",
    ]);
    assert_eq!(code(&new), 0);
    assert!(stdout(&new).contains("new skill"));
}

fn write_fm_skill(dir: &std::path::Path, frontmatter: &str) {
    std::fs::create_dir_all(dir).unwrap();
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    std::fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: {name}\n{frontmatter}\n---\n\n# S\n\nBody text for the skill with enough words to count.\n"),
    )
    .unwrap();
}

#[test]
fn unquoted_description_with_colon_gets_an_actionable_error() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("s");
    write_fm_skill(&dir, "description: Does a thing. Use when: building tools");
    let out = run(&["validate", "structure", dir.to_str().unwrap()]);
    assert_eq!(code(&out), 1);
    assert!(
        stdout(&out).contains("wrap the description in quotes"),
        "{}",
        stdout(&out)
    );

    let quoted = tmp.path().join("q");
    write_fm_skill(
        &quoted,
        "description: 'Does a thing. Use when: building tools'",
    );
    assert_eq!(
        code(&run(&["validate", "structure", quoted.to_str().unwrap()])),
        0
    );
}

#[test]
fn description_near_the_limit_warns_and_over_it_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let near = tmp.path().join("near");
    write_fm_skill(
        &near,
        &format!("description: {}", "word ".repeat(210)[..1000].trim_end()),
    );
    let out = run(&["validate", "structure", near.to_str().unwrap()]);
    assert_eq!(code(&out), 2, "{}", stdout(&out));
    assert!(stdout(&out).contains("left"), "{}", stdout(&out));

    let over = tmp.path().join("over");
    write_fm_skill(&over, &format!("description: {}", "a".repeat(1100)));
    assert_eq!(
        code(&run(&["validate", "structure", over.to_str().unwrap()])),
        1
    );

    let fine = tmp.path().join("fine");
    write_fm_skill(
        &fine,
        "description: A sufficiently descriptive description for a plain test skill.",
    );
    assert_eq!(
        code(&run(&["validate", "structure", fine.to_str().unwrap()])),
        0
    );
}

fn write_probe_fixture(root: &std::path::Path) {
    write_listing_skill(
        root,
        "skills/gen",
        "name: gen\ndescription: Generate terraform modules. Use when writing terraform resources\n",
    );
    write_listing_skill(
        root,
        "skills/val",
        "name: val\ndescription: Validate terraform configs. Use when linting terraform plans\n",
    );
    let q = |id: &str, split: &str, expect: bool, request: &str| serde_json::json!({"id": id, "split": split, "expect_trigger": expect, "request": request});
    let mut queries = Vec::new();
    for i in 0..5 {
        queries.push(q(
            &format!("p{i}"),
            if i < 3 { "train" } else { "validation" },
            true,
            &format!("make {i} resources for the cloud"),
        ));
        queries.push(q(
            &format!("n{i}"),
            if i < 3 { "train" } else { "validation" },
            false,
            &format!("bake {i} cakes"),
        ));
    }
    let probe = serde_json::json!({
        "skill": "t:gen", "skill_dir": "skills/gen", "competitors": ["skills/val"], "queries": queries
    });
    std::fs::create_dir_all(root.join("probes/baselines")).unwrap();
    std::fs::write(
        root.join("probes/gen.json"),
        serde_json::to_string(&probe).unwrap(),
    )
    .unwrap();
}

#[test]
fn probes_validate_score_and_compare_round_trip() {
    let tmp = tempfile::tempdir().unwrap();
    write_probe_fixture(tmp.path());
    let root = tmp.path().to_str().unwrap();
    let dir = tmp.path().join("probes");
    let dir = dir.to_str().unwrap();

    let v = run(&["probes", "validate", dir, "--repo-root", root]);
    assert_eq!(code(&v), 0, "{}", stdout(&v));
    assert!(stdout(&v).contains("0 error(s)"));

    let s = run(&["probes", "score", dir, "--repo-root", root]);
    assert_eq!(code(&s), 0);
    let report: serde_json::Value = serde_json::from_str(&stdout(&s)).unwrap();
    assert_eq!(report["method"], "listing-overlap");
    assert!(report["note"]
        .as_str()
        .unwrap()
        .contains("not a model-graded"));
    assert_eq!(report["skills"][0]["splits"]["train"]["n"], 6);

    let base = tmp.path().join("base.json");
    std::fs::write(&base, stdout(&s)).unwrap();
    let c = run(&[
        "probes",
        "compare",
        base.to_str().unwrap(),
        base.to_str().unwrap(),
    ]);
    assert_eq!(code(&c), 0);
    let delta: serde_json::Value = serde_json::from_str(&stdout(&c)).unwrap();
    assert_eq!(delta["skills"][0]["train"]["trigger_rate_delta"], 0.0);
}

#[test]
fn probes_validate_fails_on_a_bad_probe_file_and_score_on_a_missing_skill() {
    let tmp = tempfile::tempdir().unwrap();
    write_probe_fixture(tmp.path());
    let probes = tmp.path().join("probes");
    let mut p: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(probes.join("gen.json")).unwrap()).unwrap();
    p["skill_dir"] = "skills/missing".into();
    std::fs::write(probes.join("gen.json"), p.to_string()).unwrap();
    let root = tmp.path().to_str().unwrap();

    let v = run(&[
        "probes",
        "validate",
        probes.to_str().unwrap(),
        "--repo-root",
        root,
    ]);
    assert_eq!(code(&v), 1);
    assert!(stdout(&v).contains("FAIL"));
    let s = run(&[
        "probes",
        "score",
        probes.to_str().unwrap(),
        "--repo-root",
        root,
    ]);
    assert_eq!(code(&s), 1);
    assert_eq!(
        code(&run(&["probes", "validate", "/nonexistent/probes"])),
        1
    );
}

#[test]
fn committed_probes_validate_against_the_repository() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = run(&[
        "probes",
        "validate",
        repo.join("probes").to_str().unwrap(),
        "--repo-root",
        repo.to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0, "{}", stdout(&out));
}

#[test]
fn analyze_triggers_reads_a_triggers_marker() {
    let tmp = tempfile::tempdir().unwrap();
    let (base, head) = (tmp.path().join("base"), tmp.path().join("head"));
    write_trigger_skill(
        &base,
        "'X. Triggers: search papers, paper by doi, citation analysis'",
    );
    write_trigger_skill(&head, "'X. Triggers: search papers, citation analysis'");

    let out = run(&[
        "analyze",
        "triggers",
        head.to_str().unwrap(),
        "--base",
        base.to_str().unwrap(),
        "-o",
        "json",
    ]);
    assert_eq!(code(&out), 0);
    let report: serde_json::Value = serde_json::from_str(&stdout(&out)).unwrap();
    assert_eq!(
        report["triggers"]["dropped"],
        serde_json::json!(["paper by doi"])
    );
    assert_eq!(
        report["triggers"]["base_phrases"],
        serde_json::json!(["citation analysis", "paper by doi", "search papers"])
    );
}
