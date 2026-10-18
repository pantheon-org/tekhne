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

#[test]
fn a_missing_entries_dir_fails_and_leaves_the_index_untouched() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");
    let data = tmp.path().join("docs/journal-index.ndjson");
    write(tmp.path(), "docs/journal-index.ndjson", "committed\n");
    let view = tmp.path().join("docs/journal-index.md");

    for args in [
        vec![
            "index",
            "--data",
            data.to_str().unwrap(),
            "--view",
            view.to_str().unwrap(),
        ],
        vec!["lint"],
        vec!["backfill"],
        vec!["taxonomy", "sync", "--check"],
    ] {
        let out = Command::new(BIN)
            .args(&args)
            .args(["--entries-dir", "entires", "--root"])
            .arg(tmp.path())
            .output()
            .unwrap();
        assert!(!out.status.success(), "{args:?} must fail on a typo'd dir");
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("entires"),
            "{args:?} stderr should name the directory"
        );
    }
    assert_eq!(std::fs::read_to_string(&data).unwrap(), "committed\n");
}

#[test]
fn an_entries_dir_with_parent_or_absolute_components_is_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");
    let outside = tmp.path().join("entries");

    for bad in [
        "entries/../entries",
        "../entries",
        outside.to_str().unwrap(),
    ] {
        let out = Command::new(BIN)
            .args(["lint", "--entries-dir", bad, "--root"])
            .arg(tmp.path())
            .output()
            .unwrap();
        assert!(!out.status.success(), "{bad:?} must be rejected");
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("--entries-dir"),
            "{bad:?}: stderr should explain the rejection"
        );
    }

    // A plain `./entries` is still fine.
    let out = run(&["lint", "--entries-dir", "./entries"], tmp.path());
    assert!(out.status.success());
}

/// An unreadable subdirectory must stop the corpus commands: skipping it
/// would write a partial index and report success.
#[cfg(unix)]
#[test]
fn an_unreadable_subdirectory_fails_and_leaves_the_index_untouched() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");
    let data = tmp.path().join("docs/journal-index.ndjson");
    let view = tmp.path().join("docs/journal-index.md");
    write(tmp.path(), "docs/journal-index.ndjson", "committed\n");
    let locked = tmp.path().join("entries/2026/07-July");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).unwrap();
    // Running as root would read it anyway; nothing to prove then.
    let readable = std::fs::read_dir(&locked).is_ok();

    let out = Command::new(BIN)
        .args(["index", "--entries-dir", "entries", "--root"])
        .arg(tmp.path())
        .arg("--data")
        .arg(&data)
        .arg("--view")
        .arg(&view)
        .output()
        .unwrap();
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755)).unwrap();
    if readable {
        return;
    }
    assert!(!out.status.success(), "a partial scan must not succeed");
    assert!(String::from_utf8_lossy(&out.stderr).contains("07-July"));
    assert_eq!(std::fs::read_to_string(&data).unwrap(), "committed\n");
    assert!(!view.exists());
}

/// A symlink that points back at one of its own ancestors would make the
/// walk recurse forever; it must be reported instead.
#[cfg(unix)]
#[test]
fn a_symlink_loop_fails_instead_of_recursing_forever() {
    let tmp = tempfile::tempdir().unwrap();
    build_corpus(tmp.path(), "entries");
    let month = tmp.path().join("entries/2026/07-July");
    std::os::unix::fs::symlink(tmp.path().join("entries/2026"), month.join("loop")).unwrap();

    let out = run(&["lint", "--entries-dir", "entries"], tmp.path());
    assert!(!out.status.success(), "a symlink loop must fail");
    assert!(String::from_utf8_lossy(&out.stderr).contains("loop"));
}

#[test]
fn library_scan_and_backfill_report_an_unreadable_entries_dir() {
    use journal::taxonomy::Taxonomy;

    let tmp = tempfile::tempdir().unwrap();
    let missing = Path::new("entires");
    let taxonomy = Taxonomy::from_json(
        r#"{ "threshold": 2, "aliases": {}, "facets": {}, "pin": [], "suppress": [], "ticketPattern": "^x-[0-9]+$" }"#,
        "test",
    )
    .unwrap();

    let err = journal::scan::scan_entries_in(tmp.path(), missing, &taxonomy).unwrap_err();
    assert!(err.contains("entires"), "{err}");
    let err = journal::backfill::backfill_in(tmp.path(), missing, true).unwrap_err();
    assert!(err.to_string().contains("entires"), "{err}");
}
