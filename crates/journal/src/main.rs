//! `journal` CLI: create structured entries (`new`), check them
//! (`validate <file>`), and install the bundled companion skill
//! (`skill install`).

use std::path::{Path, PathBuf};
use std::process;

use clap::{Args, Parser, Subcommand, ValueEnum};
use journal::date::Timestamp;
use journal::entry::{EntrySpec, EntryType};
use journal::install_cmd::{self, InstallOptions, Selection};
use journal::skill_bundle;
use journal::validate::{self, Outcome};
use skill_install::agents::all as all_agents;
use skill_install::env::Environment;
use skill_install::install::InstallMode;

const VERSION: &str = "0.1.0";

#[derive(Parser)]
#[command(
    name = "journal",
    version = VERSION,
    about = "Create and validate structured journal entries",
    long_about = "journal creates timestamped markdown entries with YAML frontmatter, triple-synced dates, and template-based sections, then validates them for compliance. It promotes the journal-entry-creator skill into a self-contained CLI."
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
    /// Manage the bundled companion skill.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
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
    /// Issue-tracker key: the refinement target, or a troubleshooting slug
    /// prefix.
    #[arg(long)]
    ticket: Option<String>,
    /// Source URL, recorded for article summaries.
    #[arg(long)]
    source: Option<String>,
}

#[derive(Subcommand)]
enum SkillAction {
    /// Install the bundled skill into detected (or selected) agent directories.
    Install(SkillInstallArgs),
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

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::New(args) => run_new(args),
        Command::Validate { file } => run_validate(&file),
        Command::Skill {
            action: SkillAction::Install(args),
        } => run_skill_install(args),
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
