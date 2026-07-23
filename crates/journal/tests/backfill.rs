//! End-to-end checks for `pantheon-journal backfill`: it injects missing
//! frontmatter into dated entries and honours `--dry-run`.

use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_pantheon-journal");

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

#[test]
fn injects_missing_frontmatter() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // No frontmatter; the H1 supplies the title, the filename supplies the date.
    write(
        root,
        "2026/07/2026-07-01-alpha.md",
        "# Alpha Entry\n\ntext\n",
    );

    let status = Command::new(BIN)
        .args(["backfill", "--root"])
        .arg(root)
        .status()
        .unwrap();
    assert!(status.success());

    let out = std::fs::read_to_string(root.join("2026/07/2026-07-01-alpha.md")).unwrap();
    assert!(out.starts_with("---\n"));
    assert!(out.contains("title: \"Alpha Entry\""));
    assert!(out.contains("date: 2026-07-01"));
    assert!(out.contains("# Alpha Entry"));
}

#[test]
fn dry_run_leaves_files_untouched() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let original = "# Beta\n";
    write(root, "2026/07/2026-07-02-beta.md", original);

    let out = Command::new(BIN)
        .args(["backfill", "--dry-run", "--root"])
        .arg(root)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("would inject"));
    // Unchanged on disk.
    assert_eq!(
        std::fs::read_to_string(root.join("2026/07/2026-07-02-beta.md")).unwrap(),
        original
    );
}
