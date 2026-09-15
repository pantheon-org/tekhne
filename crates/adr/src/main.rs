//! `pantheon-adr` CLI: create, score, review and supersede Architecture
//! Decision Records, wire the ADR hooks into agent harnesses, and install the
//! bundled `adr-creator` companion skill into agent directories (the A5
//! distribution surface shared with `skill-auditor`).
//!
//! Argument parsing lives here; every command body lives in
//! [`adr_core::commands`], which returns text rather than printing, so the
//! output is asserted in tests.

use std::path::{Path, PathBuf};
use std::process;

use adr_core::commands;
use adr_core::date;
use adr_core::install_cmd::{self, InstallOptions, Selection, UninstallOptions};
use adr_core::record;
use adr_core::skill_bundle;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use common::{Error, Result};
use skill_install::agents::all as all_agents;
use skill_install::env::Environment;
use skill_install::install::InstallMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(
    name = "pantheon-adr",
    version = VERSION,
    about = "Create and manage Architecture Decision Records",
    long_about = "pantheon-adr creates, scores, reviews and supersedes Architecture Decision Records. Each record is a markdown file whose YAML frontmatter holds its status and history, so there is no separate index to keep in step. `check` scores a record against a fixed rubric and `review` gates on it; `sync` wires the same checks into agent harnesses, and `skill install` installs the bundled adr-creator skill."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// The ADR directory, shared by every command that touches records.
#[derive(Args, Clone)]
struct DirArgs {
    /// The ADR directory (default `docs/adr`, or the ADR_DIR env var).
    #[arg(long, global = true)]
    dir: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Print the version.
    Version,
    /// Create the ADR directory, and optionally the pre-push hook.
    Init {
        /// Project name shown in hook output (auto-detected from git if omitted).
        #[arg(long)]
        project: Option<String>,
        /// Install a pre-push hook that blocks on incomplete records.
        #[arg(long = "install-hooks")]
        install_hooks: bool,
        /// Remove that hook.
        #[arg(long = "uninstall-hooks", conflicts_with = "install_hooks")]
        uninstall_hooks: bool,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Create an ADR for the current branch, or for the given slug.
    Create {
        /// The record's slug; taken from the branch name when omitted.
        slug: Option<String>,
        /// One-line description, used as the record's title.
        #[arg(short = 'd', long)]
        description: Option<String>,
        /// Branch type: feat, fix, docs or chore (from the branch prefix if omitted).
        #[arg(short = 't', long = "type")]
        branch_type: Option<String>,
        /// Author name (defaults to git config user.name).
        #[arg(long)]
        author: Option<String>,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Gather branch context for filling an ADR in.
    Draft {
        /// The record's slug; inferred from the branch when omitted.
        slug: Option<String>,
        /// Emit a numbered question set instead of the context summary.
        #[arg(long)]
        bootstrap: bool,
        /// Base branch for the commit list and diff.
        #[arg(long)]
        base: Option<String>,
        /// Include pull request comments as reviewer feedback (needs `gh`).
        #[arg(long)]
        pr: bool,
        /// Machine-readable output.
        #[arg(long)]
        json: bool,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Score an ADR for completeness (0-100; exits non-zero below 80).
    Check {
        /// The record's slug.
        slug: String,
        /// Require 95 rather than 80.
        #[arg(long)]
        strict: bool,
        /// Machine-readable score and missing list.
        #[arg(long)]
        json: bool,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Mark an ADR as ready for human review (requires a score of 80).
    Review {
        /// The record's slug.
        slug: String,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// List ADRs.
    List {
        /// Filter by status: proposed, review-requested, accepted, deprecated, superseded.
        #[arg(short = 's', long)]
        status: Option<String>,
        /// Filter by type: feat, fix, docs, chore.
        #[arg(short = 't', long = "type")]
        branch_type: Option<String>,
        /// Machine-readable output.
        #[arg(long)]
        json: bool,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Summarise the ADR collection.
    Status {
        /// Machine-readable output.
        #[arg(long)]
        json: bool,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Update an ADR's metadata.
    Update {
        /// The record's slug.
        slug: String,
        /// Replacement one-line description.
        #[arg(short = 'd', long)]
        description: Option<String>,
        /// New status.
        #[arg(short = 's', long)]
        status: Option<String>,
        /// Replacement comma-separated tag list.
        #[arg(long)]
        tags: Option<String>,
        /// Slug of the ADR that supersedes this one (also sets the status).
        #[arg(long = "superseded-by")]
        superseded_by: Option<String>,
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Regenerate the browsable catalogue from the records' frontmatter.
    Index {
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Wire the ADR hooks into the agent configs this project uses.
    Sync,
    /// Print a shell completion script for `pantheon-adr`.
    Completion {
        /// The shell to generate for.
        shell: Shell,
    },
    /// Manage the bundled companion skill.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },

    /// Print ADR context for agent session injection.
    #[command(hide = true)]
    SessionStart {
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Suggest creating an ADR when the current branch has none.
    #[command(hide = true)]
    PostToolUse {
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Remind about ADRs that are not finished.
    #[command(hide = true)]
    SessionEnd {
        #[command(flatten)]
        dir: DirArgs,
    },
    /// Block a push while any proposed ADR scores below 80.
    #[command(hide = true)]
    PrePush {
        #[command(flatten)]
        dir: DirArgs,
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
            println!("{NAME} v{VERSION}");
            Ok(())
        }
        Command::Init {
            project,
            install_hooks,
            uninstall_hooks,
            dir,
        } => emit(commands::init(
            &adr_dir(dir.dir.as_deref()),
            &cwd(),
            project.as_deref(),
            install_hooks,
            uninstall_hooks,
        )),
        Command::Create {
            slug,
            description,
            branch_type,
            author,
            dir,
        } => emit(commands::create(
            &adr_dir(dir.dir.as_deref()),
            &cwd(),
            &date::today(),
            &commands::CreateArgs {
                slug: slug.as_deref(),
                description: description.as_deref(),
                branch_type: branch_type.as_deref(),
                author: author.as_deref(),
            },
        )),
        Command::Draft {
            slug,
            bootstrap,
            base,
            pr,
            json,
            dir,
        } => emit(commands::draft_cmd(
            &adr_dir(dir.dir.as_deref()),
            &cwd(),
            slug.as_deref(),
            bootstrap,
            base.as_deref(),
            pr,
            json,
        )),
        Command::Check {
            slug,
            strict,
            json,
            dir,
        } => emit_gated(commands::check(
            &adr_dir(dir.dir.as_deref()),
            &slug,
            strict,
            json,
        )),
        Command::Review { slug, dir } => emit(commands::review(
            &adr_dir(dir.dir.as_deref()),
            &slug,
            &date::now_rfc3339(),
        )),
        Command::List {
            status,
            branch_type,
            json,
            dir,
        } => emit(commands::list(
            &adr_dir(dir.dir.as_deref()),
            status.as_deref(),
            branch_type.as_deref(),
            json,
        )),
        Command::Status { json, dir } => {
            emit(commands::status(&adr_dir(dir.dir.as_deref()), &cwd(), json))
        }
        Command::Update {
            slug,
            description,
            status,
            tags,
            superseded_by,
            dir,
        } => emit(commands::update(
            &adr_dir(dir.dir.as_deref()),
            &slug,
            &date::now_rfc3339(),
            &commands::UpdateArgs {
                description: description.as_deref(),
                status: status.as_deref(),
                tags: tags.as_deref(),
                superseded_by: superseded_by.as_deref(),
            },
        )),
        Command::Index { dir } => emit(commands::index_cmd(
            &adr_dir(dir.dir.as_deref()),
            &date::today(),
        )),
        Command::Sync => emit(commands::sync_cmd(&cwd())),
        Command::Completion { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, NAME, &mut std::io::stdout());
            Ok(())
        }

        // Hook commands. Three of these print and never fail, because a hook
        // that breaks a session over an untidy ADR gets uninstalled.
        Command::SessionStart { dir } => {
            print!(
                "{}",
                commands::session_start(&adr_dir(dir.dir.as_deref()), &cwd())
            );
            Ok(())
        }
        Command::PostToolUse { dir } => {
            print!(
                "{}",
                commands::post_tool_use(&adr_dir(dir.dir.as_deref()), &cwd())
            );
            Ok(())
        }
        Command::SessionEnd { dir } => {
            print!("{}", commands::session_end(&adr_dir(dir.dir.as_deref())));
            Ok(())
        }
        Command::PrePush { dir } => {
            let (text, verdict) = commands::pre_push(&adr_dir(dir.dir.as_deref()));
            print!("{text}");
            verdict.map_err(classify)
        }

        Command::Skill {
            action: SkillAction::Install(args),
        } => run_skill_install(args).map_err(|e| (e, EXIT_GENERAL)),
        Command::Skill {
            action: SkillAction::Uninstall(args),
        } => run_skill_uninstall(args).map_err(|e| (e, EXIT_GENERAL)),
    };

    if let Err((message, code)) = result {
        eprintln!("Error: {message}");
        process::exit(code);
    }
}

/// A general failure.
const EXIT_GENERAL: i32 = 1;

/// A record or directory that does not exist.
const EXIT_NOT_FOUND: i32 = 3;

/// The failure message and exit code for an error.
fn classify(e: Error) -> (String, i32) {
    let code = match e {
        Error::NotFound(_) => EXIT_NOT_FOUND,
        _ => EXIT_GENERAL,
    };
    (e.to_string(), code)
}

/// Print a command's output, or turn its error into a message and exit code.
fn emit(result: Result<String>) -> std::result::Result<(), (String, i32)> {
    match result {
        Ok(text) => {
            print!("{text}");
            Ok(())
        }
        Err(e) => Err(classify(e)),
    }
}

/// Print a gating command's report, then honour its verdict.
fn emit_gated(
    result: Result<(String, std::result::Result<(), Error>)>,
) -> std::result::Result<(), (String, i32)> {
    match result {
        Ok((text, verdict)) => {
            print!("{text}");
            verdict.map_err(classify)
        }
        Err(e) => Err(classify(e)),
    }
}

/// The working directory, used to locate the repository and its branch.
fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Resolve the ADR directory from the flag or the `ADR_DIR` env var.
fn adr_dir(flag: Option<&str>) -> PathBuf {
    let env_value = std::env::var(record::DIR_ENV).ok();
    record::resolve_dir(flag, env_value.as_deref())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cli_definition_is_internally_consistent() {
        // clap's own audit: duplicate flags, conflicting names, bad defaults.
        Cli::command().debug_assert();
    }

    #[test]
    fn completion_generates_for_every_supported_shell() {
        for shell in [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ] {
            let mut out = Vec::new();
            generate(shell, &mut Cli::command(), NAME, &mut out);
            let script = String::from_utf8(out).expect("completion scripts are UTF-8");
            assert!(
                script.contains(NAME),
                "{shell} script should name the binary"
            );
        }
    }

    #[test]
    fn the_completion_script_covers_every_subcommand() {
        let mut out = Vec::new();
        generate(Shell::Bash, &mut Cli::command(), NAME, &mut out);
        let script = String::from_utf8(out).expect("utf-8");

        // Hidden hook commands are deliberately absent: they are called by
        // machinery, and offering them for tab completion invites a human to
        // run one by hand.
        for visible in [
            "init", "create", "draft", "check", "review", "list", "status", "update", "index",
            "sync", "skill",
        ] {
            assert!(
                script.contains(visible),
                "bash completion should offer {visible}"
            );
        }
    }
}
