//! `batch --details-markdown` writes the pull request detail blocks without
//! changing anything else the command does.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const SKILL: &str = "---\nname: demo\ndescription: A deliberately thin skill so that it grades below A.\n---\n\n# Demo\n\nBody.\n";

fn repo_with_skills(names: &[&str]) -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("temp repo");
    for name in names {
        let dir = repo.path().join("skills").join(name);
        fs::create_dir_all(&dir).expect("skill dir");
        fs::write(dir.join("SKILL.md"), SKILL).expect("skill file");
    }
    repo
}

fn batch(repo: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pantheon-skill-auditor"))
        .arg("batch")
        .args(["demo/one", "demo/two"])
        .arg("--repo-root")
        .arg(repo)
        .args(extra)
        .output()
        .expect("run pantheon-skill-auditor")
}

#[test]
fn stdout_and_exit_code_are_identical_with_and_without_the_flag_in_json_mode() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let out_file = repo.path().join("details.md");
    let plain = batch(repo.path(), &["--json"]);
    let with_flag = batch(
        repo.path(),
        &[
            "--json",
            "--details-markdown",
            out_file.to_str().expect("utf-8"),
        ],
    );
    assert_eq!(plain.stdout, with_flag.stdout);
    assert_eq!(plain.status.code(), with_flag.status.code());
    assert!(out_file.exists(), "details file written");
}

#[test]
fn stdout_and_exit_code_are_identical_with_and_without_the_flag_in_table_mode() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let out_file = repo.path().join("details.md");
    let plain = batch(repo.path(), &[]);
    let with_flag = batch(
        repo.path(),
        &["--details-markdown", out_file.to_str().expect("utf-8")],
    );
    assert_eq!(plain.stdout, with_flag.stdout);
    assert_eq!(plain.status.code(), with_flag.status.code());
    assert!(out_file.exists(), "details file written without --json");
}

#[test]
fn the_file_holds_one_block_per_skill_below_a() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let out_file = repo.path().join("details.md");
    batch(
        repo.path(),
        &["--details-markdown", out_file.to_str().expect("utf-8")],
    );
    let details = fs::read_to_string(&out_file).expect("details file");
    assert_eq!(details.matches("<details>").count(), 2, "{details}");
    assert!(details.contains("<code>demo/one</code>"), "{details}");
    assert!(details.contains("<code>demo/two</code>"), "{details}");
}

#[test]
fn the_file_is_written_even_when_fail_below_makes_the_run_fail() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let out_file = repo.path().join("details.md");
    let run = batch(
        repo.path(),
        &[
            "--fail-below",
            "A+",
            "--details-markdown",
            out_file.to_str().expect("utf-8"),
        ],
    );
    assert_eq!(run.status.code(), Some(1));
    assert!(
        out_file.exists(),
        "details file written before the gate exit"
    );
}

#[test]
fn a_file_that_cannot_be_written_is_a_warning_not_a_failure() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let plain = batch(repo.path(), &["--json"]);
    // A directory cannot be opened for writing as a file.
    let unwritable = repo.path().join("skills");
    let with_flag = batch(
        repo.path(),
        &[
            "--json",
            "--details-markdown",
            unwritable.to_str().expect("utf-8"),
        ],
    );
    assert_eq!(plain.stdout, with_flag.stdout);
    assert_eq!(plain.status.code(), with_flag.status.code());
    let stderr = String::from_utf8_lossy(&with_flag.stderr);
    assert!(stderr.contains("warning: details"), "{stderr}");
}

#[test]
fn a_small_budget_leaves_only_the_omitted_line() {
    let repo = repo_with_skills(&["demo/one", "demo/two"]);
    let out_file = repo.path().join("details.md");
    batch(
        repo.path(),
        &[
            "--details-markdown",
            out_file.to_str().expect("utf-8"),
            "--details-budget",
            "50",
        ],
    );
    let details = fs::read_to_string(&out_file).expect("details file");
    assert_eq!(details.matches("<details>").count(), 0, "{details}");
    assert!(
        details.contains("2 more skills below A not shown"),
        "{details}"
    );
}
