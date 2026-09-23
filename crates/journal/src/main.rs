//! `pantheon-journal` CLI: create structured entries (`new`), check them
//! (`validate <file>`), lint corpus tags against the taxonomy (`lint`),
//! promote a tag into a facet (`taxonomy add`), generate the browse index
//! (`index`) or a separate knowledge-base index (`kb-index`), backfill
//! missing frontmatter (`backfill`), and install the bundled companion skill
//! (`skill install`).

use std::path::{Path, PathBuf};
use std::process;

use clap::{Args, Parser, Subcommand, ValueEnum};
use journal::archive_media::{self, ArchiveMediaArgs as ArchiveMediaCoreArgs};
use journal::backfill;
use journal::date::Timestamp;
use journal::entry::{EntrySpec, EntryType};
use journal::index;
use journal::install_cmd::{self, InstallOptions, Selection, UninstallOptions};
use journal::kb;
use journal::lint;
use journal::scan;
use journal::skill_bundle;
use journal::taxonomy::{Taxonomy, TaxonomySource};
use journal::taxonomy_sync::{check_append_only, classify_tag, ClassifyConfidence};
use journal::validate::{self, Outcome};
use skill_install::agents::all as all_agents;
use skill_install::env::Environment;
use skill_install::install::InstallMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "pantheon-journal",
    version = VERSION,
    about = "Create and validate structured journal entries",
    long_about = "pantheon-journal creates timestamped markdown entries with YAML frontmatter, triple-synced dates, and template-based sections, then validates them for compliance. It promotes the journal-entry-creator skill into a self-contained CLI."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new journal entry.
    New(NewArgs),
    /// Validate an existing journal entry.
    Validate {
        /// Path to the entry markdown file.
        file: String,
    },
    /// Lint corpus tags against the taxonomy (advisory).
    Lint(LintArgs),
    /// Generate the browse index (NDJSON source of truth + markdown view).
    Index(IndexArgs),
    /// Generate the knowledge-base browse index (a separate, frontmatter-only
    /// corpus grouped by top-level directory, not by date).
    KbIndex(KbIndexArgs),
    /// Backfill missing frontmatter (title, date) across existing entries.
    Backfill(BackfillArgs),
    /// Download already-discovered media URLs (found via a browser) into an
    /// existing entry's assets/ directory, at original resolution.
    ArchiveMedia(ArchiveMediaCliArgs),
    /// Manage the bundled companion skill.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
    /// Manage the tag taxonomy (write path; `lint` is the read-only/advisory check).
    Taxonomy {
        #[command(subcommand)]
        action: TaxonomyAction,
    },
}

#[derive(Subcommand)]
enum TaxonomyAction {
    /// Promote a tag into a facet's canonical list.
    Add(TaxonomyAddArgs),
    /// Auto-facet confidently classifiable unfaceted tags; report the rest.
    Sync(TaxonomySyncArgs),
}

#[derive(Args)]
struct TaxonomySyncArgs {
    /// Journal root to discover `taxonomy.json` at.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Explicit taxonomy file to read and write, overriding `<root>/taxonomy.json`.
    #[arg(long)]
    taxonomy: Option<PathBuf>,
    /// Never write; fail if any tag would still be unfaceted or any
    /// append-only violation is found (for CI's check step).
    #[arg(long = "check")]
    check_only: bool,
}

#[derive(Args)]
struct TaxonomyAddArgs {
    /// The tag to promote (must not already belong to any facet).
    tag: String,
    /// The facet to add it to (must already exist in the taxonomy).
    #[arg(long)]
    facet: String,
    /// Journal root to discover `taxonomy.json` at (also where a new one is
    /// written if none exists yet).
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Explicit taxonomy file to read and write, overriding `<root>/taxonomy.json`.
    #[arg(long)]
    taxonomy: Option<PathBuf>,
    /// Show the resulting taxonomy without writing it.
    #[arg(long = "dry-run")]
    dry_run: bool,
}

#[derive(Args)]
struct NewArgs {
    /// The entry type to create.
    #[arg(long = "type", value_enum, default_value_t = EntryType::Journal)]
    entry_type: EntryType,
    /// The entry title (used in the H1 and to derive the slug).
    #[arg(long)]
    title: Option<String>,
    /// Base directory the `YYYY/MM/` tree is created under.
    #[arg(long, default_value = ".")]
    dir: PathBuf,
    /// Author recorded in the frontmatter.
    #[arg(long, default_value = "Unknown")]
    author: String,
    /// Issue-tracker key: the refinement target, the ticket being kicked off,
    /// or a troubleshooting slug prefix.
    #[arg(long)]
    ticket: Option<String>,
    /// Source URL, recorded for article summaries.
    #[arg(long)]
    source: Option<String>,
}

#[derive(Args)]
struct LintArgs {
    /// Restrict linting to entries whose path contains one of these substrings.
    /// Empty lints the whole corpus. Tag frequency is always corpus-wide.
    files: Vec<String>,
    /// Journal root to scan for entries and a `taxonomy.json`.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Explicit taxonomy file, overriding `<root>/taxonomy.json` and the
    /// embedded default.
    #[arg(long)]
    taxonomy: Option<PathBuf>,
    /// Emit the lint report as JSON instead of human-readable text.
    #[arg(long)]
    json: bool,
    /// Exit non-zero when any suggestion or unfaceted tag is found (for CI).
    #[arg(long)]
    strict: bool,
}

#[derive(Args)]
struct IndexArgs {
    /// Journal root to scan for entries and a `taxonomy.json`.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Explicit taxonomy file, overriding `<root>/taxonomy.json` and the default.
    #[arg(long)]
    taxonomy: Option<PathBuf>,
    /// NDJSON output path (default: docs/journal-index.ndjson).
    #[arg(long)]
    data: Option<PathBuf>,
    /// Markdown output path (default: docs/journal-index.md).
    #[arg(long)]
    view: Option<PathBuf>,
    /// Print the generated index to stdout instead of writing files.
    #[arg(long = "dry-run")]
    dry_run: bool,
    /// Check the committed NDJSON is byte-identical to a fresh regeneration;
    /// do not write. Any difference at all -- a stale entry, a missing new
    /// one, a manual edit -- is stale.
    #[arg(long)]
    validate: bool,
}

#[derive(Args)]
struct KbIndexArgs {
    /// Directory scanned for articles.
    #[arg(long, default_value = kb::KB_INDEX_ROOT)]
    root: PathBuf,
    /// NDJSON output path.
    #[arg(long)]
    data: Option<PathBuf>,
    /// Markdown output path.
    #[arg(long)]
    view: Option<PathBuf>,
    /// Print the generated index to stdout instead of writing files.
    #[arg(long = "dry-run")]
    dry_run: bool,
    /// Check the committed ndjson is byte-identical to a fresh regeneration;
    /// do not write. Same semantics as `index --validate`: any difference at
    /// all, including field reordering, is stale.
    #[arg(long)]
    validate: bool,
}

#[derive(Args)]
struct BackfillArgs {
    /// Journal root to scan for dated entries.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Preview changes without writing any files.
    #[arg(long = "dry-run")]
    dry_run: bool,
}

#[derive(Args)]
struct ArchiveMediaCliArgs {
    /// The entry's dated slug, e.g. `2026-09-22-jev-ai-memory-longmemeval-benchmark`.
    slug: String,
    /// Media URLs to download, already discovered (e.g. via a browser).
    urls: Vec<String>,
    /// Filename stem for the URL at the same position (repeatable; must be
    /// given once per URL, or not at all).
    #[arg(short = 'n', long = "name")]
    name: Vec<String>,
    /// Filename prefix.
    #[arg(short = 'p', long = "prefix", default_value = "media")]
    prefix: String,
    /// Overwrite a destination file that already exists.
    #[arg(short = 'f', long)]
    force: bool,
    /// Journal root the entry lives under.
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

#[derive(Subcommand)]
enum SkillAction {
    /// Install the bundled skill into detected (or selected) agent directories.
    Install(SkillInstallArgs),
    /// Remove the bundled skill from detected (or selected) agent directories.
    Uninstall(SkillUninstallArgs),
}

/// How the bundled skill is placed into each target directory.
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ModeArg {
    /// Recursively copy the skill (self-contained; the default).
    Copy,
    /// Symlink each target at a per-user extraction of the skill.
    Symlink,
}

impl From<ModeArg> for InstallMode {
    fn from(mode: ModeArg) -> Self {
        match mode {
            ModeArg::Copy => InstallMode::Copy,
            ModeArg::Symlink => InstallMode::Symlink,
        }
    }
}

#[derive(Args)]
struct SkillInstallArgs {
    /// Target a specific agent by slug (repeatable). Defaults to every
    /// detected agent.
    #[arg(long = "agent", value_name = "NAME")]
    agent: Vec<String>,
    /// Install into every agent in the universal list, not only detected ones.
    #[arg(long, conflicts_with = "agent")]
    all: bool,
    /// Install into project-local skills directories instead of the global
    /// ones (global is the default for a distributed binary).
    #[arg(long)]
    local: bool,
    /// Placement mode for the skill files.
    #[arg(long, value_enum, default_value_t = ModeArg::Copy)]
    mode: ModeArg,
    /// Show what would be installed without writing anything.
    #[arg(long = "dry-run")]
    dry_run: bool,
    /// List the agents that can be targeted and exit.
    #[arg(long = "list-agents")]
    list_agents: bool,
}

#[derive(Args)]
struct SkillUninstallArgs {
    /// Target a specific agent by slug (repeatable). Defaults to every
    /// detected agent.
    #[arg(long = "agent", value_name = "NAME")]
    agent: Vec<String>,
    /// Remove from every agent in the universal list, not only detected ones.
    #[arg(long, conflicts_with = "agent")]
    all: bool,
    /// Operate on project-local skills directories instead of the global ones
    /// (global is the default for a distributed binary).
    #[arg(long)]
    local: bool,
    /// Show what would be removed without deleting anything.
    #[arg(long = "dry-run")]
    dry_run: bool,
    /// List the agents that can be targeted and exit.
    #[arg(long = "list-agents")]
    list_agents: bool,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::New(args) => run_new(args),
        Command::Validate { file } => run_validate(&file),
        Command::Lint(args) => run_lint(args),
        Command::Index(args) => run_index(args),
        Command::KbIndex(args) => run_kb_index(args),
        Command::Backfill(args) => run_backfill(args),
        Command::ArchiveMedia(args) => run_archive_media(args),
        Command::Skill {
            action: SkillAction::Install(args),
        } => run_skill_install(args),
        Command::Skill {
            action: SkillAction::Uninstall(args),
        } => run_skill_uninstall(args),
        Command::Taxonomy {
            action: TaxonomyAction::Add(args),
        } => run_taxonomy_add(args),
        Command::Taxonomy {
            action: TaxonomyAction::Sync(args),
        } => run_taxonomy_sync(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run_new(args: NewArgs) -> std::result::Result<(), String> {
    let title = args
        .title
        .filter(|t| !t.trim().is_empty())
        .unwrap_or_else(|| args.entry_type.default_title().to_string());

    let spec = EntrySpec {
        entry_type: args.entry_type,
        title,
        author: args.author,
        timestamp: Timestamp::now(),
        ticket: args.ticket,
        source: args.source,
    };

    let path = spec
        .create(&args.dir)
        .map_err(|e| format!("cannot create entry: {e}"))?;
    println!("Created {}", path.display());
    Ok(())
}

fn run_validate(file: &str) -> std::result::Result<(), String> {
    let path = PathBuf::from(file);
    match validate::validate(&path) {
        Ok(Outcome::Validated) => {
            println!("OK: {} passes compliance checks", path.display());
            Ok(())
        }
        Ok(Outcome::Skipped) => {
            println!(
                "SKIP: {} is not a journal entry (template or outside a YYYY/ tree)",
                path.display()
            );
            Ok(())
        }
        Err(violation) => Err(format!("{}: {violation}", path.display())),
    }
}

/// Lint corpus tags against the taxonomy and report advisory findings.
fn run_lint(args: LintArgs) -> std::result::Result<(), String> {
    let (taxonomy, source) = Taxonomy::resolve(&args.root, args.taxonomy.as_deref())
        .map_err(|e| format!("cannot load taxonomy: {e}"))?;
    let scan = scan::scan_entries(&args.root, &taxonomy);
    let report = lint::lint(&scan.entries, &taxonomy, &args.files)
        .map_err(|e| format!("lint failed: {e}"))?;

    if args.json {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("cannot serialise report: {e}"))?;
        println!("{json}");
    } else {
        print_lint_report(&taxonomy, &source, &scan, &report);
    }

    // Advisory by default; --strict turns findings into a non-zero exit (2, to
    // distinguish a lint finding from a hard error, which exits 1).
    if args.strict && !report.is_clean() {
        process::exit(2);
    }
    Ok(())
}

/// Print a human-readable lint report. Read failures go to stderr as warnings
/// so they are visible without polluting the findings on stdout.
fn print_lint_report(
    taxonomy: &Taxonomy,
    source: &TaxonomySource,
    scan: &scan::ScanResult,
    report: &lint::LintReport,
) {
    let n = scan.entries.len();
    println!(
        "Linted {n} entr{} against taxonomy: {source}",
        if n == 1 { "y" } else { "ies" }
    );
    for err in &scan.errors {
        eprintln!("  warning: {}: {}", err.file, err.error);
    }

    if report.is_clean() {
        println!("No tag issues found.");
        return;
    }

    if !report.alias_suggestions.is_empty() {
        println!("\nAlias suggestions ({}):", report.alias_suggestions.len());
        for s in &report.alias_suggestions {
            println!(
                "  \"{}\" looks like \"{}\"; add an alias?",
                s.tag, s.suggestion
            );
        }
    }

    if !report.unfaceted.is_empty() {
        println!(
            "\nUnfaceted tags at or over threshold {} ({}):",
            taxonomy.threshold,
            report.unfaceted.len()
        );
        for u in &report.unfaceted {
            println!(
                "  \"{}\" ({}); assign a facet or suppress it",
                u.tag, u.count
            );
        }
    }
}

/// Generate (or validate) the browse index: an NDJSON source of truth and a
/// markdown view rendered from the same scanned entries.
fn run_index(args: IndexArgs) -> std::result::Result<(), String> {
    let (taxonomy, _source) = Taxonomy::resolve(&args.root, args.taxonomy.as_deref())
        .map_err(|e| format!("cannot load taxonomy: {e}"))?;
    let data = args
        .data
        .unwrap_or_else(|| PathBuf::from(index::INDEX_DATA_PATH));
    let view = args
        .view
        .unwrap_or_else(|| PathBuf::from(index::INDEX_VIEW_PATH));

    let scan = scan::scan_entries(&args.root, &taxonomy);
    for err in &scan.errors {
        eprintln!("  warning: {}: {}", err.file, err.error);
    }
    let today = Timestamp::now().date.iso();
    let (ndjson, view_content) = index::build_index(&scan.entries, &taxonomy, &today)
        .map_err(|e| format!("cannot build index: {e}"))?;

    // Byte-identical to a fresh regeneration, not just a structural check on
    // whatever is already committed: mirrors kb-index's --validate. A check
    // that only inspected records already in the index would pass for a
    // brand-new, not-yet-indexed entry and suppress the fix that should run.
    if args.validate {
        let committed = std::fs::read_to_string(&data)
            .map_err(|e| format!("cannot read {}: {e}", data.display()))?;
        if committed != ndjson {
            let probe = index::file_exists_under(&args.root);
            for e in index::validate_ndjson(&committed, Some(&probe)) {
                eprintln!("  x {e}");
            }
            return Err(format!(
                "{} is stale relative to the entries on disk",
                data.display()
            ));
        }
        println!(
            "OK: {} is valid and up to date ({} entries)",
            data.display(),
            scan.entries.len()
        );
        return Ok(());
    }

    if args.dry_run {
        print!("{ndjson}");
        println!("{view_content}");
        return Ok(());
    }

    for path in [&data, &view] {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
            }
        }
    }
    std::fs::write(&data, &ndjson).map_err(|e| format!("cannot write {}: {e}", data.display()))?;
    std::fs::write(&view, &view_content)
        .map_err(|e| format!("cannot write {}: {e}", view.display()))?;
    println!(
        "Wrote {} and {} ({} entries)",
        data.display(),
        view.display(),
        scan.entries.len()
    );
    Ok(())
}

/// Generate (or validate) the knowledge-base index. Any article that fails to
/// scan (missing/malformed frontmatter) fails the whole run, matching the
/// journal-entry validator's stance that a bad record should never blend in
/// silently.
fn run_kb_index(args: KbIndexArgs) -> std::result::Result<(), String> {
    let data = args
        .data
        .unwrap_or_else(|| PathBuf::from(kb::KB_INDEX_DATA_PATH));
    let view = args
        .view
        .unwrap_or_else(|| PathBuf::from(kb::KB_INDEX_VIEW_PATH));

    let scan = kb::scan_kb_articles(&args.root);
    for err in &scan.errors {
        eprintln!("  x {}: {}", err.file, err.error);
    }
    if !scan.errors.is_empty() {
        return Err(format!(
            "kb-index scan failed with {} error(s)",
            scan.errors.len()
        ));
    }
    let ndjson = kb::records_to_ndjson(&scan.records);

    // Byte-identical to a fresh regeneration, not a structural check: this
    // mirrors the consuming repo's own `--validate`, which hk's check step
    // relies on to know a fix is actually needed.
    if args.validate {
        let committed = std::fs::read_to_string(&data)
            .map_err(|e| format!("cannot read {}: {e}", data.display()))?;
        if committed != ndjson {
            return Err(format!(
                "{} is stale relative to the articles on disk",
                data.display()
            ));
        }
        println!(
            "OK: {} is valid and up to date ({} records)",
            data.display(),
            scan.records.len()
        );
        return Ok(());
    }

    let today = Timestamp::now().date.iso();
    let link_prefix = kb::compute_link_prefix(&args.root, &view);
    let view_content = kb::render_kb_index(&scan.records, &today, &link_prefix);

    if args.dry_run {
        print!("{ndjson}");
        println!("{view_content}");
        return Ok(());
    }

    for path in [&data, &view] {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
            }
        }
    }
    std::fs::write(&data, &ndjson).map_err(|e| format!("cannot write {}: {e}", data.display()))?;
    std::fs::write(&view, &view_content)
        .map_err(|e| format!("cannot write {}: {e}", view.display()))?;
    println!(
        "Wrote {} and {} ({} records)",
        data.display(),
        view.display(),
        scan.records.len()
    );
    Ok(())
}

/// Backfill missing `title` / `date` frontmatter across dated entries.
fn run_backfill(args: BackfillArgs) -> std::result::Result<(), String> {
    let report = backfill::backfill(&args.root, args.dry_run)
        .map_err(|e| format!("backfill failed: {e}"))?;

    for w in &report.warnings {
        eprintln!("  warning: {w}");
    }

    let verb = if args.dry_run {
        "would inject"
    } else {
        "injected"
    };
    println!(
        "Processed {} file(s): {verb} {} title(s), {} date(s)",
        report.processed, report.titles_injected, report.dates_injected
    );

    if !report.missing_tags.is_empty() {
        eprintln!("\n{} file(s) have no tags:", report.missing_tags.len());
        for f in &report.missing_tags {
            eprintln!("  {f}");
        }
    }
    Ok(())
}

fn run_archive_media(args: ArchiveMediaCliArgs) -> std::result::Result<(), String> {
    let names = if args.name.is_empty() {
        None
    } else {
        Some(args.name.as_slice())
    };
    let core_args = ArchiveMediaCoreArgs {
        slug: &args.slug,
        urls: &args.urls,
        names,
        prefix: &args.prefix,
        force: args.force,
    };
    let saved =
        archive_media::archive_media(&args.root, &core_args, archive_media::fetch_bytes, |line| {
            eprintln!("{line}")
        })
        .map_err(|e| e.to_string())?;
    for asset in &saved {
        println!("{} ({} bytes)", asset.path.display(), asset.bytes);
    }
    Ok(())
}

/// Promote a tag into a facet, writing the taxonomy back to disk (or previewing
/// the result with `--dry-run`). Errors if the tag is already faceted or the
/// facet does not exist -- see [`journal::taxonomy::Taxonomy::add_tag_to_facet`].
fn run_taxonomy_add(args: TaxonomyAddArgs) -> std::result::Result<(), String> {
    let (mut taxonomy, source) = Taxonomy::resolve(&args.root, args.taxonomy.as_deref())
        .map_err(|e| format!("cannot load taxonomy: {e}"))?;

    taxonomy
        .add_tag_to_facet(&args.tag, &args.facet)
        .map_err(|e| e.to_string())?;

    let target = match &source {
        TaxonomySource::Explicit(path) | TaxonomySource::Root(path) => path.clone(),
        // No taxonomy.json exists yet; start one at the conventional location
        // rather than pretending to write into the embedded, read-only default.
        TaxonomySource::EmbeddedDefault => args.root.join("taxonomy.json"),
    };

    if args.dry_run {
        let json = taxonomy
            .to_json_pretty()
            .map_err(|e| format!("cannot serialise taxonomy: {e}"))?;
        println!("Dry run: would write {}:\n\n{json}", target.display());
        return Ok(());
    }

    taxonomy
        .write_to_path(&target)
        .map_err(|e| format!("cannot write {}: {e}", target.display()))?;
    println!(
        "Added \"{}\" to facet \"{}\" in {}",
        args.tag,
        args.facet,
        target.display()
    );
    Ok(())
}

/// Read `taxonomy.json` as it existed at `sha`, via `git show`. Returns
/// `Ok(None)` (not an error) when the commit or path can't be resolved, so a
/// missing or unreachable baseline just skips the append-only check rather
/// than failing the whole command.
fn read_baseline_taxonomy(
    sha: &str,
    taxonomy_path: &Path,
) -> std::result::Result<Option<Taxonomy>, String> {
    let toplevel = process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !toplevel.status.success() {
        return Ok(None);
    }
    let repo_root = PathBuf::from(String::from_utf8_lossy(&toplevel.stdout).trim().to_string());

    let abs_taxonomy = match taxonomy_path.canonicalize() {
        Ok(p) => p,
        Err(_) => return Ok(None),
    };
    let rel = match abs_taxonomy.strip_prefix(&repo_root) {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    let output = process::Command::new("git")
        .args(["show", &format!("{sha}:{}", rel.display())])
        .current_dir(&repo_root)
        .output()
        .map_err(|e| format!("cannot run git show: {e}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    let json = String::from_utf8_lossy(&output.stdout);
    let baseline = Taxonomy::from_json(&json, "append-only baseline")
        .map_err(|e| format!("cannot parse baseline taxonomy at {sha}: {e}"))?;
    Ok(Some(baseline))
}

/// Auto-facet every confidently-classifiable unfaceted tag (per
/// `lint::lint`'s threshold/ticket-pattern detection and `classify_tag`'s
/// heuristics), write the result unless `--check`, and check the append-only
/// invariant against `appendOnlyBaseline` if the taxonomy declares one.
/// `--check` never writes and exits non-zero if anything is still
/// outstanding, matching the journal CLI's `taxonomy sync --check` semantics
/// this repo's `hk.pkl` gate relies on.
fn run_taxonomy_sync(args: TaxonomySyncArgs) -> std::result::Result<(), String> {
    let (taxonomy, source) = Taxonomy::resolve(&args.root, args.taxonomy.as_deref())
        .map_err(|e| format!("cannot load taxonomy: {e}"))?;

    let scan = scan::scan_entries(&args.root, &taxonomy);
    let report =
        lint::lint(&scan.entries, &taxonomy, &[]).map_err(|e| format!("lint failed: {e}"))?;

    let mut next_taxonomy = taxonomy.clone();
    let mut auto_faceted: Vec<(String, String)> = Vec::new();
    let mut ambiguous_tags: Vec<String> = Vec::new();
    let mut still_unfaceted: Vec<(String, u32)> = Vec::new();

    for u in &report.unfaceted {
        let classification = classify_tag(&u.tag, &taxonomy);
        let confidently_faceted =
            matches!(classification.confidence, ClassifyConfidence::Confident)
                .then_some(classification.facet.as_deref())
                .flatten()
                .filter(|facet| next_taxonomy.facets.contains_key(*facet));

        match confidently_faceted {
            Some(facet) if !args.check_only => {
                let facet = facet.to_string();
                next_taxonomy
                    .add_tag_to_facet(&u.tag, &facet)
                    .map_err(|e| e.to_string())?;
                auto_faceted.push((u.tag.clone(), facet));
            }
            Some(_) => still_unfaceted.push((u.tag.clone(), u.count)),
            None => {
                ambiguous_tags.push(u.tag.clone());
                still_unfaceted.push((u.tag.clone(), u.count));
            }
        }
    }

    let target = match &source {
        TaxonomySource::Explicit(path) | TaxonomySource::Root(path) => path.clone(),
        TaxonomySource::EmbeddedDefault => args.root.join("taxonomy.json"),
    };

    if !args.check_only && !auto_faceted.is_empty() {
        next_taxonomy
            .write_to_path(&target)
            .map_err(|e| format!("cannot write {}: {e}", target.display()))?;
    }

    let append_only_violations = match &taxonomy.append_only_baseline {
        Some(sha) => match read_baseline_taxonomy(sha, &target) {
            Ok(Some(baseline)) => check_append_only(&baseline, &next_taxonomy),
            Ok(None) => Vec::new(),
            Err(e) => {
                eprintln!("warning: cannot read append-only baseline {sha}: {e}");
                Vec::new()
            }
        },
        None => Vec::new(),
    };

    if args.check_only {
        for (tag, count) in &still_unfaceted {
            if ambiguous_tags.contains(tag) {
                eprintln!(
                    "tag \"{tag}\" ({count}) is ambiguous - run: pantheon-journal taxonomy add {tag} --facet <facet>"
                );
            } else {
                eprintln!(
                    "tag \"{tag}\" ({count}) crossed the threshold but has no facet - run: pantheon-journal taxonomy sync"
                );
            }
        }
        for v in &append_only_violations {
            eprintln!("append-only violation: {}", v.detail);
        }
        if !still_unfaceted.is_empty() || !append_only_violations.is_empty() {
            process::exit(1);
        }
    } else {
        for (tag, facet) in &auto_faceted {
            println!("assigned \"{tag}\" to \"{facet}\"");
        }
        if auto_faceted.is_empty() {
            println!("Nothing to sync.");
        }
    }

    Ok(())
}

/// Install the bundled companion skill into agent directories.
fn run_skill_install(args: SkillInstallArgs) -> std::result::Result<(), String> {
    if args.list_agents {
        print_agent_list();
        return Ok(());
    }

    let env = Environment::from_env().map_err(|e| format!("cannot resolve environment: {e}"))?;

    let selection = if args.all {
        Selection::All
    } else if !args.agent.is_empty() {
        Selection::Explicit(args.agent.clone())
    } else {
        Selection::Detected
    };

    let opts = InstallOptions {
        selection,
        global: !args.local,
        mode: args.mode.into(),
        dry_run: args.dry_run,
    };

    // The embedded skill is extracted to a stable per-user directory. Copies
    // are duplicated from there; symlinks point back at it, so it must outlive
    // the command (unlike a temporary directory).
    let bundle_root = bundle_home(&env);
    let source_skill_dir = if args.dry_run {
        bundle_root.join(skill_bundle::skill_name())
    } else {
        skill_bundle::materialise(&bundle_root)
            .map_err(|e| format!("cannot unpack bundled skill: {e}"))?
    };

    let exists = |p: &Path| p.exists();
    let report = install_cmd::run_install(&opts, &env, &exists, &source_skill_dir)
        .map_err(|e| e.to_string())?;

    print_install_report(&opts, skill_bundle::skill_name(), &report);
    Ok(())
}

/// Resolve the per-user directory the embedded skill is extracted into
/// (`$XDG_DATA_HOME/tekhne/journal`, or `~/.local/share/...`).
fn bundle_home(env: &Environment) -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| env.home.join(".local/share"))
        .join("tekhne")
        .join("journal")
}

/// Print the agents `skill install` can target, in table order.
fn print_agent_list() {
    println!("Agents journal can install into:\n");
    for agent in all_agents() {
        let scope = if agent.show_in_universal_list {
            ""
        } else {
            " (opt-in only)"
        };
        println!("  {:<16} {}{scope}", agent.name, agent.display_name);
    }
}

/// Print a human-readable summary of an install run.
fn print_install_report(
    opts: &InstallOptions,
    skill_name: &str,
    report: &install_cmd::InstallReport,
) {
    let mode = match opts.mode {
        InstallMode::Copy => "copy",
        InstallMode::Symlink => "symlink",
    };
    let scope = if opts.global { "global" } else { "local" };

    if report.outcomes.is_empty() {
        println!(
            "No agents selected. No installed agents were detected; pass --agent <name> or --all, or run --list-agents."
        );
        return;
    }

    if opts.dry_run {
        println!("Dry run: would install '{skill_name}' ({mode}, {scope}) into:");
    } else {
        println!("Installed '{skill_name}' ({mode}, {scope}) into:");
    }

    let mut installed = 0;
    let mut failed = 0;
    for outcome in &report.outcomes {
        match (&outcome.error, &outcome.installed_path) {
            (Some(err), _) => {
                failed += 1;
                println!("  {:<16} FAILED: {err}", outcome.agent);
            }
            (None, Some(path)) => {
                installed += 1;
                println!("  {:<16} {}", outcome.agent, path.display());
            }
            (None, None) => {}
        }
    }

    if !report.missing.is_empty() {
        println!(
            "\nNote: these agents are not detected on this machine but were targeted anyway: {}",
            report.missing.join(", ")
        );
    }

    let verb = if opts.dry_run { "planned" } else { "installed" };
    println!("\n{installed} {verb}, {failed} failed.");
}

/// Remove the bundled companion skill from agent directories.
fn run_skill_uninstall(args: SkillUninstallArgs) -> std::result::Result<(), String> {
    if args.list_agents {
        print_agent_list();
        return Ok(());
    }

    let env = Environment::from_env().map_err(|e| format!("cannot resolve environment: {e}"))?;

    let selection = if args.all {
        Selection::All
    } else if !args.agent.is_empty() {
        Selection::Explicit(args.agent.clone())
    } else {
        Selection::Detected
    };

    let opts = UninstallOptions {
        selection,
        global: !args.local,
        dry_run: args.dry_run,
    };

    let exists = |p: &Path| p.exists();
    let report = install_cmd::run_uninstall(&opts, &env, &exists, skill_bundle::skill_name())
        .map_err(|e| e.to_string())?;

    print_uninstall_report(&opts, skill_bundle::skill_name(), &report);
    Ok(())
}

/// Print a human-readable summary of an uninstall run.
fn print_uninstall_report(
    opts: &UninstallOptions,
    skill_name: &str,
    report: &install_cmd::UninstallReport,
) {
    let scope = if opts.global { "global" } else { "local" };

    if report.outcomes.is_empty() {
        println!(
            "No agents selected. No installed agents were detected; pass --agent <name> or --all, or run --list-agents."
        );
        return;
    }

    if opts.dry_run {
        println!("Dry run: would remove '{skill_name}' ({scope}) from:");
    } else {
        println!("Removed '{skill_name}' ({scope}) from:");
    }

    let mut removed = 0;
    let mut absent = 0;
    let mut failed = 0;
    for outcome in &report.outcomes {
        match &outcome.error {
            Some(err) => {
                failed += 1;
                println!("  {:<16} FAILED: {err}", outcome.agent);
            }
            None if outcome.removed => {
                removed += 1;
                println!("  {:<16} {}", outcome.agent, outcome.target_path.display());
            }
            None => {
                absent += 1;
                println!("  {:<16} (nothing to remove)", outcome.agent);
            }
        }
    }

    if !report.missing.is_empty() {
        println!(
            "\nNote: these agents are not detected on this machine but were targeted anyway: {}",
            report.missing.join(", ")
        );
    }

    let verb = if opts.dry_run { "planned" } else { "removed" };
    println!("\n{removed} {verb}, {absent} already absent, {failed} failed.");
}
