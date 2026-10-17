//! End-to-end checks for `--entries-dir`: a corpus whose year directories live
//! under a wrapper directory (`entries/2026/...`) must scan, index, lint,
//! backfill and round-trip, with the wrapper prefix kept in the stored `file`
//! field. The default `.` must keep finding year directories at the root.

use std::path::Path;
use std::process::Command;

use journal::validate::{validate, Outcome};

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-journal");

const ENTRY_DIR: &str = "2026/07-July/22-Wednesday";

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

fn entry_doc(title: &str, tags: &[&str]) -> String {
    let mut doc = format!("---\ntitle: {title}\ntags:\n");
    for tag in tags {
        doc.push_str(&format!("  - {tag}\n"));
    }
    doc.push_str(&format!("---\n\n# {title} - July 22, 2026\n"));
    doc
}

/// A taxonomy plus two entries under `<wrapper>/2026/07-July/22-Wednesday/`.
/// An empty `wrapper` puts the year directory at the root.
fn build_corpus(root: &Path, wrapper: &str) {
    write(
        root,
        "taxonomy.json",
        r#"{ "threshold": 2, "aliases": {}, "facets": { "type": ["general"] }, "pin": [], "suppress": [], "ticketPattern": "^x-[0-9]+$" }"#,
    );
    let prefix = if wrapper.is_empty() {
        String::new()
    } else {
        format!("{wrapper}/")
    };
    write(
        root,
        &format!("{prefix}{ENTRY_DIR}/2026-07-22-first.md"),
        &entry_doc("First", &["general", "webhooks"]),
    );
    write(
        root,
        &format!("{prefix}{ENTRY_DIR}/2026-07-22-second.md"),
        &entry_doc("Second", &["general", "webhooks"]),
    );
}

fn run(args: &[&str], root: &Path) -> std::process::Output {
    Command::new(BIN)
        .args(args)
        .arg("--root")
        .arg(root)
        .output()
        .unwrap()
}

#[test]
fn index_keeps_the_wrapper_prefix_and_validates() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");
    let data = tmp.path().join("docs/journal-index.ndjson");
    let view = tmp.path().join("docs/journal-index.md");

    let out = Command::new(BIN)
        .args(["index", "--entries-dir", "entries", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .arg("--view")
        .arg(&view)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let ndjson = std::fs::read_to_string(&data).unwrap();
    assert_eq!(ndjson.lines().count(), 2);
    assert!(ndjson.contains(&format!(
        r#""file":"entries/{ENTRY_DIR}/2026-07-22-first.md""#
    )));
    let view = std::fs::read_to_string(&view).unwrap();
    assert!(
        view.contains(&format!("](../entries/{ENTRY_DIR}/2026-07-22-")),
        "index links must keep the entries/ prefix:\n{view}"
    );

    let status = Command::new(BIN)
        .args(["index", "--validate", "--entries-dir", "entries", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .status()
        .unwrap();
    assert!(status.success(), "a fresh index must validate");
}

#[test]
fn wrapper_corpus_is_invisible_without_the_option() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");

    let out = run(&["index", "--dry-run"], tmp.path());
    assert!(out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("0 entries"),
        "the default `.` must not look inside entries/"
    );
}

#[test]
fn default_dot_still_finds_root_year_directories() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "");

    let out = run(&["index", "--dry-run"], tmp.path());
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("2 entries"), "{stdout}");
    assert!(stdout.contains(&format!(r#""file":"{ENTRY_DIR}/2026-07-22-first.md""#)));
}

#[test]
fn lint_reads_the_wrapper() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");

    let out = run(&["lint", "--json", "--entries-dir", "entries"], tmp.path());
    assert!(out.status.success());
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(
        report["unfaceted"]
            .as_array()
            .unwrap()
            .iter()
            .any(|u| u["tag"] == "webhooks" && u["count"] == 2),
        "lint must see both wrapped entries, got {report}"
    );
}

#[test]
fn backfill_reads_and_writes_inside_the_wrapper() {
    let tmp = tempfile::tempdir().unwrap();
    write(
        tmp.path(),
        &format!("entries/{ENTRY_DIR}/2026-07-22-bare.md"),
        "# Bare Entry\n\nBody.\n",
    );

    let out = run(&["backfill", "--entries-dir", "entries"], tmp.path());
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let doc = std::fs::read_to_string(
        tmp.path()
            .join(format!("entries/{ENTRY_DIR}/2026-07-22-bare.md")),
    )
    .unwrap();
    assert!(
        doc.starts_with("---\n"),
        "frontmatter should be injected:\n{doc}"
    );
}

#[test]
fn taxonomy_sync_scans_the_wrapper() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");

    let out = run(
        &["taxonomy", "sync", "--check", "--entries-dir", "entries"],
        tmp.path(),
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("webhooks"),
        "sync must see the wrapped entries' unfaceted tag:\n{text}"
    );
}

#[test]
fn new_creates_under_the_wrapper_and_the_entry_round_trips() {
    let tmp = tempfile::tempdir().unwrap();

    let out = Command::new(BIN)
        .args([
            "new",
            "--title",
            "Wrapped",
            "--entries-dir",
            "entries",
            "--dir",
        ])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let created = String::from_utf8_lossy(&out.stdout);
    let path = created.trim().strip_prefix("Created ").unwrap();
    let rel = Path::new(path).strip_prefix(tmp.path()).unwrap();
    assert!(
        rel.starts_with("entries"),
        "entry not under entries/: {rel:?}"
    );
    assert_eq!(validate(Path::new(path)), Ok(Outcome::Validated));

    // The entry it just wrote is found by the index under the same option.
    let out = Command::new(BIN)
        .args(["index", "--dry-run", "--entries-dir", "entries", "--root"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("\"file\":\"entries/"));
}
