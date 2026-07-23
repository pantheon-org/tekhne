//! End-to-end checks for the `journal lint` command: taxonomy resolution,
//! JSON output, the embedded-default fallback, and the `--strict` exit code.

use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_journal");

/// Write a dated entry with the given block-sequence tags under `root`.
fn write_entry(root: &Path, rel: &str, tags: &[&str]) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut doc = String::from("---\ntitle: T\ntags:\n");
    for tag in tags {
        doc.push_str(&format!("  - {tag}\n"));
    }
    doc.push_str("---\n\n# T\n");
    std::fs::write(path, doc).unwrap();
}

/// A corpus whose taxonomy facets only `general`, so `webhooks` is an unfaceted
/// threshold-crosser and `webhook` is a near-duplicate of it.
fn build_corpus(root: &Path) {
    std::fs::write(
        root.join("taxonomy.json"),
        r#"{
            "threshold": 2,
            "aliases": {},
            "facets": { "type": ["general"] },
            "pin": [],
            "suppress": [],
            "ticketPattern": "^x-[0-9]+$"
        }"#,
    )
    .unwrap();
    write_entry(root, "2026/07/2026-07-01-a.md", &["general", "webhooks"]);
    write_entry(root, "2026/07/2026-07-02-b.md", &["general", "webhook"]);
    write_entry(root, "2026/07/2026-07-03-c.md", &["general", "webhooks"]);
}

#[test]
fn json_report_flags_unfaceted_and_alias() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path());

    let out = Command::new(BIN)
        .args(["lint", "--json", "--root"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success(), "lint --json should exit 0 by default");

    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let unfaceted = report["unfaceted"].as_array().unwrap();
    assert!(
        unfaceted
            .iter()
            .any(|u| u["tag"] == "webhooks" && u["count"] == 2),
        "expected webhooks flagged as unfaceted, got {report}"
    );
    let aliases = report["alias_suggestions"].as_array().unwrap();
    assert!(
        !aliases.is_empty(),
        "expected at least one alias suggestion, got {report}"
    );
}

#[test]
fn strict_exits_non_zero_on_findings() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path());

    let status = Command::new(BIN)
        .args(["lint", "--strict", "--root"])
        .arg(tmp.path())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(2), "--strict should exit 2 on findings");
}

#[test]
fn falls_back_to_embedded_default_when_no_taxonomy() {
    let tmp = tempfile::tempdir().unwrap();
    // No taxonomy.json; the sole tag is faceted by the embedded default's type
    // facet, so a clean run using the embedded fallback is expected.
    write_entry(tmp.path(), "2026/07/2026-07-01-a.md", &["general"]);

    let out = Command::new(BIN)
        .args(["lint", "--root"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("embedded default taxonomy"),
        "expected embedded-default source in output, got: {stdout}"
    );
    assert!(stdout.contains("No tag issues found."), "got: {stdout}");
}
