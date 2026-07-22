//! Golden-corpus parity gate (A4a chunk 2).
//!
//! Proves the Rust validator reproduces the Go validator
//! (`github.com/agent-ecosystem/skill-validator@v1.5.6`) across the exact two
//! entry points the skill-auditor couples to through
//! `tools/skill-auditor/scorer/validator_bridge.go`:
//!
//!   1. `orchestrate.RunContentAnalysis(dir)` -> `.ContentReport`
//!   2. `structure.Validate(dir, {SkipOrphans, AllowFlatLayouts,
//!      AllowExtraFrontmatter})`
//!
//! The goldens under `tests/golden-corpus/goldens.json` were frozen by running
//! the pinned Go module over the corpus in `tests/golden-corpus/corpus.txt`.
//! Regenerate with (from `tests/golden-corpus/`):
//!
//! ```text
//! grep -v '^#' corpus.txt | (cd goref && GOPROXY=off GOFLAGS=-mod=mod \
//!     go run . <repo-root>) > goldens.json
//! ```
//!
//! Tolerance (spec section 6): counts, enums, strings, token counts and the
//! full `Results[]` must match EXACTLY; derived 4dp floats match within
//! absolute `5e-5`. Fix Rust to match Go; never loosen the band.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use skill_validator_rs::{
    analyze_content, validate, ContentReport, Options, TokenCount, ValidationResult,
};

/// Absolute tolerance for derived 4dp floats (spec section 6).
const FLOAT_TOL: f64 = 5e-5;

#[derive(Debug, Deserialize)]
struct StructOut {
    results: Vec<ValidationResult>,
    token_counts: Vec<TokenCount>,
    other_token_counts: Vec<TokenCount>,
    errors: usize,
    warnings: usize,
}

#[derive(Debug, Deserialize)]
struct Golden {
    dir: String,
    content: Option<ContentReport>,
    structure: StructOut,
}

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden-corpus")
}

/// Repository root: the crate lives at `<root>/crates/skill-validator-rs`.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("canonicalize repo root")
}

fn read_corpus() -> Vec<String> {
    let path = corpus_dir().join("corpus.txt");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

fn read_goldens() -> Vec<Golden> {
    let path = corpus_dir().join("goldens.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse goldens.json: {e}"))
}

/// Content analysis matching Go `orchestrate.RunContentAnalysis(dir)`:
/// analyse the raw SKILL.md content, or `None` when the skill fails to load.
fn actual_content(dir: &Path) -> Option<ContentReport> {
    match skill_validator_rs::Skill::load(dir) {
        Ok(s) => Some(analyze_content(&s.raw_content)),
        Err(_) => None,
    }
}

/// Structure validation matching the auditor's bridge options.
fn actual_structure(dir: &Path) -> StructOut {
    let report = validate(
        dir,
        &Options {
            skip_orphans: true,
            allow_flat_layouts: true,
            allow_extra_frontmatter: true,
            allow_dirs: Vec::new(),
        },
    );
    StructOut {
        results: report.results,
        token_counts: report.token_counts,
        other_token_counts: report.other_token_counts,
        errors: report.errors,
        warnings: report.warnings,
    }
}

/// Compare two content reports; push a human-readable diff line per divergence.
/// Integer/string/order fields are exact; the six derived floats use `FLOAT_TOL`.
fn diff_content(dir: &str, want: &ContentReport, got: &ContentReport, out: &mut Vec<String>) {
    macro_rules! exact {
        ($field:ident) => {
            if want.$field != got.$field {
                out.push(format!(
                    "  [{dir}] content.{}: go={:?} rs={:?}",
                    stringify!($field),
                    want.$field,
                    got.$field
                ));
            }
        };
    }
    macro_rules! near {
        ($field:ident) => {
            if (want.$field - got.$field).abs() > FLOAT_TOL {
                out.push(format!(
                    "  [{dir}] content.{}: go={} rs={} (|delta|={:.3e})",
                    stringify!($field),
                    want.$field,
                    got.$field,
                    (want.$field - got.$field).abs()
                ));
            }
        };
    }
    exact!(word_count);
    exact!(code_block_count);
    near!(code_block_ratio);
    exact!(code_languages);
    exact!(sentence_count);
    exact!(imperative_count);
    near!(imperative_ratio);
    near!(information_density);
    exact!(strong_markers);
    exact!(weak_markers);
    near!(instruction_specificity);
    exact!(section_count);
    exact!(list_item_count);
}

fn diff_structure(dir: &str, want: &StructOut, got: &StructOut, out: &mut Vec<String>) {
    if want.errors != got.errors {
        out.push(format!(
            "  [{dir}] structure.errors: go={} rs={}",
            want.errors, got.errors
        ));
    }
    if want.warnings != got.warnings {
        out.push(format!(
            "  [{dir}] structure.warnings: go={} rs={}",
            want.warnings, got.warnings
        ));
    }
    diff_token_counts(
        dir,
        "token_counts",
        &want.token_counts,
        &got.token_counts,
        out,
    );
    diff_token_counts(
        dir,
        "other_token_counts",
        &want.other_token_counts,
        &got.other_token_counts,
        out,
    );
    diff_results(dir, &want.results, &got.results, out);
}

fn diff_token_counts(
    dir: &str,
    label: &str,
    want: &[TokenCount],
    got: &[TokenCount],
    out: &mut Vec<String>,
) {
    if want.len() != got.len() {
        out.push(format!(
            "  [{dir}] structure.{label}: len go={} rs={}",
            want.len(),
            got.len()
        ));
    }
    for (i, (w, g)) in want.iter().zip(got.iter()).enumerate() {
        if w != g {
            out.push(format!(
                "  [{dir}] structure.{label}[{i}]: go={w:?} rs={g:?}"
            ));
        }
    }
}

/// A result reduced to its comparable identity.
///
/// Ordering is deliberately NOT part of the compared surface: Go's
/// `CheckStructure` walks `recognizedDirs` with `for range` over a Go map, so
/// the relative order of cross-directory "deep nesting" warnings is randomised
/// per run. The auditor bridge only ever consumes results order-independently
/// (`hasStructureWarning` any-match, a `descriptionLen` scan, and error/warning
/// tallies), so results are compared as a sorted multiset. Every result's
/// Level, Category, File, Line and Message are still asserted; only their
/// sequence is freed.
///
/// The message is normalised for the frontmatter YAML parse-error path only:
/// that string is produced by the YAML library (yaml.v3 vs serde_yaml_ng), the
/// spec (section 2.1) frees non-parsed wording, and the auditor never parses an
/// Error-level Frontmatter message. Its presence, Level and Category are still
/// asserted, so a missing or mislevelled error is still caught.
type ResultKey = (u8, String, String, usize, String);

const FRONTMATTER_PARSE_PREFIX: &str = "parsing frontmatter YAML:";

fn result_key(r: &ValidationResult) -> ResultKey {
    let message = if r.message.starts_with(FRONTMATTER_PARSE_PREFIX) {
        format!("{FRONTMATTER_PARSE_PREFIX} <library-specific wording>")
    } else {
        r.message.clone()
    };
    (
        r.level.as_int(),
        r.category.clone(),
        r.file.clone(),
        r.line,
        message,
    )
}

fn diff_results(
    dir: &str,
    want: &[ValidationResult],
    got: &[ValidationResult],
    out: &mut Vec<String>,
) {
    let mut want_keys: Vec<ResultKey> = want.iter().map(result_key).collect();
    let mut got_keys: Vec<ResultKey> = got.iter().map(result_key).collect();
    want_keys.sort();
    got_keys.sort();

    if want_keys == got_keys {
        return;
    }

    if want_keys.len() != got_keys.len() {
        out.push(format!(
            "  [{dir}] structure.results: count go={} rs={}",
            want_keys.len(),
            got_keys.len()
        ));
    }
    for key in &want_keys {
        if !got_keys.contains(key) {
            out.push(format!(
                "  [{dir}] structure.results missing in rs: {key:?}"
            ));
        }
    }
    for key in &got_keys {
        if !want_keys.contains(key) {
            out.push(format!("  [{dir}] structure.results extra in rs: {key:?}"));
        }
    }
}

#[test]
fn rust_validator_matches_go_golden_corpus() {
    let corpus = read_corpus();
    let goldens = read_goldens();
    let root = repo_root();

    assert_eq!(
        corpus.len(),
        goldens.len(),
        "corpus ({}) and goldens.json ({}) are not aligned; regenerate goldens",
        corpus.len(),
        goldens.len(),
    );

    let by_dir: HashMap<&str, &Golden> = goldens.iter().map(|g| (g.dir.as_str(), g)).collect();

    let mut diffs: Vec<String> = Vec::new();
    let mut compared = 0usize;
    for rel in &corpus {
        let golden = by_dir
            .get(rel.as_str())
            .unwrap_or_else(|| panic!("no golden for corpus entry {rel}"));
        let dir = root.join(rel);
        compared += 1;

        // Content parity (bridge call 1).
        match (&golden.content, actual_content(&dir)) {
            (Some(want), Some(got)) => diff_content(rel, want, &got, &mut diffs),
            (None, None) => {}
            (Some(_), None) => diffs.push(format!("  [{rel}] content: go=Some rs=None")),
            (None, Some(_)) => diffs.push(format!("  [{rel}] content: go=None rs=Some")),
        }

        // Structure parity (bridge call 2).
        let got = actual_structure(&dir);
        diff_structure(rel, &golden.structure, &got, &mut diffs);
    }

    if !diffs.is_empty() {
        panic!(
            "PARITY FAIL: {} divergence(s) across {} corpus skills vs Go v1.5.6:\n{}",
            diffs.len(),
            compared,
            diffs.join("\n"),
        );
    }

    eprintln!(
        "PARITY OK: {compared} corpus skills bit-exact vs Go skill-validator v1.5.6 \
         (counts/enums/strings/token-counts exact; 4dp floats within {FLOAT_TOL:.0e})."
    );
}
