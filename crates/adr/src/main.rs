//! `adr` CLI: create, list, and supersede Architecture Decision Records, plus
//! install the bundled `adr-creator` companion skill into agent directories
//! (the A5 distribution surface shared with `skill-auditor`).

use std::path::{Path, PathBuf};
use std::process;

use adr_core::adr as core;
use adr_core::date;
use adr_core::install_cmd::{self, InstallOptions, Selection, UninstallOptions};
use adr_core::skill_bundle;
use clap::{Args, Parser, Subcommand, ValueEnum};
use skill_install::agents::all as all_agents;
use skill_install::env::Environment;
use skill_install::install::InstallMode;

const VERSION: &str = "0.1.0";

#[derive(Parser)]
#[command(
    name = "pantheon-adr",
    version = VERSION,
    about = "Create and manage Architecture Decision Records",
    long_about = "pantheon-adr creates, lists, and supersedes Architecture Decision Records from the house template, and installs the bundled adr-creator skill into agent directories."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the version.
    Version,
    /// Create the next-numbered ADR from the house template.
    New {
        /// The ADR title (used for the heading and kebab-case file name).
        title: String,
        /// The ADR directory (default `docs/adr`, or the ADR_DIR env var).
        #[arg(long)]
        dir: Option<String>,
    },
    /// List existing ADRs (number, status, title).
    List {
        /// The ADR directory (default `docs/adr`, or the ADR_DIR env var).
        #[arg(long)]
        dir: Option<String>,
    },
    /// Supersede an existing ADR with a new one.
    Supersede {
        /// The number of the ADR being superseded.
        number: u32,
        /// The title of the new ADR.
        new_title: String,
        /// The ADR directory (default `docs/adr`, or the ADR_DIR env var).
        #[arg(long)]
        dir: Option<String>,
    },
    /// Manage the bundled companion skill.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
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
        Command::Version => {
            println!("adr v{VERSION}");
            Ok(())
        }
        Command::New { title, dir } => run_new(&title, dir.as_deref()),
        Command::List { dir } => run_list(dir.as_deref()),
        Command::Supersede {
            number,
            new_title,
            dir,
        } => run_supersede(number, &new_title, dir.as_deref()),
        Command::Skill {
            action: SkillAction::Install(args),
        } => run_skill_install(args),
        Command::Skill {
            action: SkillAction::Uninstall(args),
        } => run_skill_uninstall(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Resolve the ADR directory from the flag or the `ADR_DIR` env var.
fn adr_dir(flag: Option<&str>) -> PathBuf {
    let env_value = std::env::var(core::DIR_ENV).ok();
    core::resolve_dir(flag, env_value.as_deref())
}

fn run_new(title: &str, dir_flag: Option<&str>) -> std::result::Result<(), String> {
    let dir = adr_dir(dir_flag);
    let path = core::create_new(&dir, title, &date::today()).map_err(|e| e.to_string())?;
    println!("Created {}", path.display());
    Ok(())
}

fn run_list(dir_flag: Option<&str>) -> std::result::Result<(), String> {
    let dir = adr_dir(dir_flag);
    let entries = core::list(&dir).map_err(|e| e.to_string())?;
    if entries.is_empty() {
        println!("No ADRs found in {}", dir.display());
        return Ok(());
    }
    let width = entries
        .iter()
        .map(|e| e.status.len())
        .max()
        .unwrap_or(0)
        .max(6);
    for entry in entries {
        println!(
            "ADR-{:04}  {:<width$}  {}",
            entry.number, entry.status, entry.title
        );
    }
    Ok(())
}

fn run_supersede(
    number: u32,
    new_title: &str,
    dir_flag: Option<&str>,
) -> std::result::Result<(), String> {
    let dir = adr_dir(dir_flag);
    let (old_path, new_path) =
        core::supersede(&dir, number, new_title, &date::today()).map_err(|e| e.to_string())?;
    println!("Superseded {}", old_path.display());
    println!("Created {}", new_path.display());
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
/// (`$XDG_DATA_HOME/tekhne/adr`, or `~/.local/share/...`).
fn bundle_home(env: &Environment) -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| env.home.join(".local/share"))
        .join("tekhne")
        .join("adr")
}

/// Print the agents `skill install` can target, in table order.
fn print_agent_list() {
    println!("Agents adr can install into:\n");
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
