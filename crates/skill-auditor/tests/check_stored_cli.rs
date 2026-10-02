//! End to end: the `check-stored` exit codes and output, on a throwaway repo.
//! Exit 0 = every stored audit is current, 1 = stale or missing, 2 = error.

use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-skill-auditor");

const SKILL: &str = "---\nname: demo\ndescription: A demo skill used by the check-stored tests. Use when testing.\n---\n\n# Demo\n\nShort body.\n";

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .arg("--repo-root")
        .arg(root)
        .output()
        .expect("run auditor")
}

fn code(o: &Output) -> i32 {
    o.status.code().expect("exit code")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn repo_with_skill(body: &str) -> tempfile::TempDir {
    let root = tempdir().unwrap();
    std::fs::create_dir_all(root.path().join(".git")).unwrap();
    let dir = root.path().join("skills/group/demo");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("SKILL.md"), body).unwrap();
    root
}

#[test]
fn missing_audit_exits_1_and_names_the_skill() {
    let root = repo_with_skill(SKILL);
    let out = run(root.path(), &["check-stored", "group/demo"]);
    assert_eq!(code(&out), 1);
    assert!(
        stdout(&out).contains("STALE group/demo: no stored audit"),
        "{}",
        stdout(&out)
    );
}

#[test]
fn freshly_stored_audit_exits_0_and_prints_nothing() {
    let root = repo_with_skill(SKILL);
    assert_eq!(
        code(&run(root.path(), &["batch", "group/demo", "--store"])),
        0
    );
    let out = run(root.path(), &["check-stored", "group/demo"]);
    assert_eq!(code(&out), 0, "{}", stdout(&out));
    assert_eq!(stdout(&out), "");
}

#[test]
fn changing_the_skill_after_storing_makes_the_audit_stale() {
    let root = repo_with_skill(SKILL);
    assert_eq!(
        code(&run(root.path(), &["batch", "group/demo", "--store"])),
        0
    );
    let richer = format!(
        "{SKILL}\n## When to Use\n\n- one\n- two\n- three\n\n## When Not to Use\n\n- a\n- b\n- c\n\n## Philosophy\n\n- x\n- y\n- z\n\n```bash\n./run.sh\n```\n\n```bash\n./run.sh two\n```\n\n```bash\n./run.sh three\n```\n\n```bash\n./run.sh four\n```\n\n```bash\n./run.sh five\n```\n\n```bash\n./run.sh six\n```\n"
    );
    std::fs::write(root.path().join("skills/group/demo/SKILL.md"), richer).unwrap();
    let out = run(root.path(), &["check-stored", "group/demo"]);
    assert_eq!(code(&out), 1);
    let text = stdout(&out);
    assert!(text.starts_with("STALE group/demo: "), "{text}");
    assert!(
        text.contains("grade changed")
            || text.contains("score changed")
            || text.contains("dimension changed"),
        "{text}"
    );
}

#[test]
fn unknown_skill_is_an_error_not_stale() {
    let root = repo_with_skill(SKILL);
    let out = run(root.path(), &["check-stored", "group/does-not-exist"]);
    assert_eq!(code(&out), 2);
}

#[test]
fn malformed_stored_audit_is_an_error_not_stale() {
    let root = repo_with_skill(SKILL);
    let dir = root.path().join("skills/group/demo/.audits/2026-10-02");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("audit.json"), "{ not json").unwrap();
    let out = run(root.path(), &["check-stored", "group/demo"]);
    assert_eq!(code(&out), 2);
    assert!(String::from_utf8_lossy(&out.stderr).contains("audit.json"));
}

#[test]
fn an_error_wins_over_a_stale_result_in_the_same_run() {
    let root = repo_with_skill(SKILL);
    let out = run(
        root.path(),
        &["check-stored", "group/demo", "group/does-not-exist"],
    );
    assert_eq!(code(&out), 2);
    assert!(
        stdout(&out).contains("STALE group/demo: no stored audit"),
        "{}",
        stdout(&out)
    );
}

#[test]
fn json_output_has_the_contracted_fields() {
    let root = repo_with_skill(SKILL);
    let out = run(root.path(), &["check-stored", "group/demo", "--json"]);
    assert_eq!(code(&out), 1);
    let v: serde_json::Value = serde_json::from_str(&stdout(&out)).expect("json");
    let first = &v.as_array().expect("array")[0];
    for key in [
        "skill",
        "reason",
        "storedGrade",
        "currentGrade",
        "storedTotal",
        "currentTotal",
    ] {
        assert!(first.get(key).is_some(), "missing {key} in {first}");
    }
    assert_eq!(first["skill"], "group/demo");
    assert!(first["storedTotal"].is_null());
}

#[test]
fn json_is_an_empty_array_when_everything_is_current() {
    let root = repo_with_skill(SKILL);
    assert_eq!(
        code(&run(root.path(), &["batch", "group/demo", "--store"])),
        0
    );
    let out = run(root.path(), &["check-stored", "group/demo", "--json"]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out).trim(), "[]");
}
