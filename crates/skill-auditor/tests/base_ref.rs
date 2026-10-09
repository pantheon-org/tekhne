//! `evaluate --base <ref>` and `batch --base <ref>` score a skill folder as it
//! was at a git ref and report the delta against the current tree.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const THIN: &str = "---\nname: demo\ndescription: A deliberately thin skill so that it grades below A.\n---\n\n# Demo\n\nBody.\n";
const RICHER: &str = "---\nname: demo\ndescription: A richer demonstration skill that explains when to use it, what it produces and what to avoid, so a model can match requests to it.\n---\n\n# Demo\n\n## When to Use\n\n- NEVER skip the check, because production data depends on it.\n\n## When NOT to Use\n\n- ALWAYS prefer the simple path when it is enough.\n\n```bash\n./run.sh\n```\n";

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["-c", "commit.gpgsign=false"])
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn write(repo: &Path, rel: &str, content: &str) {
    let path = repo.join(rel);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write");
}

/// A repo whose `base` tag holds `skills/demo/one` (thin, with a reference
/// file) and whose working tree then holds the richer version with no
/// references.
fn repo_with_history() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("temp repo");
    let p = repo.path();
    git(p, &["init", "-q", "-b", "main"]);
    write(p, "skills/demo/one/SKILL.md", THIN);
    write(
        p,
        "skills/demo/one/references/notes.md",
        "# Notes\n\nA reference.\n",
    );
    git(p, &["add", "."]);
    git(p, &["commit", "-q", "-m", "base"]);
    git(p, &["tag", "base"]);
    write(p, "skills/demo/one/SKILL.md", RICHER);
    fs::remove_dir_all(p.join("skills/demo/one/references")).expect("rm references");
    write(p, "skills/demo/two/SKILL.md", THIN);
    git(p, &["add", "."]);
    git(p, &["commit", "-q", "-m", "change"]);
    repo
}

fn run(repo: &Path, sub: &str, skills: &[&str], extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .arg(sub)
        .args(skills)
        .arg("--repo-root")
        .arg(repo)
        .args(extra)
        .output()
        .expect("run pantheon-skill-auditor")
}

fn json(out: &Output) -> Value {
    assert!(
        out.status.success(),
        "exit {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("json on stdout")
}

#[test]
fn evaluate_reports_baseline_current_and_delta() {
    let repo = repo_with_history();
    let v = json(&run(
        repo.path(),
        "evaluate",
        &["demo/one"],
        &["--json", "--base", "base"],
    ));
    assert_eq!(v["schema_version"], 1);
    assert_eq!(v["skill"], "demo/one");
    assert_eq!(v["status"], "changed");
    assert_eq!(v["base"]["ref"], "base");
    assert_eq!(v["base"]["sha"].as_str().expect("sha").len(), 40);
    let before = v["baseline"]["total"].as_i64().expect("baseline total");
    let after = v["current"]["total"].as_i64().expect("current total");
    assert!(after > before, "the richer skill should score higher");
    assert_eq!(v["delta"].as_i64(), Some(after - before));
    let dims = v["dimension_deltas"].as_object().expect("dimension deltas");
    let summed: i64 = dims.values().map(|d| d.as_i64().expect("int")).sum();
    assert_eq!(summed, after - before);
}

#[test]
fn the_whole_skill_folder_is_read_at_the_ref() {
    let repo = repo_with_history();
    let v = json(&run(
        repo.path(),
        "evaluate",
        &["demo/one"],
        &["--json", "--base", "base"],
    ));
    assert_eq!(
        v["baseline"]["hasReferences"], true,
        "references/ existed at the ref"
    );
    assert_eq!(
        v["current"]["hasReferences"], false,
        "and is gone from the tree"
    );
}

#[test]
fn a_skill_missing_at_the_ref_is_new_not_an_error() {
    let repo = repo_with_history();
    let v = json(&run(
        repo.path(),
        "evaluate",
        &["demo/two"],
        &["--json", "--base", "base"],
    ));
    assert_eq!(v["status"], "new");
    assert!(v["baseline"].is_null());
    assert!(v["delta"].is_null());
    assert!(v["current"]["total"].is_number());
}

#[test]
fn an_unchanged_skill_reports_zero_delta() {
    let repo = repo_with_history();
    let v = json(&run(
        repo.path(),
        "evaluate",
        &["demo/two"],
        &["--json", "--base", "HEAD"],
    ));
    assert_eq!(v["status"], "unchanged");
    assert_eq!(v["delta"], 0);
}

#[test]
fn batch_reports_one_record_per_skill() {
    let repo = repo_with_history();
    let v = json(&run(
        repo.path(),
        "batch",
        &["demo/one", "demo/two"],
        &["--json", "--base", "base"],
    ));
    let records = v.as_array().expect("array");
    let status = |skill: &str| {
        records
            .iter()
            .find(|r| r["skill"] == skill)
            .map(|r| r["status"].as_str().expect("status").to_string())
    };
    assert_eq!(records.len(), 2);
    assert_eq!(status("demo/one").as_deref(), Some("changed"));
    assert_eq!(status("demo/two").as_deref(), Some("new"));
}

#[test]
fn text_output_names_the_ref_and_the_delta() {
    let repo = repo_with_history();
    let out = run(repo.path(), "evaluate", &["demo/one"], &["--base", "base"]);
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("Compared with base"), "{text}");
}

#[test]
fn a_missing_ref_fails_with_a_clear_message() {
    let repo = repo_with_history();
    let out = run(
        repo.path(),
        "evaluate",
        &["demo/one"],
        &["--base", "no-such-ref"],
    );
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("git ref 'no-such-ref' not found"), "{err}");
    assert!(
        err.contains("shallow"),
        "should mention shallow clones: {err}"
    );
}

#[test]
fn a_ref_starting_with_a_dash_is_rejected_before_git() {
    let repo = repo_with_history();
    for bad in ["--output=/tmp/x", "-h", "--help"] {
        let out = run(
            repo.path(),
            "evaluate",
            &["demo/one"],
            &[&format!("--base={bad}")],
        );
        assert!(!out.status.success(), "{bad} should fail");
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.contains("must not start with '-'"), "{bad}: {err}");
    }
}

#[test]
fn base_requires_repo_root() {
    let out = Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .args(["evaluate", "demo/one", "--base", "main"])
        .output()
        .expect("run pantheon-skill-auditor");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("--repo-root"));
}

#[test]
fn a_repo_root_that_is_not_a_git_checkout_is_named_in_the_error() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "skills/demo/one/SKILL.md", THIN);
    let out = run(dir.path(), "evaluate", &["demo/one"], &["--base", "main"]);
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("not a git checkout"), "{err}");
}

#[test]
fn without_base_the_output_has_no_comparison_fields() {
    let repo = repo_with_history();
    let v = json(&run(repo.path(), "evaluate", &["demo/one"], &["--json"]));
    assert!(v.get("schema_version").is_none());
    assert!(v["total"].is_number());
}

#[test]
fn the_working_tree_is_left_untouched() {
    let repo = repo_with_history();
    let before = fs::read_to_string(repo.path().join("skills/demo/one/SKILL.md")).expect("read");
    json(&run(
        repo.path(),
        "evaluate",
        &["demo/one"],
        &["--json", "--base", "base"],
    ));
    let after = fs::read_to_string(repo.path().join("skills/demo/one/SKILL.md")).expect("read");
    assert_eq!(before, after);
    assert!(!repo.path().join("skills/demo/one/references").exists());
}
