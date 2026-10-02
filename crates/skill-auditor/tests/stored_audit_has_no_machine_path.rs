//! A stored audit, and `--json` output, name the skill by its repo-relative key
//! (for example `group/demo`), never by the absolute path of the checkout it
//! was run from. `evaluate` always did this; `batch` did not, so audits stored
//! from a temporary worktree carried a developer's local path.

use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-skill-auditor");

const SKILL: &str = "---\nname: demo\ndescription: A demo skill used by the path tests. Use when testing.\n---\n\n# Demo\n\nShort body.\n";

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .arg("--repo-root")
        .arg(root)
        .output()
        .expect("run auditor")
}

fn repo() -> tempfile::TempDir {
    let root = tempdir().unwrap();
    std::fs::create_dir_all(root.path().join(".git")).unwrap();
    let dir = root.path().join("skills/group/demo");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("SKILL.md"), SKILL).unwrap();
    root
}

fn read_stored(root: &Path, file: &str) -> String {
    let audits = root.join("skills/group/demo/.audits");
    let dated = std::fs::read_dir(&audits)
        .unwrap()
        .flatten()
        .next()
        .expect("a dated folder")
        .path();
    std::fs::read_to_string(dated.join(file)).unwrap()
}

#[test]
fn batch_store_records_the_relative_key_in_every_file() {
    let root = repo();
    assert!(run(root.path(), &["batch", "group/demo", "--store"])
        .status
        .success());
    let tmp = root.path().to_string_lossy().into_owned();
    for file in ["audit.json", "Analysis.md", "Remediation.md"] {
        let text = read_stored(root.path(), file);
        assert!(
            !text.contains(&tmp),
            "{file} contains the checkout path:\n{text}"
        );
        assert!(
            !text.contains("/SKILL.md"),
            "{file} still names SKILL.md:\n{text}"
        );
    }
    let audit: serde_json::Value =
        serde_json::from_str(&read_stored(root.path(), "audit.json")).unwrap();
    assert_eq!(audit["skill"], "group/demo");
}

#[test]
fn batch_json_names_the_skill_by_its_relative_key() {
    let root = repo();
    let out = run(root.path(), &["batch", "group/demo", "--json"]);
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v[0]["skill"], "group/demo");
}

#[test]
fn a_path_argument_gives_the_same_key_as_the_relative_name() {
    let root = repo();
    let abs = root.path().join("skills/group/demo");
    let out = run(root.path(), &["batch", abs.to_str().unwrap(), "--json"]);
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v[0]["skill"], "group/demo");
}

#[test]
fn evaluate_still_records_the_relative_key() {
    let root = repo();
    assert!(run(root.path(), &["evaluate", "group/demo", "--store"])
        .status
        .success());
    let audit: serde_json::Value =
        serde_json::from_str(&read_stored(root.path(), "audit.json")).unwrap();
    assert_eq!(audit["skill"], "group/demo");
}
