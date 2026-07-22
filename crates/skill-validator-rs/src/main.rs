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
mod render;
mod run;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use skill_validator_rs::Options;

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
}

#[derive(Subcommand)]
enum AnalyzeGroup {
    /// Report content-quality metrics for SKILL.md.
    Content {
        /// The skill directory.
        dir: String,
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

fn main() -> ExitCode {
    // Parse manually so clap usage errors map to exit code 3 while `--help` and
    // `--version` still exit 0 (the Go tool's `cmd/exitcode.go` contract).
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => return handle_parse_error(err),
    };

    match dispatch(&cli) {
        Ok((report, gated)) => finish(&cli, &report, gated),
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(1)
        }
    }
}

/// Run the selected command. The bool is whether the exit code is gated on the
/// report's findings (`false` for pure analysis, which is always exit 0).
fn dispatch(cli: &Cli) -> Result<(CliReport, bool), String> {
    match &cli.command {
        Command::Validate {
            group: ValidateGroup::Structure { dir },
        } => Ok((run::validate_structure(dir, &cli.options()), true)),
        Command::Check { dir } => Ok((run::check(dir, &cli.options(), cli.per_file), true)),
        Command::Analyze {
            group: AnalyzeGroup::Content { dir },
        } => Ok((run::analyze_content_cmd(dir)?, false)),
    }
}

/// Print the rendered report (and any annotations) and return the exit code.
fn finish(cli: &Cli, report: &CliReport, gated: bool) -> ExitCode {
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
        return ExitCode::SUCCESS;
    }
    ExitCode::from(exit_code(report.errors, report.warnings, cli.strict) as u8)
}

/// Map a clap parse outcome to an exit code: `--help`/`--version` exit 0,
/// everything else (unknown flag, missing arg, bad subcommand) exits 3.
fn handle_parse_error(err: clap::Error) -> ExitCode {
    use clap::error::ErrorKind;
    let _ = err.print();
    match err.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => ExitCode::SUCCESS,
        _ => ExitCode::from(3),
    }
}
