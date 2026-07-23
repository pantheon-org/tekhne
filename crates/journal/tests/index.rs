//! End-to-end checks for `pantheon-journal index`: dry-run output, writing the
//! NDJSON + markdown pair, and the `--validate` gate.

use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-journal");

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

/// A small corpus with a taxonomy and two dated entries.
fn build_corpus(root: &Path) {
    std::fs::write(
        root.join("taxonomy.json"),
        r#"{ "threshold": 2, "aliases": {}, "facets": { "type": ["troubleshooting", "general"], "tech": ["aws"] }, "pin": [], "suppress": [], "ticketPattern": "^x-[0-9]+$" }"#,
    )
    .unwrap();
    write(
        root,
        "2026/07/2026-07-09-newer.md",
        "---\ntitle: Newer\ntags:\n  - aws\n  - troubleshooting\n  - x-42\n---\n\n# Newer - July 9, 2026\n\n## Session Overview\n\nA newer entry.\n",
    );
    write(
        root,
        "2026/07/2026-07-01-older.md",
        "---\ntitle: Older\ntags:\n  - aws\n---\n\n# Older - July 1, 2026\n",
    );
}

#[test]
fn dry_run_prints_ndjson_and_markdown() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path());

    let out = Command::new(BIN)
        .args(["index", "--dry-run", "--root"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);

    // NDJSON: newest entry first, compact one-per-line.
    assert!(stdout.contains(r#""file":"2026/07/2026-07-09-newer.md""#));
    assert!(stdout.contains(r#""type":"troubleshooting""#));
    // Markdown view sections.
    assert!(stdout.contains("# Journal Index"));
    assert!(stdout.contains("2 entries"));
    assert!(stdout.contains("## Recent Entries"));
    assert!(stdout.contains("## By Ticket"));
    assert!(stdout.contains("### x-42 (1)"));
}

#[test]
fn writes_and_then_validates_the_pair() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path());
    let data = tmp.path().join("out/index.ndjson");
    let view = tmp.path().join("out/index.md");

    let status = Command::new(BIN)
        .args(["index", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .arg("--view")
        .arg(&view)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(data.is_file() && view.is_file());
    assert_eq!(std::fs::read_to_string(&data).unwrap().lines().count(), 2);

    // The freshly written NDJSON validates clean (files resolve under --root).
    let status = Command::new(BIN)
        .args(["index", "--validate", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn validate_rejects_broken_ndjson() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path());
    let data = tmp.path().join("bad.ndjson");
    // date does not match the slug prefix.
    std::fs::write(
        &data,
        "{\"file\":\"2026/07/2026-07-01-older.md\",\"files\":[\"2026/07/2026-07-01-older.md\"],\"slug\":\"2026-07-01-older\",\"date\":\"2026-07-02\",\"title\":\"T\",\"tags\":[],\"type\":\"general\",\"tickets\":[],\"authors\":[],\"status\":\"\",\"summary\":\"\"}\n",
    )
    .unwrap();

    let status = Command::new(BIN)
        .args(["index", "--validate", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .status()
        .unwrap();
    assert!(!status.success(), "broken ndjson must fail validation");
}
