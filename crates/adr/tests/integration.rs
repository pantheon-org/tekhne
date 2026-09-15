//! End-to-end tests driving the public `adr_core` library the way the CLI
//! does: the full lifecycle of a decision from creation through the
//! completeness gate to supersession, the catalogue derived from it, and
//! installing the embedded skill into a temporary agent directory.

use std::path::Path;

use adr_core::commands::{self, CreateArgs, UpdateArgs};
use adr_core::install_cmd::{run_install, InstallOptions, Selection};
use adr_core::record::Status;
use adr_core::{skill_bundle, store};
use skill_install::env::Environment;
use skill_install::install::InstallMode;

/// Enough real prose to satisfy the rubric's word-count rules.
const PROSE: &str = "Requests cross four services and we cannot follow a single one end to end \
    because each service writes its own log format with no shared correlation identifier, so \
    reconstructing a slow request means grepping four log stores by timestamp and guessing.";

/// Replace the template body with prose that scores 100.
fn fill(dir: &Path, slug: &str) {
    let mut record = store::load(dir, slug).expect("load the record just created");
    record.body = format!(
        "# {slug}\n\n## Problem Statement\n### Context\n{PROSE}\n\n\
         ## Decision Record\n### Options Considered\nA vendor agent, or OTLP behind a collector.\n\n\
         ### Chosen Solution\n{PROSE}\n\n### Rationale\n{PROSE}\n\n\
         ## Impact Assessment\n- **Performance**: about 1% CPU at 10% sampling\n\
         - **Security**: attributes scrubbed at the collector\n"
    );
    store::save(dir, &record).expect("save the filled record");
}

#[test]
fn the_full_lifecycle_end_to_end() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let dir = tmp.path().join("docs").join("adr");

    commands::init(&dir, tmp.path(), Some("demo"), false, false).expect("init");

    // Create, outside a repository, with an explicit slug.
    let args = CreateArgs {
        slug: Some("adopt-opentelemetry"),
        description: Some("Adopt OpenTelemetry for tracing"),
        ..Default::default()
    };
    commands::create(&dir, tmp.path(), "2026-09-15", &args).expect("create");
    assert!(dir.join("adopt-opentelemetry.md").is_file());

    // An untouched template must not pass the gate.
    let (_, verdict) = commands::check(&dir, "adopt-opentelemetry", false, false).expect("check");
    assert!(verdict.is_err(), "the template must not score as finished");
    assert!(
        commands::review(&dir, "adopt-opentelemetry", "2026-09-15T10:00:00Z").is_err(),
        "review must refuse an unfinished record"
    );

    // Fill it in, and the gate opens.
    fill(&dir, "adopt-opentelemetry");
    let (report, verdict) =
        commands::check(&dir, "adopt-opentelemetry", false, false).expect("check");
    assert!(verdict.is_ok(), "{report}");
    assert!(report.contains("Score: 100/100"), "{report}");

    commands::review(&dir, "adopt-opentelemetry", "2026-09-15T10:00:00Z").expect("review");
    commands::update(
        &dir,
        "adopt-opentelemetry",
        "2026-09-15T11:00:00Z",
        &UpdateArgs {
            status: Some("accepted"),
            tags: Some("tracing,observability"),
            ..Default::default()
        },
    )
    .expect("accept");

    let accepted = store::load(&dir, "adopt-opentelemetry").expect("load");
    assert_eq!(accepted.meta.status, Status::Accepted);
    assert_eq!(accepted.meta.tags, vec!["tracing", "observability"]);
    assert_eq!(
        accepted.meta.history.len(),
        2,
        "proposed to review-requested to accepted"
    );

    // Supersede it with a second decision; both ends of the chain are linked.
    let replacement = CreateArgs {
        slug: Some("adopt-grafana-tempo"),
        description: Some("Adopt Grafana Tempo as the tracing backend"),
        ..Default::default()
    };
    commands::create(&dir, tmp.path(), "2026-09-16", &replacement).expect("create replacement");
    commands::update(
        &dir,
        "adopt-opentelemetry",
        "2026-09-16T09:00:00Z",
        &UpdateArgs {
            superseded_by: Some("adopt-grafana-tempo"),
            ..Default::default()
        },
    )
    .expect("supersede");

    let old = store::load(&dir, "adopt-opentelemetry").expect("load old");
    assert_eq!(old.meta.status, Status::Superseded);
    assert_eq!(
        old.meta.superseded_by.as_deref(),
        Some("adopt-grafana-tempo")
    );
    let new = store::load(&dir, "adopt-grafana-tempo").expect("load new");
    assert_eq!(new.meta.supersedes.as_deref(), Some("adopt-opentelemetry"));

    // The prose of the superseded record is untouched by any of that.
    assert!(
        old.body.contains(PROSE),
        "superseding must not rewrite the decision text"
    );

    // Listing, filtering and the catalogue all agree.
    let listed = commands::list(&dir, None, None, false).expect("list");
    assert!(listed.contains("2 ADR(s)"), "{listed}");
    let superseded = commands::list(&dir, Some("superseded"), None, false).expect("list");
    assert!(superseded.contains("adopt-opentelemetry"), "{superseded}");

    commands::index_cmd(&dir, "2026-09-16").expect("index");
    let catalogue = std::fs::read_to_string(dir.join("index.md")).expect("catalogue");
    assert!(catalogue.contains("## Superseded"), "{catalogue}");
    assert!(
        catalogue.contains("superseded by `adopt-grafana-tempo`"),
        "{catalogue}"
    );

    // The catalogue is not itself a record.
    assert_eq!(store::load_all(&dir).expect("load all").len(), 2);
}

#[test]
fn status_json_is_stable_for_a_fixed_collection() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let dir = tmp.path().join("docs").join("adr");
    commands::init(&dir, tmp.path(), Some("demo"), false, false).expect("init");

    for slug in ["one", "two"] {
        let args = CreateArgs {
            slug: Some(slug),
            ..Default::default()
        };
        commands::create(&dir, tmp.path(), "2026-09-15", &args).expect("create");
    }

    let first = commands::status(&dir, tmp.path(), true).expect("status");
    let second = commands::status(&dir, tmp.path(), true).expect("status");
    assert_eq!(first, second, "JSON output must be deterministic");

    let value: serde_json::Value = serde_json::from_str(&first).expect("valid json");
    assert_eq!(value["total"], 2);
    assert_eq!(value["by_status"]["proposed"], 2);
}

#[test]
fn a_record_round_trips_through_the_filesystem_unchanged() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let dir = tmp.path().join("docs").join("adr");
    let args = CreateArgs {
        slug: Some("round-trip"),
        description: Some("Round trip"),
        ..Default::default()
    };
    commands::create(&dir, tmp.path(), "2026-09-15", &args).expect("create");

    let loaded = store::load(&dir, "round-trip").expect("load");
    let rendered = loaded.render().expect("render");
    let reloaded = adr_core::record::Record::parse("round-trip", &rendered).expect("parse");
    assert_eq!(reloaded, loaded);
}

#[test]
fn skill_installs_into_a_temp_agent_dir() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).expect("create home");
    let env = Environment::resolve(home.clone(), home.clone(), None, None, None);

    // Extract the embedded skill, then install it into an explicit agent.
    let bundle_root = tmp.path().join("bundle");
    let source = skill_bundle::materialise(&bundle_root).expect("materialise the bundled skill");
    assert!(source.join("SKILL.md").is_file());

    let opts = InstallOptions {
        selection: Selection::Explicit(vec!["claude-code".into()]),
        global: true,
        mode: InstallMode::Copy,
        dry_run: false,
    };
    let exists = |p: &Path| p.exists();
    let report = run_install(&opts, &env, &exists, &source).expect("install");

    assert_eq!(report.outcomes.len(), 1);
    let installed = report.outcomes[0]
        .installed_path
        .as_ref()
        .expect("an installed path");
    assert!(installed.join("SKILL.md").is_file());
    assert_eq!(
        installed.file_name().expect("a directory name"),
        "adr-creator"
    );
}
