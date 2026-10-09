//! An older auditor build must still accept a `criteria.json` that uses
//! `failure_check`: it ignores the field, so the file sums and scores as before.
//!
//! The older build is whatever binary `PANTHEON_LEGACY_AUDITOR` names, for
//! example the release on PATH. The test is skipped when it is unset, so CI does
//! not depend on what happens to be installed.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

const SKILL: &str = "---\nname: demo\ndescription: A deliberately thin skill so that it grades below A.\n---\n\n# Demo\n\nBody.\n";
const PLAIN: &str =
    r#"{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":40}]}"#;
const WITH_FAILURE_CHECK: &str = r#"{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":40},{"description":"never deletes the file","failure_check":true}]}"#;

fn repo_with_criteria(criteria: &str) -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("temp repo");
    let skill = repo.path().join("skills/demo/one");
    let evals = skill.join("evals");
    fs::create_dir_all(&evals).expect("evals dir");
    fs::write(skill.join("SKILL.md"), SKILL).expect("skill");
    fs::write(
        evals.join("instructions.json"),
        r#"{"instructions":[{"type":"a"}]}"#,
    )
    .expect("instructions");
    fs::write(
        evals.join("summary.json"),
        r#"{"instructions_coverage":{"coverage_percentage":85}}"#,
    )
    .expect("summary");
    for n in 1..=3 {
        let dir = evals.join(format!("scenario-{n}"));
        fs::create_dir_all(&dir).expect("scenario dir");
        fs::write(dir.join("task.md"), "# Task").expect("task");
        fs::write(dir.join("capability.txt"), "cap").expect("capability");
        fs::write(dir.join("criteria.json"), criteria).expect("criteria");
    }
    repo
}

fn audit(binary: &str, repo: &Path) -> Value {
    let out = Command::new(binary)
        .args(["evaluate", "demo/one", "--json", "--repo-root"])
        .arg(repo)
        .output()
        .expect("run auditor");
    assert!(
        out.status.success(),
        "{binary} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("json")
}

#[test]
fn an_older_build_accepts_a_criteria_file_that_uses_failure_check() {
    let Some(legacy) = std::env::var_os("PANTHEON_LEGACY_AUDITOR") else {
        eprintln!("skipped: PANTHEON_LEGACY_AUDITOR is not set");
        return;
    };
    let legacy = legacy.to_str().expect("utf-8 path");
    let plain = repo_with_criteria(PLAIN);
    let marked = repo_with_criteria(WITH_FAILURE_CHECK);

    let old_plain = audit(legacy, plain.path());
    let old_marked = audit(legacy, marked.path());
    assert_eq!(old_marked["errors"], 0, "{old_marked}");
    assert_eq!(old_marked["total"], old_plain["total"]);
    assert_eq!(old_marked["dimensions"], old_plain["dimensions"]);

    let current = env!("CARGO_BIN_EXE_pantheon-skill-auditor");
    let new_marked = audit(current, marked.path());
    assert_eq!(
        new_marked["total"], old_marked["total"],
        "a new build must score it the same"
    );
    assert_eq!(new_marked["dimensions"], old_marked["dimensions"]);
}
