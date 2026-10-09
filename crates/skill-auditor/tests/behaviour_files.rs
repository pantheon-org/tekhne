//! Behaviour files inside an eval scenario: reading them, reporting a broken
//! one without touching the score, and the shell validator that checks their
//! shape. A broken file is pushed through both the Rust path and the shell path.

use serde_json::Value;
use skill_auditor::behaviour::load::{load, problems};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BEHAVIOUR: &str = "schema_version: 1\nname: notes helper\nprompt: Follow the notes file.\nexpect:\n  files_changed: [NOTES.md]\n  must_not:\n    files_changed: [TODO.md]\n  output_contains: [done]\n";

fn trace(files: &str, output: &str) -> String {
    format!(
        r#"{{
  "schema_version": 1,
  "header": {{ "harness": "claude-code", "date": "2026-10-09", "skill_commit": "abc1234", "source": "recorded" }},
  "toolCalls": [{{ "tool": "Read", "input": {{}} }}, {{ "tool": "Edit", "input": {{}} }}],
  "filesChanged": {files},
  "commandsRun": [],
  "output": "{output}"
}}"#
    )
}

fn write(dir: &Path, rel: &str, content: &str) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write");
}

/// A scenario folder with the three files the auditor already requires.
fn scenario() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    write(dir.path(), "task.md", "# Task\n\nA prompt.\n");
    write(dir.path(), "capability.txt", "Follows the notes file.\n");
    write(
        dir.path(),
        "criteria.json",
        r#"{ "checklist": [ { "name": "one", "description": "d", "max_score": 100 } ] }"#,
    );
    dir
}

fn with_behaviour(dir: &Path) {
    write(dir, "behaviour.yaml", BEHAVIOUR);
    write(dir, "traces/old.json", &trace(r#"["NOTES.md"]"#, "done"));
    write(
        dir,
        "traces/new.json",
        &trace(r#"["NOTES.md", "TODO.md"]"#, "done"),
    );
}

// ---- the Rust loader -----------------------------------------------------------

#[test]
fn a_scenario_without_behaviour_files_is_not_a_problem() {
    let dir = scenario();
    assert!(load(dir.path()).expect("no problem").is_none());
    assert!(problems(dir.path()).is_empty());
}

#[test]
fn a_valid_scenario_loads_with_both_traces() {
    let dir = scenario();
    with_behaviour(dir.path());
    let loaded = load(dir.path()).expect("valid").expect("present");
    assert_eq!(loaded.behaviour.name, "notes helper");
    assert_eq!(loaded.old.files_changed, ["NOTES.md"]);
    assert_eq!(loaded.new.files_changed, ["NOTES.md", "TODO.md"]);
    assert!(problems(dir.path()).is_empty());
}

#[test]
fn a_broken_behaviour_file_is_named() {
    let dir = scenario();
    with_behaviour(dir.path());
    write(dir.path(), "behaviour.yaml", "name: [unterminated");
    let p = problems(dir.path());
    assert_eq!(p.len(), 1, "{p:?}");
    assert!(p[0].starts_with("behaviour.yaml:"), "{p:?}");
}

#[test]
fn a_broken_trace_is_named() {
    let dir = scenario();
    with_behaviour(dir.path());
    write(dir.path(), "traces/new.json", "{ this is not json");
    let p = problems(dir.path());
    assert_eq!(p.len(), 1, "{p:?}");
    assert!(p[0].starts_with("traces/new.json:"), "{p:?}");
    assert!(p[0].contains("invalid JSON"), "{p:?}");
}

#[test]
fn every_problem_is_reported_not_just_the_first() {
    let dir = scenario();
    write(dir.path(), "behaviour.yaml", "name: [unterminated");
    write(dir.path(), "traces/old.json", "nope");
    let p = problems(dir.path());
    assert_eq!(p.len(), 3, "{p:?}"); // behaviour.yaml, old.json broken, new.json missing
}

#[test]
fn a_missing_trace_is_named() {
    let dir = scenario();
    with_behaviour(dir.path());
    fs::remove_file(dir.path().join("traces/old.json")).expect("rm");
    let p = problems(dir.path());
    assert_eq!(p, ["traces/old.json is missing"]);
}

#[test]
fn traces_without_a_behaviour_file_are_a_problem() {
    let dir = scenario();
    write(dir.path(), "traces/old.json", &trace("[]", "x"));
    let p = problems(dir.path());
    assert_eq!(p.len(), 1, "{p:?}");
    assert!(p[0].contains("without behaviour.yaml"), "{p:?}");
}

// ---- evaluate: the score does not move -----------------------------------------

fn repo_with_skill() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repo");
    let skill = repo.path().join("skills/demo/one");
    write(
        &skill,
        "SKILL.md",
        "---\nname: one\ndescription: A demo skill used by the behaviour tests.\n---\n\n# One\n\nDo the thing.\n",
    );
    let sc = skill.join("evals/scenario-1");
    write(&sc, "task.md", "# Task\n\nA prompt.\n");
    write(&sc, "capability.txt", "Follows the notes file.\n");
    write(
        &sc,
        "criteria.json",
        r#"{ "checklist": [ { "name": "one", "description": "d", "max_score": 100 } ] }"#,
    );
    repo
}

fn evaluate(repo: &Path) -> Value {
    let out: Output = Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .args(["evaluate", "demo/one", "--json", "--repo-root"])
        .arg(repo)
        .output()
        .expect("run auditor");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("json")
}

fn warning_messages(v: &Value) -> Vec<String> {
    v["warningDetails"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|d| d["message"].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn valid_behaviour_files_change_neither_score_nor_warnings() {
    let repo = repo_with_skill();
    let before = evaluate(repo.path());
    with_behaviour(&repo.path().join("skills/demo/one/evals/scenario-1"));
    let after = evaluate(repo.path());
    assert_eq!(before["total"], after["total"]);
    assert_eq!(before["dimensions"], after["dimensions"]);
    assert_eq!(warning_messages(&before), warning_messages(&after));
}

#[test]
fn a_broken_trace_adds_one_report_only_warning_and_no_score_change() {
    let repo = repo_with_skill();
    let sc = repo.path().join("skills/demo/one/evals/scenario-1");
    let before = evaluate(repo.path());
    with_behaviour(&sc);
    write(&sc, "traces/new.json", "{ this is not json");
    let after = evaluate(repo.path());
    assert_eq!(before["total"], after["total"]);
    assert_eq!(before["dimensions"], after["dimensions"]);
    assert_eq!(after["errors"], before["errors"]);
    assert_eq!(
        after["warnings"].as_i64(),
        before["warnings"].as_i64().map(|n| n + 1)
    );
    let msgs = warning_messages(&after);
    let hit = msgs
        .iter()
        .find(|m| m.contains("scenario-1/traces/new.json"))
        .unwrap_or_else(|| panic!("no warning naming the file: {msgs:?}"));
    assert!(hit.contains("not scored"), "{hit}");
}

// ---- the shell validator --------------------------------------------------------

fn validator() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../skills/agentic-harness/skill-quality-auditor/scripts/validate-behaviour-files.sh",
    )
}

fn have(tool: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {tool}")])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// The validator uses `yq` and `jq`, like the artifact validator beside it. If
/// either is missing the test says so out loud and stops, so a skipped test is
/// visible and not a quiet pass.
fn tools_present() -> bool {
    let ok = have("yq") && have("jq");
    if !ok {
        eprintln!("SKIPPED: yq and jq are both needed for the shell validator tests");
    }
    ok
}

fn run_validator(file: &Path) -> Output {
    Command::new("sh")
        .arg(validator())
        .arg(file)
        .output()
        .expect("run validator")
}

#[test]
fn the_shell_validator_accepts_valid_behaviour_files() {
    if !tools_present() {
        return;
    }
    let dir = scenario();
    with_behaviour(dir.path());
    for f in ["behaviour.yaml", "traces/old.json", "traces/new.json"] {
        let out = run_validator(&dir.path().join(f));
        assert!(
            out.status.success(),
            "{f}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

#[test]
fn the_shell_validator_rejects_a_broken_trace_and_names_it() {
    if !tools_present() {
        return;
    }
    let dir = scenario();
    with_behaviour(dir.path());
    write(dir.path(), "traces/new.json", "{ this is not json");
    let out = run_validator(&dir.path().join("traces/new.json"));
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("traces/new.json"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn the_shell_validator_rejects_a_trace_with_an_unknown_field() {
    if !tools_present() {
        return;
    }
    let dir = scenario();
    with_behaviour(dir.path());
    let bad = trace("[]", "x").replace("\"output\"", "\"outputs\"");
    write(dir.path(), "traces/new.json", &bad);
    let out = run_validator(&dir.path().join("traces/new.json"));
    assert!(!out.status.success());
}

#[test]
fn the_shell_validator_rejects_a_misspelt_expectation() {
    if !tools_present() {
        return;
    }
    let dir = scenario();
    with_behaviour(dir.path());
    write(
        dir.path(),
        "behaviour.yaml",
        "schema_version: 1\nname: n\nexpect:\n  output_not_contain: [x]\n",
    );
    let out = run_validator(&dir.path().join("behaviour.yaml"));
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("output_not_contain"));
}

#[test]
fn the_shell_validator_rejects_a_behaviour_file_without_its_traces() {
    if !tools_present() {
        return;
    }
    let dir = scenario();
    write(dir.path(), "behaviour.yaml", BEHAVIOUR);
    let out = run_validator(&dir.path().join("behaviour.yaml"));
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("traces/old.json"));
}
