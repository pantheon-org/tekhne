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
//! The goldens matched the pinned Go module at freeze time. Go is now retired
//! (#212), so `tests/golden-corpus/goldens.json` is a Rust regression baseline:
//! regenerate it from the Rust validator (the reference of record) and review
//! the diff:
//!
//! ```text
//! BLESS_GOLDENS=1 cargo test -p skill-validator-rs --test golden_parity
//! ```
//!
//! The corpus has two tiers, because pinning exact output for live repository
//! content made the gate a change-detector for skill text rather than for
//! validator behaviour. Any skill edit, or a release rewriting CHANGELOGs,
//! invalidated the baseline; and since CI does not run this crate on a
//! `skills/**` change, it rotted silently instead of failing.
//!
//!   * **Frozen fixtures** (everything outside `skills/`) are pinned exactly.
//!     Tolerance (spec section 6): counts, enums, strings, token counts and the
//!     full `Results[]` must match EXACTLY; derived 4dp floats match within
//!     absolute `5e-5`. Never loosen the band.
//!   * **Live skills** (`skills/**`) are checked against invariants that hold
//!     whatever the skill says: results are non-empty, the error and warning
//!     tallies agree with the results' levels, and a content report that is
//!     produced has a non-zero word count and in-range ratios.
//!
//! Only the frozen tier is written to `goldens.json`. To widen exact coverage,
//! add a fixture rather than a `skills/` entry.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use skill_validator_rs::{
    analyze_content, validate, ContentReport, Level, Options, TokenCount, ValidationResult,
};

/// Absolute tolerance for derived 4dp floats (spec section 6).
const FLOAT_TOL: f64 = 5e-5;

#[derive(Debug, Serialize, Deserialize)]
struct StructOut {
    results: Vec<ValidationResult>,
    token_counts: Vec<TokenCount>,
    other_token_counts: Vec<TokenCount>,
    errors: usize,
    warnings: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Golden {
    dir: String,
    content: Option<ContentReport>,
    structure: StructOut,
}

/// When `BLESS_GOLDENS` is set, regenerate `goldens.json` from the Rust
/// validator (the reference of record now that Go is retired; #212) and return
/// true so the caller skips the assertions.
fn bless_if_requested(corpus: &[String], root: &Path) -> bool {
    if std::env::var_os("BLESS_GOLDENS").is_none() {
        return false;
    }
    let goldens: Vec<Golden> = corpus
        .iter()
        .filter(|rel| !is_live_skill(rel))
        .map(|rel| {
            let dir = root.join(rel);
            Golden {
                dir: rel.clone(),
                content: actual_content(&dir),
                structure: actual_structure(&dir),
            }
        })
        .collect();
    let json = serde_json::to_string_pretty(&goldens).expect("serialize goldens") + "\n";
    fs::write(corpus_dir().join("goldens.json"), json).expect("write goldens.json");
    eprintln!("BLESSED {} goldens from the Rust validator", goldens.len());
    true
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

/// Corpus entries split into two tiers by whether their input can change.
///
/// Fixtures under this crate are frozen, so their output is pinned exactly.
/// Entries under `skills/` are live repository content that any skill edit or
/// release CHANGELOG rewrite legitimately changes; pinning those made the gate
/// a change-detector for skill text rather than for validator behaviour, and it
/// silently rotted whenever CI did not happen to run. They are checked for
/// invariants instead, which is what actually catches a broken validator.
fn is_live_skill(rel: &str) -> bool {
    rel.starts_with("skills/")
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

/// Invariants every validator run must satisfy whatever the skill says.
///
/// These hold independently of the skill's text, so editing a skill can never
/// break them, but a validator that miscounts levels, returns nothing, or emits
/// an out-of-range ratio still fails. Verified to hold across all live corpus
/// entries at the time of the split.
fn check_invariants(rel: &str, dir: &Path, out: &mut Vec<String>) {
    let structure = actual_structure(dir);

    if structure.results.is_empty() {
        out.push(format!("  [{rel}] structure.results is empty"));
    }

    let errors = count_at_level(&structure, Level::Error);
    if errors != structure.errors {
        out.push(format!(
            "  [{rel}] structure.errors={} but {errors} result(s) are Error",
            structure.errors
        ));
    }

    let warnings = count_at_level(&structure, Level::Warning);
    if warnings != structure.warnings {
        out.push(format!(
            "  [{rel}] structure.warnings={} but {warnings} result(s) are Warning",
            structure.warnings
        ));
    }

    // A skill that fails to load yields no content report; that is a valid
    // outcome, so only a report that IS produced is constrained.
    if let Some(content) = actual_content(dir) {
        if content.word_count == 0 {
            out.push(format!("  [{rel}] content.word_count is 0"));
        }
        for (label, ratio) in [
            ("code_block_ratio", content.code_block_ratio),
            ("imperative_ratio", content.imperative_ratio),
            ("information_density", content.information_density),
            ("instruction_specificity", content.instruction_specificity),
        ] {
            if !(0.0..=1.0).contains(&ratio) {
                out.push(format!(
                    "  [{rel}] content.{label}={ratio} outside 0.0..=1.0"
                ));
            }
        }
    }
}

fn count_at_level(structure: &StructOut, level: Level) -> usize {
    structure
        .results
        .iter()
        .filter(|r| r.level.as_int() == level.as_int())
        .count()
}

#[test]
fn rust_validator_matches_go_golden_corpus() {
    let corpus = read_corpus();
    let root = repo_root();

    if bless_if_requested(&corpus, &root) {
        return;
    }

    let (live, frozen): (Vec<&String>, Vec<&String>) =
        corpus.iter().partition(|rel| is_live_skill(rel));

    let goldens = read_goldens();
    assert_eq!(
        frozen.len(),
        goldens.len(),
        "frozen corpus entries ({}) and goldens.json ({}) are not aligned; regenerate goldens",
        frozen.len(),
        goldens.len(),
    );

    let by_dir: HashMap<&str, &Golden> = goldens.iter().map(|g| (g.dir.as_str(), g)).collect();

    let mut diffs: Vec<String> = Vec::new();

    // Frozen fixtures: output pinned exactly.
    for rel in &frozen {
        let golden = by_dir
            .get(rel.as_str())
            .unwrap_or_else(|| panic!("no golden for corpus entry {rel}"));
        let dir = root.join(rel);

        // Content parity (bridge call 1).
        match (&golden.content, actual_content(&dir)) {
            (Some(want), Some(got)) => diff_content(rel, want, &got, &mut diffs),
            (None, None) => {}
            (Some(_), None) => diffs.push(format!("  [{rel}] content: want=Some got=None")),
            (None, Some(_)) => diffs.push(format!("  [{rel}] content: want=None got=Some")),
        }

        // Structure parity (bridge call 2).
        let got = actual_structure(&dir);
        diff_structure(rel, &golden.structure, &got, &mut diffs);
    }

    // Live skills: invariants only, so a skill edit cannot break the gate.
    for rel in &live {
        check_invariants(rel, &root.join(rel), &mut diffs);
    }

    if !diffs.is_empty() {
        panic!(
            "PARITY FAIL: {} divergence(s) across {} frozen fixture(s) and {} live skill(s):\n{}",
            diffs.len(),
            frozen.len(),
            live.len(),
            diffs.join("\n"),
        );
    }

    eprintln!(
        "PARITY OK: {} frozen fixture(s) bit-exact (counts/enums/strings/token-counts exact; \
         4dp floats within {FLOAT_TOL:.0e}); {} live skill(s) satisfy the validator invariants.",
        frozen.len(),
        live.len(),
    );
}
