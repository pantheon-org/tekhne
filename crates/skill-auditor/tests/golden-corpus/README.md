# Golden-corpus grade-parity gate (A4b chunk 2)

This harness proves the Rust `skill-auditor` scorer reproduces the Go reference
auditor (`tools/skill-auditor/scorer`) grade-for-grade and dimension-for-dimension
across the whole corpus.

## Verdict: PARITY OK

132/132 corpus skills match the Go auditor: `Total`, `Grade`, every `Dimensions`
entry, `Errors`, `Warnings`, `Lines`, `HasReferences`, `ReferenceCount`,
`ReferenceSectionCompliant` are all exact, and the `ErrorDetails`/`WarningDetails`
sets match. All 132 are grade-exact AND dimension-exact.

## Parity surface

The auditor scores a skill with `scorer.Score(skillPath)`, the same deterministic
entry point the `evaluate` subcommand uses (no LLM, no network). For each corpus
entry `goref/main.go` runs `scorer.Score` and emits the full auditor `Result` as
canonical JSON. `Date` is zeroed (Go uses `time.Now()`) and `Skill` is rewritten
to the repo-root-relative corpus directory so goldens are machine independent.

## Layout

- `corpus.txt` — repo-root-relative skill directories, one per line (`#`
  comments and blanks ignored). Read by BOTH the Go harness and the Rust test.
- `goldens.json` — index of `{dir, result}` records frozen from the Go auditor.
- `goref/` — self-contained Go module that calls `scorer.Score` (via a local
  `replace` onto `tools/skill-auditor`).
- `../grade_parity.rs` — the Rust integration test.

## Corpus (132 skills)

- 2 of the crate's own testdata fixtures under
  `crates/skill-auditor/testdata/fixtures/`: `skill-full` (an A-grade skill with
  `evals/` and `references/`) and `skill-minimal` (a near-empty F-grade skill).
- 130 real tekhne skills: the entire `skills/**/SKILL.md` tree across every
  populated domain (agentic-harness, ci-cd, development, documentation,
  infrastructure, observability, package-mgmt, project-mgmt, repository-mgmt,
  software-engineering, specialized, testing). This is the identical real-skill
  set the A4a validator golden corpus used. Nothing was truncated.

The realised grade spread runs A through F (1 A, 3 B+, 12 B, 24 C+, 42 C, 36 D,
14 F), and includes `skills/documentation/conventional-commits`, whose
frontmatter fails to load, exercising the nil-content path across every
content-driven dimension.

## Comparison rules

- **Exact:** `Total`, `Grade`, `MaxTotal`, every `Dimensions` entry, `Errors`,
  `Warnings`, `Lines`, `HasReferences`, `ReferenceCount`,
  `ReferenceSectionCompliant`. Grades and dimensions are integers, so parity is
  exact; a differing integer is a real Rust bug to fix, never a tolerance to widen.
- **Sorted multiset:** `ErrorDetails` and `WarningDetails` are compared as sorted
  multisets of `(dimension, message)`. Go builds these lists by iterating
  per-dimension diagnostics and the auditor never depends on their order, so
  ordering is not part of the compared surface. Their membership (which bucket a
  diagnostic lands in, error vs warning) still is.
- **Excluded:** `Date` (Go uses `time.Now()`; the Rust test pins it) and `Skill`
  (an absolute path in Go; the harness rewrites it to the relative corpus dir).

## Documented Go non-determinism handled

`ErrorDetails`/`WarningDetails` ordering is the only Go-side non-determinism on
this surface: the Go scorer appends diagnostics per dimension and the underlying
validator walks recognised directories over a Go map. The auditor only ever
consumes these lists order-independently, so the test compares them as sorted
multisets rather than as sequences. Every other compared field is a fixed-order
integer, string or boolean and is asserted exactly.

## Regenerate

```bash
# from this directory; requires a Go toolchain >= 1.25.5 (auto via GOTOOLCHAIN)
grep -v '^#' corpus.txt | (cd goref && GOTOOLCHAIN=auto GOPROXY=off \
    GOFLAGS=-mod=mod go run -buildvcs=false . <repo-root>) > goldens.json
```

## Run the gate

```bash
cargo test -p skill-auditor --test grade_parity -- --nocapture
```
