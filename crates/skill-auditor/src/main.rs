//! `skill-auditor` CLI, ported from the Go cobra command surface: `version`,
//! `evaluate <skill>` and `batch <skill...>`, with `--json`, `--store`,
//! `--repo-root` and (batch only) `--fail-below`.

use clap::{Args, Parser, Subcommand, ValueEnum};
use skill_auditor::duplication;
use skill_auditor::install_cmd::{self, InstallOptions, Selection, UninstallOptions};
use skill_auditor::reporter;
use skill_auditor::scorer::{self, grade_rank, Result as AuditResult};
use skill_auditor::skill_bundle;
use skill_install::agents::all as all_agents;
use skill_install::env::Environment;
use skill_install::install::InstallMode;
use std::path::{Path, PathBuf};
use std::process;

const VERSION: &str = "0.1.0";

#[derive(Parser)]
#[command(
    name = "skill-auditor",
    version = VERSION,
    about = "Audit skill quality using the 9-dimension framework",
    long_about = "skill-auditor evaluates skills against the 9-dimension quality framework, combining skill-validator structural checks with custom D1-D9 scoring."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the version.
    Version,
    /// Evaluate a single skill.
    Evaluate {
        /// The skill (domain/skill-name, path, or SKILL.md path).
        skill: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
        /// Persist result to .context/audits/.
        #[arg(long)]
        store: bool,
        /// Repo root (auto-detected if empty).
        #[arg(long = "repo-root")]
        repo_root: Option<String>,
    },
    /// Evaluate multiple skills.
    Batch {
        /// One or more skills.
        #[arg(required = true)]
        skills: Vec<String>,
        /// Emit JSON array output.
        #[arg(long)]
        json: bool,
        /// Persist each result to .context/audits/.
        #[arg(long)]
        store: bool,
        /// Exit 1 if any skill scores below this grade (e.g. B+).
        #[arg(long = "fail-below")]
        fail_below: Option<String>,
        /// Repo root (auto-detected if empty).
        #[arg(long = "repo-root")]
        repo_root: Option<String>,
    },
    /// Detect duplication across skills (line-overlap or composite similarity).
    Duplication(DuplicationArgs),
    /// Manage the bundled companion skill.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
}

#[derive(Args)]
struct DuplicationArgs {
    /// Root directory to scan for `SKILL.md` files.
    #[arg(default_value = "skills")]
    skills_dir: String,
    /// Use the composite (semantic + structural + lexical) algorithm instead of
    /// the basic line-overlap one.
    #[arg(long)]
    enhanced: bool,
    /// Only report pairs at or above this similarity percentage.
    #[arg(long, default_value_t = duplication::MODERATE_THRESHOLD)]
    threshold: u32,
    /// Emit JSON instead of a Markdown report.
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum SkillAction {
    /// Install the bundled skill into detected (or selected) agent directories.
    Install(SkillInstallArgs),
    /// Remove the bundled skill from detected (or selected) agent directories.
    Uninstall(SkillUninstallArgs),
    /// List the files embedded in this binary (what `install` would write).
    Bundle(SkillBundleArgs),
}

#[derive(Args)]
struct SkillBundleArgs {
    /// List the embedded files with sizes (the default when no flag is given).
    #[arg(long)]
    list: bool,
    /// Emit the bundle manifest as JSON instead of a table.
    #[arg(long, conflicts_with = "list")]
    manifest: bool,
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
        Command::Version => {
            println!("skill-auditor v{VERSION}");
            Ok(())
        }
        Command::Evaluate {
            skill,
            json,
            store,
            repo_root,
        } => run_evaluate(&skill, json, store, repo_root.as_deref()),
        Command::Batch {
            skills,
            json,
            store,
            fail_below,
            repo_root,
        } => run_batch(
            &skills,
            json,
            store,
            fail_below.as_deref(),
            repo_root.as_deref(),
        ),
        Command::Duplication(args) => run_duplication(args),
        Command::Skill {
            action: SkillAction::Install(args),
        } => run_skill_install(args),
        Command::Skill {
            action: SkillAction::Uninstall(args),
        } => run_skill_uninstall(args),
        Command::Skill {
            action: SkillAction::Bundle(args),
        } => run_skill_bundle(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run_evaluate(
    skill_arg: &str,
    json: bool,
    store: bool,
    repo_root_flag: Option<&str>,
) -> std::result::Result<(), String> {
    let repo_root = resolve_repo_root(repo_root_flag)
        .map_err(|e| format!("cannot determine repo root: {e}"))?;
    let skill_path = resolve_skill_path(skill_arg, &repo_root);
    let skill_key = canonical_skill_key(&skill_path, &repo_root);

    let mut result = scorer::score(&skill_path).map_err(|e| format!("scoring failed: {e}"))?;
    result.skill = skill_key.clone();

    if json {
        let data =
            serde_json::to_string_pretty(&result).map_err(|e| format!("marshal result: {e}"))?;
        println!("{data}");
    } else {
        print!("{}", reporter::format(&result));
    }

    if store {
        reporter::store(&repo_root, &skill_key, &result)
            .map_err(|e| format!("store result: {e}"))?;
    }

    Ok(())
}

struct Entry {
    arg: String,
    result: Option<AuditResult>,
    error: Option<String>,
}

fn run_batch(
    args: &[String],
    json: bool,
    store: bool,
    fail_below: Option<&str>,
    repo_root_flag: Option<&str>,
) -> std::result::Result<(), String> {
    let repo_root = resolve_repo_root(repo_root_flag)
        .map_err(|e| format!("cannot determine repo root: {e}"))?;

    let mut entries: Vec<Entry> = Vec::with_capacity(args.len());
    for arg in args {
        let skill_path = resolve_skill_path(arg, &repo_root);
        match scorer::score(&skill_path) {
            Err(e) => entries.push(Entry {
                arg: arg.clone(),
                result: None,
                error: Some(e.to_string()),
            }),
            Ok(result) => {
                if store {
                    if let Err(e) = reporter::store(&repo_root, arg, &result) {
                        eprintln!("warning: store {arg}: {e}");
                    }
                }
                entries.push(Entry {
                    arg: arg.clone(),
                    result: Some(result),
                    error: None,
                });
            }
        }
    }

    if json {
        let results: Vec<&AuditResult> = entries.iter().filter_map(|e| e.result.as_ref()).collect();
        let data =
            serde_json::to_string_pretty(&results).map_err(|e| format!("marshal results: {e}"))?;
        println!("{data}");
    } else {
        print_batch_table(&mut entries);
    }

    if let Some(grade) = fail_below {
        let threshold =
            grade_rank(grade).ok_or_else(|| format!("unknown grade {grade:?} for --fail-below"))?;
        for e in &entries {
            let Some(result) = &e.result else { continue };
            if grade_rank(&result.grade).unwrap_or(0) < threshold {
                return Err(format!(
                    "skill {} scored {}, below threshold {grade}",
                    e.arg, result.grade
                ));
            }
        }
    }

    Ok(())
}

fn print_batch_table(entries: &mut [Entry]) {
    // Sort by score descending; errored entries go last.
    entries.sort_by(|a, b| match (&a.result, &b.result) {
        (None, _) => std::cmp::Ordering::Greater,
        (_, None) => std::cmp::Ordering::Less,
        (Some(x), Some(y)) => y.total.cmp(&x.total),
    });

    let mut max_len = entries.iter().map(|e| e.arg.len()).max().unwrap_or(0);
    if max_len < 40 {
        max_len = 40;
    }

    let mut total_score = 0;
    let mut success_count = 0;
    for e in entries.iter() {
        match (&e.result, &e.error) {
            (_, Some(err)) => println!("{:<max_len$}  ERROR: {err}", e.arg),
            (Some(result), _) => {
                println!(
                    "{:<max_len$}  {:<2} ({}/{})",
                    e.arg, result.grade, result.total, result.max_total
                );
                total_score += result.total;
                success_count += 1;
            }
            _ => {}
        }
    }

    let sep = "\u{2500}".repeat(max_len + 20);
    println!("{sep}");
    let avg = if success_count > 0 {
        total_score / success_count
    } else {
        0
    };
    println!("Total: {} skill(s)  Average: {avg}/140", entries.len());
}

/// Detect and report duplication across the skills under `--skills-dir`.
fn run_duplication(args: DuplicationArgs) -> std::result::Result<(), String> {
    let root = Path::new(&args.skills_dir);
    if !root.is_dir() {
        return Err(format!("skills directory not found: {}", args.skills_dir));
    }
    let skills = duplication::load_skills(root);

    if args.enhanced {
        let pairs = duplication::detect_enhanced(&skills);
        if args.json {
            let json = serde_json::to_string_pretty(&pairs).map_err(|e| format!("marshal: {e}"))?;
            println!("{json}");
        } else {
            print!(
                "{}",
                render_enhanced(&args.skills_dir, skills.len(), &pairs)
            );
        }
    } else {
        let pairs = duplication::detect_basic(&skills, args.threshold);
        if args.json {
            let json = serde_json::to_string_pretty(&pairs).map_err(|e| format!("marshal: {e}"))?;
            println!("{json}");
        } else {
            print!(
                "{}",
                render_basic(&args.skills_dir, skills.len(), args.threshold, &pairs)
            );
        }
    }
    Ok(())
}

/// Render the basic (line-overlap) duplication report as Markdown.
fn render_basic(
    dir: &str,
    count: usize,
    threshold: u32,
    pairs: &[duplication::BasicPair],
) -> String {
    let mut s = String::new();
    s.push_str("# Skill Duplication Report\n\n## Summary\n");
    s.push_str(&format!("- Skills analyzed: {count}\n"));
    s.push_str(&format!("- Directory: {dir}\n"));
    s.push_str(&format!("- Threshold: >{threshold}% similarity\n\n"));
    s.push_str("## High Duplication Pairs\n\n");
    if pairs.is_empty() {
        s.push_str("None above threshold.\n");
    } else {
        for p in pairs {
            s.push_str(&format!("### {} \u{2194} {}\n", p.name1, p.name2));
            s.push_str(&format!("- Similarity: {}%\n", p.similarity));
            s.push_str(&format!("- Common lines: {}\n", p.common_lines));
            s.push_str("- Recommendation: Consider aggregation\n\n");
        }
    }
    s
}

/// Render the enhanced (composite) duplication report as Markdown, grouped by
/// similarity band.
fn render_enhanced(dir: &str, count: usize, pairs: &[duplication::EnhancedPair]) -> String {
    use duplication::Band;

    let mut s = String::new();
    s.push_str("# Enhanced Skill Duplication Report\n\n## Executive Summary\n");
    s.push_str(&format!("- Skills analyzed: {count}\n"));
    s.push_str(&format!("- Directory: {dir}\n"));
    s.push_str(&format!(
        "- Thresholds: Moderate >={}%, High >={}%, Critical >={}%\n\n",
        duplication::MODERATE_THRESHOLD,
        duplication::HIGH_THRESHOLD,
        duplication::CRITICAL_THRESHOLD
    ));

    for band in [Band::Critical, Band::High, Band::Moderate] {
        s.push_str(&format!("## {} Duplications\n\n", band.label()));
        let group: Vec<&duplication::EnhancedPair> =
            pairs.iter().filter(|p| p.band == band).collect();
        if group.is_empty() {
            s.push_str("None.\n\n");
            continue;
        }
        for p in group {
            s.push_str(&format!("### {} \u{2194} {}\n", p.name1, p.name2));
            s.push_str(&format!(
                "**Overall Similarity**: {}% {}\n\n",
                p.composite,
                band.label()
            ));
            s.push_str("| Metric | Score | Weight |\n|--------|-------|--------|\n");
            s.push_str(&format!("| Semantic | {}% | 40% |\n", p.semantic));
            s.push_str(&format!("| Structural | {}% | 35% |\n", p.structural));
            s.push_str(&format!("| Lexical | {}% | 25% |\n\n", p.lexical));
            s.push_str(&format!("**Action Required**: {}\n\n", band.action()));
        }
    }
    s
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

/// Report the files baked into this binary, either as a table (`--list`, the
/// default) or as JSON (`--manifest`).
fn run_skill_bundle(args: SkillBundleArgs) -> std::result::Result<(), String> {
    if args.manifest {
        print_bundle_manifest()
    } else {
        print_bundle_list();
        Ok(())
    }
}

/// Print a human-readable table of every embedded file and a size summary.
fn print_bundle_list() {
    let entries = skill_bundle::manifest();
    let width = entries
        .iter()
        .map(|e| e.path.len())
        .max()
        .unwrap_or(0)
        .max(20);

    println!("Embedded skill: {}", skill_bundle::skill_name());
    println!(
        "Files: {}   Total: {}\n",
        entries.len(),
        human_bytes(skill_bundle::total_bytes())
    );
    for entry in &entries {
        println!("  {:<width$}  {:>10}", entry.path, human_bytes(entry.bytes));
    }
}

/// JSON view of the bundle manifest: skill name, counts, and per-file sizes.
#[derive(serde::Serialize)]
struct BundleManifest<'a> {
    skill: &'a str,
    file_count: usize,
    total_bytes: usize,
    files: Vec<BundleManifestFile<'a>>,
}

#[derive(serde::Serialize)]
struct BundleManifestFile<'a> {
    path: &'a str,
    bytes: usize,
}

/// Serialise the embedded-file manifest to pretty JSON on stdout.
fn print_bundle_manifest() -> std::result::Result<(), String> {
    let entries = skill_bundle::manifest();
    let manifest = BundleManifest {
        skill: skill_bundle::skill_name(),
        file_count: entries.len(),
        total_bytes: skill_bundle::total_bytes(),
        files: entries
            .iter()
            .map(|e| BundleManifestFile {
                path: e.path,
                bytes: e.bytes,
            })
            .collect(),
    };
    let json =
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("marshal manifest: {e}"))?;
    println!("{json}");
    Ok(())
}

/// Format a byte count as a compact human-readable string (B / KB / MB).
fn human_bytes(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    let b = bytes as f64;
    if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

/// Resolve the per-user directory the embedded skill is extracted into
/// (`$XDG_DATA_HOME/tekhne/skill-auditor`, or `~/.local/share/...`).
fn bundle_home(env: &Environment) -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| env.home.join(".local/share"))
        .join("tekhne")
        .join("skill-auditor")
}

/// Print the agents `skill install` can target, in table order.
fn print_agent_list() {
    println!("Agents skill-auditor can install into:\n");
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

/// Derive the `domain/skill-name` storage key from an absolute SKILL.md path
/// by stripping `<repo_root>/skills/` and the trailing `/SKILL.md` (Go
/// `canonicalSkillKey`).
fn canonical_skill_key(skill_path: &Path, repo_root: &Path) -> String {
    let prefix = repo_root.join("skills");
    let prefix_str = format!("{}{}", prefix.to_string_lossy(), std::path::MAIN_SEPARATOR);
    let path_str = skill_path.to_string_lossy();
    let key = path_str.strip_prefix(&prefix_str).unwrap_or(&path_str);
    let suffix = format!("{}SKILL.md", std::path::MAIN_SEPARATOR);
    key.strip_suffix(&suffix).unwrap_or(key).to_string()
}

/// Convert a skill arg to an absolute path to SKILL.md (Go `resolveSkillPath`).
fn resolve_skill_path(skill_arg: &str, repo_root: &Path) -> PathBuf {
    let is_fs_path = Path::new(skill_arg).is_absolute()
        || skill_arg.starts_with("./")
        || skill_arg.starts_with("../");

    let base: PathBuf = if is_fs_path {
        std::path::absolute(skill_arg).unwrap_or_else(|_| PathBuf::from(skill_arg))
    } else {
        repo_root.join("skills").join(skill_arg)
    };

    if base.to_string_lossy().ends_with("SKILL.md") {
        base
    } else {
        base.join("SKILL.md")
    }
}

/// Return the repo root from flag or auto-detection (Go `resolveRepoRoot`).
fn resolve_repo_root(flag: Option<&str>) -> std::io::Result<PathBuf> {
    if let Some(value) = flag {
        if !value.is_empty() {
            return Ok(PathBuf::from(value));
        }
    }
    let cwd = std::env::current_dir()?;
    find_repo_root(&cwd)
}

/// Walk up from `dir` until a `.git` or `go.mod` entry is found (Go
/// `findRepoRoot`).
fn find_repo_root(dir: &Path) -> std::io::Result<PathBuf> {
    let mut current = dir;
    loop {
        if current.join(".git").exists() || current.join("go.mod").exists() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("no .git or go.mod found above {}", dir.display()),
                ))
            }
        }
    }
}
