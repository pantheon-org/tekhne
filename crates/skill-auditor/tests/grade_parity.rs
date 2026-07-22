//! Golden-corpus grade-parity gate (A4b chunk 2).
//!
//! Proves the Rust `skill-auditor` scorer reproduces the Go auditor
//! (`tools/skill-auditor/scorer`) grade-for-grade and dimension-for-dimension
//! across the whole corpus. The goldens under
//! `tests/golden-corpus/goldens.json` were frozen by running the Go scorer
//! (`scorer.Score`, the same entry the `evaluate` subcommand uses) over the
//! corpus in `tests/golden-corpus/corpus.txt`. Regenerate with (from
//! `tests/golden-corpus/`):
//!
//! ```text
//! grep -v '^#' corpus.txt | (cd goref && GOTOOLCHAIN=auto GOPROXY=off \
//!     GOFLAGS=-mod=mod go run -buildvcs=false . <repo-root>) > goldens.json
//! ```
//!
//! Tolerance: grades and dimension scores are integers, so parity is EXACT for
//! `total`, `grade`, `max_total`, every `dimensions` entry, `errors`,
//! `warnings`, `lines`, `has_references`, `reference_count` and
//! `reference_section_compliant`. `error_details`/`warning_details` are compared
//! as SORTED multisets of `(dimension, message)`: Go builds these lists by
//! iterating per-dimension diagnostics and the auditor never depends on their
//! order, so sequence is not part of the compared surface (their membership,
//! i.e. which diagnostics landed in the error vs warning bucket, still is).
//! `date` is EXCLUDED (Go uses `time.Now()`; the Rust side pins it). Fix Rust to
//! match Go; never loosen the comparison.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use skill_auditor::scorer::{score_with_date, Diagnostic, Result as ScoreResult};

/// Fixed date injected into the Rust scorer so scoring is deterministic. Its
/// value is irrelevant to parity because `date` is excluded from the diff.
const PINNED_DATE: &str = "2026-07-22";

#[derive(Debug, Deserialize)]
struct Golden {
    dir: String,
    result: ScoreResult,
}

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden-corpus")
}

/// Repository root: the crate lives at `<root>/crates/skill-auditor`.
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

/// A diagnostic reduced to its comparable identity: `(dimension, message)`. The
/// `severity` field is not serialised by either side (it only decides the
/// error-vs-warning bucket, which is already encoded by list membership).
type DiagKey = (String, String);

fn diag_keys(diags: &[Diagnostic]) -> Vec<DiagKey> {
    let mut keys: Vec<DiagKey> = diags
        .iter()
        .map(|d| (d.dimension.clone(), d.message.clone()))
        .collect();
    keys.sort();
    keys
}

/// Compare one skill's Go golden against the Rust result; push one diff line per
/// divergence. Every field is exact except `date` (excluded) and the ordering of
/// the detail lists (sorted-multiset compared).
fn diff_result(dir: &str, want: &ScoreResult, got: &ScoreResult, out: &mut Vec<String>) {
    macro_rules! exact {
        ($field:ident, $label:literal) => {
            if want.$field != got.$field {
                out.push(format!(
                    "  [{dir}] {}: go={:?} rs={:?}",
                    $label, want.$field, got.$field
                ));
            }
        };
    }
    exact!(total, "total");
    exact!(grade, "grade");
    exact!(max_total, "maxTotal");
    exact!(lines, "lines");
    exact!(has_references, "hasReferences");
    exact!(reference_count, "referenceCount");
    exact!(reference_section_compliant, "referenceSectionCompliant");
    exact!(errors, "errors");
    exact!(warnings, "warnings");

    // Dimensions: compare every key/value (both are sorted BTreeMaps).
    if want.dimensions != got.dimensions {
        let mut keys: Vec<&String> = want
            .dimensions
            .keys()
            .chain(got.dimensions.keys())
            .collect();
        keys.sort();
        keys.dedup();
        for k in keys {
            let w = want.dimensions.get(k);
            let g = got.dimensions.get(k);
            if w != g {
                out.push(format!("  [{dir}] dimensions.{k}: go={w:?} rs={g:?}"));
            }
        }
    }

    // Detail lists: sorted multisets of (dimension, message).
    if diag_keys(&want.error_details) != diag_keys(&got.error_details) {
        out.push(format!(
            "  [{dir}] errorDetails: go={:?} rs={:?}",
            diag_keys(&want.error_details),
            diag_keys(&got.error_details)
        ));
    }
    if diag_keys(&want.warning_details) != diag_keys(&got.warning_details) {
        out.push(format!(
            "  [{dir}] warningDetails: go={:?} rs={:?}",
            diag_keys(&want.warning_details),
            diag_keys(&got.warning_details)
        ));
    }
}

#[test]
fn rust_auditor_matches_go_golden_corpus() {
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
    let mut grade_and_dim_exact = 0usize;
    for rel in &corpus {
        let golden = by_dir
            .get(rel.as_str())
            .unwrap_or_else(|| panic!("no golden for corpus entry {rel}"));
        let skill_path = root.join(rel).join("SKILL.md");
        let got = score_with_date(&skill_path, PINNED_DATE)
            .unwrap_or_else(|e| panic!("score {}: {e}", skill_path.display()));
        compared += 1;

        let before = diffs.len();
        diff_result(rel, &golden.result, &got, &mut diffs);
        if diffs.len() == before
            && golden.result.grade == got.grade
            && golden.result.dimensions == got.dimensions
        {
            grade_and_dim_exact += 1;
        }
    }

    if !diffs.is_empty() {
        panic!(
            "PARITY FAIL: {} divergence(s) across {} corpus skills vs Go auditor:\n{}",
            diffs.len(),
            compared,
            diffs.join("\n"),
        );
    }

    eprintln!(
        "PARITY OK: {grade_and_dim_exact}/{compared} corpus skills grade-exact AND \
         dimension-exact vs the Go skill-auditor (all Result fields exact bar the \
         excluded date; detail lists compared as sorted multisets)."
    );
}
