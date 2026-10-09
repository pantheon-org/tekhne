//! The behaviour report: old and new traces compared assertion by assertion,
//! shown beside the score and never part of it. Skills without behaviour files
//! must see no change in `evaluate` output or in stored audits.

use serde_json::Value;
use skill_auditor::behaviour::ledger::{
    compare, format_text, report_for_skill, to_json, Report, Verdict,
};
use skill_auditor::behaviour::load::load;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const BEHAVIOUR: &str = "schema_version: 1\nname: notes helper\nprompt: Follow the notes file.\nexpect:\n  files_changed: [NOTES.md]\n  must_not:\n    files_changed: [TODO.md]\n  output_contains: [done]\n";

fn trace(commit: &str, source: &str, files: &str, output: &str) -> String {
    format!(
        r#"{{
  "schema_version": 1,
  "header": {{ "harness": "claude-code", "date": "2026-10-09", "skill_commit": "{commit}", "source": "{source}" }},
  "toolCalls": [{{ "tool": "Edit", "input": {{}} }}],
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

/// Old passes everything. New also changes TODO.md, which the scenario forbids.
fn regressing_scenario(dir: &Path) {
    write(dir, "task.md", "# Task\n\nA prompt.\n");
    write(dir, "capability.txt", "Follows the notes file.\n");
    write(
        dir,
        "criteria.json",
        r#"{ "checklist": [ { "name": "one", "description": "d", "max_score": 100 } ] }"#,
    );
    write(dir, "behaviour.yaml", BEHAVIOUR);
    write(
        dir,
        "traces/old.json",
        &trace("abc1234", "recorded", r#"["NOTES.md"]"#, "done"),
    );
    write(
        dir,
        "traces/new.json",
        &trace("def5678", "recorded", r#"["NOTES.md", "TODO.md"]"#, "done"),
    );
}

fn report_for(dir: &Path) -> Report {
    let loaded = load(dir).expect("valid").expect("present");
    Report {
        schema_version: skill_auditor::behaviour::SCHEMA_VERSION,
        scenarios: vec![compare("scenario-1", &loaded)],
    }
}

// ---- the ledger ----------------------------------------------------------------

#[test]
fn a_check_that_passed_before_and_fails_now_is_a_regression() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    let report = report_for(dir.path());
    let s = &report.scenarios[0];
    assert_eq!(s.regressions, 1);
    let verdicts: Vec<Verdict> = s.rows.iter().map(|r| r.verdict).collect();
    assert_eq!(
        verdicts,
        [Verdict::Held, Verdict::Regression, Verdict::Held]
    );
    let reg = &s.rows[1];
    assert!(reg.old_pass && !reg.new_pass);
    assert_eq!(reg.expected, "files_changed does not include TODO.md");
}

#[test]
fn a_check_that_failed_before_and_passes_now_is_fixed_and_one_failing_both_times_is_failing() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    // Old misses NOTES.md and the output text; new gets them. Nothing is forbidden now.
    write(
        dir.path(),
        "traces/old.json",
        &trace("abc1234", "recorded", "[]", "nope"),
    );
    write(
        dir.path(),
        "traces/new.json",
        &trace("def5678", "recorded", r#"["NOTES.md"]"#, "done"),
    );
    write(
        dir.path(),
        "behaviour.yaml",
        &format!("{BEHAVIOUR}  output_not_contains: [nope]\n"),
    );
    let report = report_for(dir.path());
    let s = &report.scenarios[0];
    assert_eq!(s.regressions, 0);
    let by: Vec<(&str, Verdict)> = s
        .rows
        .iter()
        .map(|r| (r.expected.as_str(), r.verdict))
        .collect();
    assert!(by.contains(&("NOTES.md", Verdict::Fixed)), "{by:?}");
    assert!(by.contains(&("done", Verdict::Fixed)), "{by:?}");
    // New passes the forbidden check and the old trace did too, so it held.
    assert!(
        by.contains(&("files_changed does not include TODO.md", Verdict::Held)),
        "{by:?}"
    );
    // Old said "nope" so the forbidden text was present; new does not say it.
    assert!(
        by.contains(&("output does not include nope", Verdict::Fixed)),
        "{by:?}"
    );
}

#[test]
fn a_check_that_fails_in_both_runs_is_failing() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    write(
        dir.path(),
        "traces/old.json",
        &trace("abc1234", "recorded", "[]", "x"),
    );
    write(
        dir.path(),
        "traces/new.json",
        &trace("def5678", "recorded", "[]", "x"),
    );
    let report = report_for(dir.path());
    let s = &report.scenarios[0];
    assert_eq!(s.rows[0].verdict, Verdict::Failing); // NOTES.md never changed
}

#[test]
fn the_text_table_names_the_traces_and_marks_the_regression() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    let text = format_text(&report_for(dir.path()));
    assert!(text.contains("Behaviour checks"), "{text}");
    assert!(text.contains("not part of the score"), "{text}");
    assert!(text.contains("scenario-1: notes helper"), "{text}");
    assert!(
        text.contains("claude-code, 2026-10-09, skill abc1234, recorded"),
        "{text}"
    );
    assert!(text.contains("skill def5678"), "{text}");
    let line = text
        .lines()
        .find(|l| l.contains("TODO.md"))
        .expect("the forbidden-file row");
    assert!(
        line.contains("pass") && line.contains("FAIL") && line.contains("REGRESSION"),
        "{line}"
    );
    assert!(text.contains("1 regression"), "{text}");
}

#[test]
fn a_hand_written_trace_is_flagged_in_the_table() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    write(
        dir.path(),
        "traces/new.json",
        &trace("def5678", "hand-written", r#"["NOTES.md"]"#, "done"),
    );
    let text = format_text(&report_for(dir.path()));
    assert!(text.contains("hand-written"), "{text}");
}

#[test]
fn a_skill_without_behaviour_files_has_no_report() {
    let dir = tempfile::tempdir().expect("tempdir");
    write(dir.path(), "SKILL.md", "# A skill\n");
    write(dir.path(), "evals/scenario-1/task.md", "# Task\n");
    assert!(report_for_skill(dir.path()).is_none());
}

#[test]
fn a_skill_with_behaviour_files_reports_each_scenario_in_order() {
    let dir = tempfile::tempdir().expect("tempdir");
    write(dir.path(), "SKILL.md", "# A skill\n");
    regressing_scenario(&dir.path().join("evals/scenario-2"));
    regressing_scenario(&dir.path().join("evals/scenario-1"));
    let report = report_for_skill(dir.path()).expect("report");
    let names: Vec<&str> = report
        .scenarios
        .iter()
        .map(|s| s.scenario.as_str())
        .collect();
    assert_eq!(names, ["scenario-1", "scenario-2"]);
    assert_eq!(report.schema_version, 1);
}

#[test]
fn a_broken_scenario_is_left_out_of_the_report_not_guessed_at() {
    let dir = tempfile::tempdir().expect("tempdir");
    write(dir.path(), "SKILL.md", "# A skill\n");
    regressing_scenario(&dir.path().join("evals/scenario-1"));
    regressing_scenario(&dir.path().join("evals/scenario-2"));
    write(
        &dir.path().join("evals/scenario-2"),
        "traces/new.json",
        "{ broken",
    );
    let report = report_for_skill(dir.path()).expect("report");
    assert_eq!(report.scenarios.len(), 1);
    assert_eq!(report.scenarios[0].scenario, "scenario-1");
}

// ---- evaluate: shown beside the score, not part of it ---------------------------

fn repo_with_skill() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repo");
    let skill = repo.path().join("skills/demo/one");
    write(
        &skill,
        "SKILL.md",
        "---\nname: one\ndescription: A demo skill used by the behaviour tests.\n---\n\n# One\n\nDo the thing.\n",
    );
    regressing_scenario(&skill.join("evals/scenario-1"));
    repo
}

fn run(repo: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .args(["evaluate", "demo/one", "--repo-root"])
        .arg(repo)
        .args(extra)
        .output()
        .expect("run auditor")
}

fn json(out: &Output) -> Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("json")
}

#[test]
fn evaluate_prints_the_table_after_the_score_report() {
    let repo = repo_with_skill();
    let out = run(repo.path(), &[]);
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    let score_at = text.find("Behaviour checks").expect("table present");
    assert!(score_at > 0, "{text}");
    assert!(text.contains("REGRESSION"), "{text}");
}

#[test]
fn evaluate_json_adds_a_versioned_behaviour_field_only_when_there_are_files() {
    let repo = repo_with_skill();
    let with = json(&run(repo.path(), &["--json"]));
    assert_eq!(with["behaviour"]["schema_version"], 1);
    assert_eq!(with["behaviour"]["scenarios"][0]["regressions"], 1);

    fs::remove_file(
        repo.path()
            .join("skills/demo/one/evals/scenario-1/behaviour.yaml"),
    )
    .expect("rm");
    fs::remove_dir_all(repo.path().join("skills/demo/one/evals/scenario-1/traces")).expect("rm");
    let without = json(&run(repo.path(), &["--json"]));
    assert!(without.get("behaviour").is_none(), "{without}");
}

#[test]
fn the_score_is_the_same_with_and_without_the_behaviour_files() {
    let repo = repo_with_skill();
    let with = json(&run(repo.path(), &["--json"]));
    fs::remove_file(
        repo.path()
            .join("skills/demo/one/evals/scenario-1/behaviour.yaml"),
    )
    .expect("rm");
    fs::remove_dir_all(repo.path().join("skills/demo/one/evals/scenario-1/traces")).expect("rm");
    let without = json(&run(repo.path(), &["--json"]));
    for key in ["total", "grade", "dimensions", "errors", "warnings"] {
        assert_eq!(with[key], without[key], "{key}");
    }
}

#[test]
fn a_stored_audit_does_not_carry_the_behaviour_report() {
    let repo = repo_with_skill();
    let out = run(repo.path(), &["--store"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    // Find any stored audit.json under the repo and check none mentions behaviour.
    let mut found = 0;
    let mut stack = vec![repo.path().to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read_dir").flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == "audit.json") {
                found += 1;
                let stored: Value =
                    serde_json::from_str(&fs::read_to_string(&p).expect("read")).expect("json");
                assert!(stored.get("behaviour").is_none(), "{}", p.display());
            }
        }
    }
    assert!(found > 0, "no stored audit.json was written");
}

#[test]
fn batch_json_is_unchanged_by_behaviour_files() {
    let repo = repo_with_skill();
    let out = Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .args(["batch", "demo/one", "--json", "--repo-root"])
        .arg(repo.path())
        .output()
        .expect("run auditor");
    let v = json(&out);
    assert!(v[0].get("behaviour").is_none(), "{v}");
}

#[test]
fn evaluate_with_base_also_carries_the_behaviour_field() {
    let repo = repo_with_skill();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args([
                "-c",
                "user.email=t@t",
                "-c",
                "user.name=t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .expect("git");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q", "-b", "main"]);
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "base"]);
    let v = json(&run(repo.path(), &["--json", "--base", "HEAD"]));
    assert_eq!(v["status"], "unchanged");
    assert_eq!(v["behaviour"]["scenarios"][0]["regressions"], 1);
}

// ---- JSON output: unchanged unless there is a report ----------------------------

#[derive(serde::Serialize)]
struct Sample {
    zebra: u8,
    apple: u8,
}

#[test]
fn without_a_report_the_json_is_byte_for_byte_what_the_value_serialises_to() {
    // Field order is zebra then apple. A round trip through a JSON value would
    // sort them, so this fails if the no-report path ever goes through one.
    let sample = Sample { zebra: 1, apple: 2 };
    let direct = serde_json::to_string_pretty(&sample).expect("direct");
    assert_eq!(to_json(&sample, None).expect("json"), direct);
    assert!(direct.find("zebra").expect("z") < direct.find("apple").expect("a"));
}

#[test]
fn with_a_report_the_json_gains_a_behaviour_field_and_keeps_the_rest() {
    let dir = tempfile::tempdir().expect("tempdir");
    regressing_scenario(dir.path());
    let report = report_for(dir.path());
    let text = to_json(&Sample { zebra: 1, apple: 2 }, Some(&report)).expect("json");
    let v: Value = serde_json::from_str(&text).expect("json");
    assert_eq!(v["zebra"], 1);
    assert_eq!(v["apple"], 2);
    assert_eq!(v["behaviour"]["schema_version"], 1);
}
