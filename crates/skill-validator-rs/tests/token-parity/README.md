# S1 token-parity go/no-go gate (A4a)

This harness is the recorded S1 gate for reimplementing the skill validator in
Rust. It proves that `tiktoken-rs` `o200k_base()` produces **bit-exact token
counts** versus the Go auditor's `tiktoken-go` `O200kBase` across a diverse
corpus. Every structural threshold in the Rust validator depends on token
counts matching the Go tool exactly, so any mismatch is a NO-GO.

## Verdict: GO

All 60 corpus items match exactly (16137 total tokens).

## Layout

- `corpus.jsonl` — shared fixture, one JSON-encoded string per line. Both sides
  decode the JSON and tokenize the identical resulting bytes. JSON encoding
  preserves newlines, CRLF, combining marks and multi-codepoint emoji exactly.
- `descriptions.json` — index-aligned human labels (reporting only, never
  tokenized).
- `expected_counts.json` — index-aligned token counts from the Go reference.
- `gen_corpus.mjs` — regenerates `corpus.jsonl` and `descriptions.json`.
- `goref/` — self-contained Go module that tokenizes the corpus with
  `tiktoken-go` and emits `expected_counts.json`.
- `../token_parity.rs` — the Rust integration test that reads the same corpus
  and asserts index-by-index count parity.

## Corpus coverage (60 items)

52 synthetic + 8 real `skills/**/SKILL.md` files (2 KB - 19 KB). Synthetic items
span: empty/single-char/whitespace-only strings; plain prose and a long
paragraph; markdown headings, lists, tables, blockquotes, links, YAML
frontmatter; fenced code in python/rust/bash/json/go; inline code; emoji
(simple, ZWJ family `👨‍👩‍👧`, skin-tone modifier, flag sequences); CJK, Japanese,
Korean, Arabic (RTL); accented latin and NFD combining marks; numbers and mixed
alphanumerics; contractions/possessives; heavy whitespace, blank-line runs,
leading/trailing spaces, tabs, CRLF; URLs and emails; and literal special-token
strings (`<|endoftext|>`, `<|im_start|>`).

## Parity note (special tokens)

`tiktoken-go`'s `Encode` runs only the split regexp plus BPE merges and never
consults its special-token table, so it is equivalent to tiktoken's
`encode_ordinary`. The Rust side therefore uses `encode_ordinary` (NOT
`encode_with_special_tokens`). The corpus deliberately includes literal
`<|endoftext|>` / `<|im_start|>` strings, and both sides tokenize them as
ordinary text — confirming the chosen API pairing.

## Regenerate

```bash
# from this directory
node gen_corpus.mjs
(cd goref && go run . < ../corpus.jsonl > ../expected_counts.json)
# tiktoken-go requires go >= 1.23; use a matching toolchain if the repo pins
# an older go in mise.toml.
```

## Run the gate

```bash
cargo test -p skill-validator-rs --test token_parity -- --nocapture
```
