//! `skill-auditor` CLI, ported from the Go cobra command surface: `version`,
//! `evaluate <skill>` and `batch <skill...>`, with `--json`, `--store`,
//! `--repo-root` and (batch only) `--fail-below`.

use clap::{Parser, Subcommand};
use skill_auditor::reporter;
use skill_auditor::scorer::{self, grade_rank, Result as AuditResult};
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
