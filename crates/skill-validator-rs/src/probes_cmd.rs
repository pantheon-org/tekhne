//! `probes validate|score|compare`: the invocation-probe commands. They print
//! their own output (text findings or JSON), so they sit beside the
//! [`CliReport`](crate::model::CliReport) flow rather than inside it.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Subcommand;
use skill_validator_rs::probes::{self, ScoreReport, DEFAULT_COPY_SPAN};

#[derive(Subcommand)]
pub enum ProbesAction {
    /// Check probe files: schema, query counts, labels, splits, resolvable
    /// skills, and should-trigger probes that copy the listing. Exits 1 on an
    /// error; warnings do not fail.
    Validate {
        /// Directory of probe files (one JSON file per skill).
        dir: PathBuf,
        /// Repository root the probe paths are relative to.
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        /// Consecutive shared words that flag a should-trigger probe as copied.
        #[arg(long, default_value_t = DEFAULT_COPY_SPAN)]
        copy_span: usize,
    },
    /// Score the probes with the lexical floor and print the report as JSON.
    /// Advisory: exits 0 unless a probe file or skill cannot be read.
    Score {
        /// Directory of probe files.
        dir: PathBuf,
        /// Repository root the probe paths are relative to.
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
    },
    /// Print treatment minus baseline per skill and split, with a paired 95%
    /// interval, from two `score` reports.
    Compare {
        /// The baseline `score` report.
        baseline: PathBuf,
        /// The treatment `score` report.
        treatment: PathBuf,
    },
}

pub fn run(action: &ProbesAction) -> ExitCode {
    match execute(action) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(1)
        }
    }
}

fn execute(action: &ProbesAction) -> Result<ExitCode, String> {
    match action {
        ProbesAction::Validate {
            dir,
            repo_root,
            copy_span,
        } => {
            let files = probes::load_dir(dir)?;
            let findings = probes::validate(&files, repo_root, *copy_span);
            for f in &findings {
                println!(
                    "{}: {}: {}",
                    if f.error { "FAIL" } else { "WARN" },
                    f.file,
                    f.message
                );
            }
            let errors = findings.iter().filter(|f| f.error).count();
            println!(
                "{} probe file(s): {errors} error(s), {} warning(s)",
                files.len(),
                findings.len() - errors
            );
            Ok(ExitCode::from(u8::from(errors > 0)))
        }
        ProbesAction::Score { dir, repo_root } => {
            let files = probes::load_dir(dir)?;
            let report = probes::score(&files, repo_root)?;
            for s in &report.skills {
                let rate = |r: Option<f64>| r.map_or("n/a".to_string(), |x| format!("{x:.2}"));
                let (t, v) = (
                    &s.splits[&probes::Split::Train],
                    &s.splits[&probes::Split::Validation],
                );
                eprintln!(
                    "{} {}  train trigger_rate={} false_trigger_rate={}  validation \
                     trigger_rate={} false_trigger_rate={}",
                    s.skill,
                    probes::METHOD,
                    rate(t.trigger_rate),
                    rate(t.false_trigger_rate),
                    rate(v.trigger_rate),
                    rate(v.false_trigger_rate),
                );
            }
            print_json(&report)?;
            Ok(ExitCode::SUCCESS)
        }
        ProbesAction::Compare {
            baseline,
            treatment,
        } => {
            let report = probes::compare(&read_report(baseline)?, &read_report(treatment)?)?;
            for s in &report.skills {
                for (name, d) in [("train", &s.train), ("validation", &s.validation)] {
                    if let (Some(delta), Some(iv)) =
                        (d.trigger_rate_delta, d.trigger_rate_delta_interval)
                    {
                        eprintln!(
                            "{} {name} trigger_rate delta {delta:.3} 95% interval [{}, {}]{}",
                            s.skill,
                            iv[0],
                            iv[1],
                            if d.trigger_rate_within_noise == Some(true) {
                                ": within noise"
                            } else {
                                ""
                            }
                        );
                    }
                }
            }
            print_json(&report)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn read_report(path: &Path) -> Result<ScoreReport, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parsing {}: {e}", path.display()))
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    println!("{json}");
    Ok(())
}
