# Golden-corpus parity gate (A4a chunk 2)

This harness proves the Rust validator (`skill-validator-rs`) reproduces the Go
reference (`github.com/agent-ecosystem/skill-validator@v1.5.6`) across the exact
surface the skill-auditor consumes.

## Verdict: PARITY OK

152/152 corpus skills match the Go reference: counts, enums, strings and
per-file token counts exact; the full `Results[]` set matches; the six derived
4dp floats match within absolute `5e-5`.

## Parity surface (why these two calls)

The auditor couples to the validator through
`tools/skill-auditor/scorer/validator_bridge.go` via exactly two calls, so those
are the surface that matters (spec section 18 / 2.1):

1. `orchestrate.RunContentAnalysis(dir)`, keeping `.ContentReport`.
2. `structure.Validate(dir, {SkipOrphans: true, AllowFlatLayouts: true,
   AllowExtraFrontmatter: true})`.

`goref/main.go` invokes these two and emits, per skill, the bridge-consumed
subset as canonical JSON: the `ContentReport`, plus the structure Report's
`Results`, `TokenCounts`, `OtherTokenCounts` and `Errors`/`Warnings` tallies.
Absolute paths (`SkillDir`) are not emitted, so the goldens are machine
independent (every `File` field is already relative to the skill dir).

## Layout

- `corpus.txt` — repo-root-relative skill directories, one per line (`#`
  comments and blanks ignored). Read by BOTH the Go harness and the Rust test.
- `goldens.json` — index of records frozen from the Go reference.
- `goref/` — self-contained Go module that runs the two bridge calls.
- `fixtures/` — committed copies of the module's own `testdata/`/`examples/`
  plus `boundary/` fixtures built for the section-6 threshold inventory.
- `../golden_parity.rs` — the Rust integration test.

## Corpus (152 skills)

- 12 module `testdata`/`examples` fixtures (valid, invalid, broken-frontmatter,
  flat, rich, warnings-only, auxiliary-only, allowed-dirs, the three
  multi-skill children, review-skill) plus the multi-skill PARENT dir to
  exercise the SKILL.md-not-found path.
- 9 section-6 boundary fixtures under `fixtures/boundary/`: description at 1024
  bytes (valid) and 1025 bytes (error) with multibyte glyphs (byte-length, not
  rune-length); name at 64 bytes (valid) and 65 bytes (error); SKILL.md body at
  500 lines (no warn) and 501 lines (warn); an unclosed code fence; an internal
  `../` traversal link (Links warning); and a keyword-stuffed description.
- 130 real tekhne skills, the entire `skills/**/SKILL.md` tree across every
  populated domain (agentic-harness, ci-cd, development, documentation,
  infrastructure, observability, package-mgmt, project-mgmt, repository-mgmt,
  software-engineering, specialized, testing). Nothing was truncated.

Token-threshold boundaries above 5000 body tokens and the reference-file /
reference-total / other-total thresholds are not hand-tuned N/N+1 fixtures here:
token counts are already proven bit-exact by `token_parity.rs`, the threshold
comparisons are shared code, and the real corpus plus the line/byte boundary
fixtures exercise the surrounding branches. The one body-token warn branch that
no fixture pins deterministically is noted rather than papered over.

## Documented Go non-determinism (not a Rust defect)

Go's `structure.CheckStructure` walks `recognizedDirs` with `for range` over a
Go **map**, so the relative order of cross-directory "deep nesting" warnings is
randomised per run (observed on `skills/ci-cd/jenkinsfile/generator`). The
auditor only ever reads results order-independently (`hasStructureWarning`
any-match, a `descriptionLen` scan, error/warning tallies), so the Rust test
compares `Results[]` as a sorted multiset. Rust itself emits a deterministic
order (`["assets","references","scripts"]`); ordering is simply not part of the
compared surface.

The frontmatter YAML parse-error message differs by YAML library (yaml.v3 vs
serde_yaml_ng). The spec frees non-parsed wording and the auditor never parses
an Error-level Frontmatter message, so the test normalises that one message to
its prefix while still asserting its presence, Level, Category, File and Line.

## Regenerate

```bash
# from this directory; requires a Go toolchain >= 1.25.5 (auto via GOTOOLCHAIN)
grep -v '^#' corpus.txt | (cd goref && GOPROXY=off GOFLAGS=-mod=mod \
    go run . <repo-root>) > goldens.json
```

## Run the gate

```bash
cargo test -p skill-validator-rs --test golden_parity -- --nocapture
```
