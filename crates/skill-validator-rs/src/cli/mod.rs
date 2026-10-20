//! `skill-validator-rs`: the standalone offline validator gate (A5b). Wraps the
//! library's deterministic validation surface with a clap CLI, report rendering
//! (text/json/markdown plus GitHub Actions annotations), and the Go tool's
//! exit-code contract (`cmd/exitcode.go`): `0` clean, `1` error, `2` warning,
//! `3` CLI usage error; `--strict` promotes warnings to `1`.
//!
//! Hook-parity invocations (see `hk.pkl`, repointed by A6):
//! - pre-commit: `skill-validator-rs validate structure --allow-dirs=evals <dir>`
//!   (orphan checks run: `--skip-orphans` defaults false).
//! - pre-push:   `skill-validator-rs check --allow-dirs=evals --per-file <dir>`.

mod model;
mod probes_cmd;
mod render;
mod run;

use std::ffi::{OsStr, OsString};
use std::process::ExitCode;

use crate::Options;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use model::{exit_code, CliReport, OutputFormat};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "skill-validator-rs",
    version = VERSION,
    about = "Deterministic, offline skill validation gate",
    long_about = "skill-validator-rs runs the offline, deterministic skill checks (structure, \
internal links, content, contamination) and exits 0 (clean), 1 (error), 2 (warning) or 3 (CLI \
usage error). It is the gate consumed by the git hooks and CI."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Directory names accepted without warning (comma-separated, repeatable).
    #[arg(long, global = true, value_delimiter = ',', value_name = "CSV")]
    allow_dirs: Vec<String>,

    /// Analyse each reference file individually (`check` only).
    #[arg(long, global = true)]
    per_file: bool,

    /// Skip the orphan-file reachability checks.
    #[arg(long, global = true)]
    skip_orphans: bool,

    /// Promote warnings to a failing exit code (exit 1 instead of 2).
    #[arg(long, global = true)]
    strict: bool,

    /// Treat root-level text files as standard content (flat skills).
    #[arg(long, global = true)]
    allow_flat_layouts: bool,

    /// Do not warn about unrecognised frontmatter fields.
    #[arg(long, global = true)]
    allow_extra_frontmatter: bool,

    /// Output format.
    #[arg(short = 'o', long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    output: OutputFormat,

    /// Also emit GitHub Actions `::error`/`::warning` annotation lines.
    #[arg(long, global = true)]
    emit_annotations: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a skill (subgroups select the check group).
    Validate {
        #[command(subcommand)]
        group: ValidateGroup,
    },
    /// Run every group (structure, links, content, contamination) on a skill.
    Check {
        /// The skill directory.
        dir: String,
    },
    /// Invocation probes: would a skill's description win the requests it should?
    /// A lexical estimate, never a model-graded rate; always advisory.
    Probes {
        #[command(subcommand)]
        action: probes_cmd::ProbesAction,
    },
    /// Analyse a skill without validating (subgroups select the analysis).
    Analyze {
        #[command(subcommand)]
        group: AnalyzeGroup,
    },
}

#[derive(Subcommand)]
enum ValidateGroup {
    /// Validate directory layout, frontmatter, tokens, markdown, links, orphans.
    Structure {
        /// The skill directory.
        dir: String,
    },
    /// Validate artifact conventions (script shebangs, schema/template
    /// placement, skill-dir layout, name match, self-containment).
    Artifacts {
        /// Files to validate. With none, the whole `skills/` tree is scanned.
        paths: Vec<String>,
    },
}

#[derive(Subcommand)]
enum AnalyzeGroup {
    /// Report content-quality metrics for SKILL.md.
    Content {
        /// The skill directory.
        dir: String,
    },
    /// Name trigger phrases a skill's description carried at a base and no
    /// longer carries. Advisory: always exits 0.
    Triggers {
        /// The head skill directory.
        dir: String,
        /// The same skill at the base commit (a checkout or worktree). A
        /// directory without a SKILL.md means a new skill.
        #[arg(long)]
        base: String,
    },
    /// Estimate the shared skill-listing description budget for a skills root.
    /// Advisory: always exits 0.
    Listing {
        /// The skills root to scan.
        root: String,
        /// Fixed budget in characters (the documented fallback is 8000).
        #[arg(long)]
        budget_chars: Option<usize>,
        /// Model context window in tokens; budget = tokens x chars/token x fraction.
        #[arg(long)]
        context_tokens: Option<usize>,
        /// `skillListingBudgetFraction` (default 0.01). Without a window, the
        /// result is a band over 200k and 1M tokens.
        #[arg(long)]
        fraction: Option<f64>,
        /// Characters per token used to derive a budget (default 4).
        #[arg(long)]
        chars_per_token: Option<f64>,
        /// Also read this machine's budget: SLASH_COMMAND_TOOL_CHAR_BUDGET and
        /// skillListingBudgetFraction from the Claude settings files. Explicit
        /// flags win.
        #[arg(long)]
        from_settings: bool,
    },
}

impl Cli {
    fn options(&self) -> Options {
        Options {
            skip_orphans: self.skip_orphans,
            allow_extra_frontmatter: self.allow_extra_frontmatter,
            allow_flat_layouts: self.allow_flat_layouts,
            allow_dirs: self.allow_dirs.clone(),
        }
    }
}

/// Run the validator CLI on `args` (the first item is the program name) and
/// return its process exit code. `bin_name` is the name shown in usage and
/// help text, so a host binary can embed this CLI under its own name.
///
/// Parses manually so clap usage errors map to exit code 3 while `--help` and
/// `--version` still exit 0 (the Go tool's `cmd/exitcode.go` contract).
pub fn run_code<I, T>(bin_name: &str, args: I) -> u8
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let command = Cli::command().bin_name(bin_name.to_owned());
    let cli = match command
        .try_get_matches_from(args)
        .and_then(|matches| Cli::from_arg_matches(&matches))
    {
        Ok(cli) => cli,
        Err(err) => return handle_parse_error(err),
    };

    if let Command::Probes { action } = &cli.command {
        return probes_cmd::run(action);
    }

    match dispatch(&cli) {
        Ok((report, gated)) => finish(&cli, &report, gated),
        Err(message) => {
            eprintln!("error: {message}");
            1
        }
    }
}

/// Whether `args` (the first item is the program name) is a validator
/// invocation: validator global flags, then one of its subcommands. A host
/// binary calls this before its own parsing so that flags placed before the
/// subcommand (`--strict validate ...`, `-o json analyze ...`) keep working
/// exactly as on the standalone binary.
///
/// The flag table comes from the clap definition, so a new global flag is
/// recognised without touching this function. Anything else (an unknown flag,
/// another subcommand, `help`, a `--` separator) is not a validator invocation.
pub fn is_invocation<S: AsRef<OsStr>>(args: &[S]) -> bool {
    let mut command = Cli::command();
    command.build();
    let mut rest = args.iter().skip(1);
    while let Some(arg) = rest.next() {
        let Some(arg) = arg.as_ref().to_str() else {
            return false;
        };
        if let Some(long) = arg.strip_prefix("--") {
            let (name, inline_value) = match long.split_once('=') {
                Some((name, _)) => (name, true),
                None => (long, false),
            };
            let Some(flag) = command.get_arguments().find(|a| a.get_long() == Some(name)) else {
                return false;
            };
            if flag.get_action().takes_values() && !inline_value {
                rest.next();
            }
        } else if let Some(shorts) = arg.strip_prefix('-').filter(|s| !s.is_empty()) {
            let Some(flag) = command
                .get_arguments()
                .find(|a| a.get_short() == shorts.chars().next())
            else {
                return false;
            };
            // `-o json` takes its value from the next argument; `-ojson` carries it.
            if flag.get_action().takes_values() && shorts.chars().count() == 1 {
                rest.next();
            }
        } else {
            return arg != "help" && command.get_subcommands().any(|s| s.get_name() == arg);
        }
    }
    false
}

/// [`run_code`] as an [`ExitCode`], for a binary's `main`.
pub fn run<I, T>(bin_name: &str, args: I) -> ExitCode
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    ExitCode::from(run_code(bin_name, args))
}

/// Run the selected command. The bool is whether the exit code is gated on the
/// report's findings (`false` for pure analysis, which is always exit 0).
fn dispatch(cli: &Cli) -> Result<(CliReport, bool), String> {
    match &cli.command {
        Command::Validate {
            group: ValidateGroup::Structure { dir },
        } => Ok((run::validate_structure(dir, &cli.options()), true)),
        Command::Validate {
            group: ValidateGroup::Artifacts { paths },
        } => Ok((run::validate_artifacts(paths), true)),
        Command::Check { dir } => Ok((run::check(dir, &cli.options(), cli.per_file), true)),
        Command::Probes { .. } => unreachable!("handled before dispatch"),
        Command::Analyze {
            group: AnalyzeGroup::Content { dir },
        } => Ok((run::analyze_content_cmd(dir)?, false)),
        Command::Analyze {
            group: AnalyzeGroup::Triggers { dir, base },
        } => Ok((run::analyze_triggers_cmd(dir, base)?, false)),
        Command::Analyze {
            group:
                AnalyzeGroup::Listing {
                    root,
                    budget_chars,
                    context_tokens,
                    fraction,
                    chars_per_token,
                    from_settings,
                },
        } => {
            let inputs = crate::listing::BudgetInputs {
                chars: *budget_chars,
                context_tokens: *context_tokens,
                fraction: *fraction,
                chars_per_token: *chars_per_token,
            };
            Ok((
                run::analyze_listing_cmd(root, inputs, *from_settings)?,
                false,
            ))
        }
    }
}

/// Print the rendered report (and any annotations) and return the exit code.
fn finish(cli: &Cli, report: &CliReport, gated: bool) -> u8 {
    if cli.emit_annotations {
        let annotations = render::render_annotations(report);
        if !annotations.is_empty() {
            // Keep stdout valid for machine consumers when emitting JSON.
            if cli.output == OutputFormat::Json {
                eprint!("{annotations}");
            } else {
                print!("{annotations}");
            }
        }
    }

    print!("{}", render::render(report, cli.output));

    if !gated {
        return 0;
    }
    exit_code(report.errors, report.warnings, cli.strict) as u8
}

/// Map a clap parse outcome to an exit code: `--help`/`--version` exit 0,
/// everything else (unknown flag, missing arg, bad subcommand) exits 3.
fn handle_parse_error(err: clap::Error) -> u8 {
    use clap::error::ErrorKind;
    let _ = err.print();
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => 0,
        _ => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::is_invocation;

    fn argv(rest: &[&str]) -> Vec<String> {
        std::iter::once("pantheon-skill-auditor")
            .chain(rest.iter().copied())
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn plain_subcommands_are_validator_invocations() {
        for sub in ["validate", "check", "probes", "analyze"] {
            assert!(is_invocation(&argv(&[sub, "dir"])), "{sub}");
        }
    }

    #[test]
    fn global_flags_before_the_subcommand_are_skipped() {
        assert!(is_invocation(&argv(&[
            "--strict",
            "validate",
            "structure",
            "d"
        ])));
        assert!(is_invocation(&argv(&[
            "-o", "json", "analyze", "listing", "r"
        ])));
        assert!(is_invocation(&argv(&["-ojson", "check", "d"])));
        assert!(is_invocation(&argv(&["--output=json", "check", "d"])));
        assert!(is_invocation(&argv(&[
            "--allow-dirs",
            "evals",
            "check",
            "d"
        ])));
        assert!(is_invocation(&argv(&[
            "--allow-dirs=evals",
            "--per-file",
            "check",
            "d"
        ])));
    }

    #[test]
    fn a_flag_value_is_not_mistaken_for_the_subcommand() {
        // `check` here is the value of --allow-dirs, so no subcommand follows.
        assert!(!is_invocation(&argv(&["--allow-dirs", "check"])));
    }

    #[test]
    fn other_invocations_are_left_to_the_host() {
        assert!(!is_invocation(&argv(&[])));
        assert!(!is_invocation(&argv(&["evaluate", "skill"])));
        assert!(!is_invocation(&argv(&["help"])));
        assert!(!is_invocation(&argv(&["help", "validate"])));
        assert!(!is_invocation(&argv(&["--unknown", "validate"])));
        assert!(!is_invocation(&argv(&["--", "validate"])));
        assert!(!is_invocation(&argv(&["--version"])));
    }
}
