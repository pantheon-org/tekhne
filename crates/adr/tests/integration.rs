//! End-to-end tests driving the public `adr` library the way the CLI does:
//! numbering across several ADRs, rendering, superseding, and installing the
//! embedded skill into a temporary agent directory.

use std::path::Path;

use adr_core::adr::{self, AdrDraft};
use adr_core::install_cmd::{run_install, InstallOptions, Selection};
use adr_core::skill_bundle;
use skill_install::env::Environment;
use skill_install::install::InstallMode;

#[test]
fn numbering_and_rendering_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let first = adr::create_new(root, "Adopt Rust for tooling", "2026-07-22").unwrap();
    let second = adr::create_new(root, "Use cargo-dist for releases", "2026-07-22").unwrap();
    let third = adr::create_new(root, "Bundle skills in binaries", "2026-07-22").unwrap();

    assert!(first.ends_with("0001-adopt-rust-for-tooling.md"));
    assert!(second.ends_with("0002-use-cargo-dist-for-releases.md"));
    assert!(third.ends_with("0003-bundle-skills-in-binaries.md"));

    let listed = adr::list(root).unwrap();
    assert_eq!(listed.len(), 3);
    assert_eq!(listed[0].number, 1);
    assert_eq!(listed[2].title, "Bundle skills in binaries");
    assert!(listed.iter().all(|e| e.status == "Proposed"));

    let body = std::fs::read_to_string(&first).unwrap();
    assert!(body.starts_with("# ADR-0001: Adopt Rust for tooling\n"));
    assert!(body.contains("**Date:** 2026-07-22"));
    assert!(body.trim_end().ends_with("- [Link to related ADRs]"));
}

#[test]
fn supersede_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    adr::create_new(root, "Original decision", "2026-07-22").unwrap();
    let (old_path, new_path) = adr::supersede(root, 1, "Revised decision", "2026-07-23").unwrap();

    let entries = adr::list(root).unwrap();
    assert_eq!(entries.len(), 2);

    let old = entries.iter().find(|e| e.number == 1).unwrap();
    assert_eq!(old.status, "Superseded by ADR-0002");
    assert_eq!(old.path, old_path);

    let new = entries.iter().find(|e| e.number == 2).unwrap();
    assert_eq!(new.status, "Accepted");
    assert_eq!(new.path, new_path);

    let new_body = std::fs::read_to_string(&new_path).unwrap();
    assert!(new_body.contains("Supersedes ADR-0001"));
}

#[test]
fn render_is_stable_for_a_fixed_draft() {
    let draft = AdrDraft {
        number: 42,
        title: "Deterministic output".to_string(),
        date: "2026-07-22".to_string(),
        status: "Proposed".to_string(),
        supersedes: None,
    };
    assert_eq!(adr::render(&draft), adr::render(&draft));
}

#[test]
fn skill_installs_into_a_temp_agent_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    let env = Environment::resolve(home.clone(), home.clone(), None, None, None);

    // Extract the embedded skill, then install it into an explicit agent.
    let bundle_root = tmp.path().join("bundle");
    let source = skill_bundle::materialise(&bundle_root).unwrap();
    assert!(source.join("SKILL.md").is_file());

    let opts = InstallOptions {
        selection: Selection::Explicit(vec!["claude-code".into()]),
        global: true,
        mode: InstallMode::Copy,
        dry_run: false,
    };
    let exists = |p: &Path| p.exists();
    let report = run_install(&opts, &env, &exists, &source).unwrap();

    assert_eq!(report.outcomes.len(), 1);
    let installed = report.outcomes[0].installed_path.as_ref().unwrap();
    assert!(installed.join("SKILL.md").is_file());
    assert_eq!(installed.file_name().unwrap(), "adr-creator");
}
