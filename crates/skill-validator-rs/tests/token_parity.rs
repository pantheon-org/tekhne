//! S1 token-parity go/no-go gate (A4a).
//!
//! Proves `tiktoken-rs` `o200k_base()` produces bit-exact token COUNTS versus
//! the Go auditor's `tiktoken-go` `O200kBase` across a diverse corpus. Every
//! structural threshold in the Rust validator depends on these counts matching
//! the Go tool exactly, so any mismatch here is a NO-GO for A4a.
//!
//! Inputs (all committed under `tests/token-parity/`):
//!   * `corpus.jsonl`         one JSON-encoded string per line (shared bytes)
//!   * `expected_counts.json` index-aligned counts from the Go reference
//!   * `descriptions.json`    index-aligned human labels (reporting only)
//!
//! Regenerate the reference with:
//!   node tests/token-parity/gen_corpus.mjs
//!   (cd tests/token-parity/goref && go run . < ../corpus.jsonl > ../expected_counts.json)
//!
//! Parity note: `tiktoken-go`'s `Encode` runs only the split regexp + BPE
//! merges and never consults its special-token table, so it is equivalent to
//! tiktoken's `encode_ordinary`. We therefore compare against
//! `encode_ordinary`; the corpus deliberately includes literal `<|endoftext|>`
//! and `<|im_start|>` strings to confirm both sides treat them as ordinary text.

use std::fs;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/token-parity")
}

fn load_corpus() -> Vec<String> {
    let path = fixture_dir().join("corpus.jsonl");
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    raw.lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, line)| {
            serde_json::from_str::<String>(line)
                .unwrap_or_else(|e| panic!("corpus line {i} is not a JSON string: {e}"))
        })
        .collect()
}

fn load_json_array<T: serde::de::DeserializeOwned>(name: &str) -> Vec<T> {
    let path = fixture_dir().join(name);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {name}: {e}"))
}

#[test]
fn o200k_base_token_counts_match_go_reference() {
    let corpus = load_corpus();
    let expected: Vec<usize> = load_json_array("expected_counts.json");
    let descriptions: Vec<String> = load_json_array("descriptions.json");

    assert_eq!(
        corpus.len(),
        expected.len(),
        "corpus ({}) and expected_counts ({}) are not index-aligned; regenerate the Go reference",
        corpus.len(),
        expected.len(),
    );
    assert_eq!(
        corpus.len(),
        descriptions.len(),
        "corpus ({}) and descriptions ({}) are not index-aligned; regenerate the corpus",
        corpus.len(),
        descriptions.len(),
    );

    let bpe = tiktoken_rs::o200k_base().expect("load o200k_base");

    let mut mismatches: Vec<String> = Vec::new();
    for (i, text) in corpus.iter().enumerate() {
        // encode_ordinary matches tiktoken-go's Encode (no special-token split).
        let rs_count = bpe.encode_ordinary(text).len();
        let go_count = expected[i];
        if rs_count != go_count {
            mismatches.push(format!(
                "  [{i}] {desc}: go={go_count} rs={rs_count} (delta {delta:+})",
                desc = descriptions[i],
                delta = rs_count as i64 - go_count as i64,
            ));
        }
    }

    if !mismatches.is_empty() {
        panic!(
            "NO-GO: {}/{} corpus items diverge between tiktoken-rs o200k_base and tiktoken-go O200kBase:\n{}",
            mismatches.len(),
            corpus.len(),
            mismatches.join("\n"),
        );
    }

    let total: usize = expected.iter().sum();
    eprintln!(
        "GO: {} corpus items, {} total tokens, bit-exact tiktoken-rs vs tiktoken-go parity.",
        corpus.len(),
        total,
    );
}
