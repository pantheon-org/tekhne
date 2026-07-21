# S1 spike: skill-validator Go-free port spec

**Status**: Complete (21-07-2026). Feeds A4a (build `skill-validator-rs`) and A4b (auditor parity).
**Task**: S1 (Wave 1) of the monorepo tools distribution migration.
**Source module**: `github.com/agent-ecosystem/skill-validator@v1.5.6` (pinned in `tools/skill-auditor/go.mod`), 6,617 non-test Go LOC.
**Gates**: A4a and A4b cannot start until the token-parity go/no-go (section 7) clears.

## 1. Decision

Reimplement the validator in Rust as `skill-validator-rs`. No Go wrapper, no FFI, no shelling out to the Go binary. The port covers the offline, deterministic validation surface only. The LLM scoring path (`judge`, `evaluate`, score reporting) is out of scope for this crate and is not reimplemented.

Rationale: the consumed surface is compact (~2,900 non-test LOC), pure, and offline. The only genuine risk is exact tokenizer parity (section 7), which a wrapper would not remove.

## 2. Scope

### 2.1 Consumed by the auditor (A4b parity target)

The auditor couples to the validator through one file, `tools/skill-auditor/scorer/validator_bridge.go`, via exactly two calls:

1. `orchestrate.RunContentAnalysis(skillDir)`, keeping `.ContentReport`.
2. `structure.Validate(skillDir, {SkipOrphans: true, AllowFlatLayouts: true, AllowExtraFrontmatter: true})`.

The bridge reads only: `Structure.TokenCounts[]` (matched on `File == "SKILL.md"` or prefix `"SKILL.md "`), `Structure.Results[]` filtered by `Level`/`Category`/`Message`, and `Content.*`.

**Load-bearing message strings** (the auditor parses these, so the Rust port must emit them verbatim):
- `description: (%d chars)` (parsed with `Sscanf` for description length).
- Any `Category == "Links" && Level == Warning` (internal-link warning detection).
- Arbitrary substrings looked up via `hasStructureWarning(substr)`.

Non-parsed message wording may differ, but `Errors`/`Warnings` tallies and exit codes must match exactly.

### 2.2 Hook consumers (gate parity)

From `hk.pkl`:
- pre-commit `skill-validate`: `validate structure --allow-dirs=evals <dir>`. Note `SkipOrphans` defaults **false** here, so orphan checks run (unlike the auditor call).
- pre-push `skill-check`: `check --allow-dirs=evals --per-file <dir>` runs all groups: structure + links + content + contamination.

### 2.3 In-scope packages (offline-deterministic core, ~2,939 LOC)

`structure` (1,497), `contamination` (331), `orchestrate` (203), `content` (180), `skill` (167), `skillcheck` (132), `types` (235, minus scoring types), `util` (104), `links/extract.go` (90 of the 297-LOC `links` pkg).

Plus ~700 LOC of CLI (`clap`) and report rendering (text/json/markdown/annotations) for hook parity. This is not needed for auditor parity, only for the standalone gate.

### 2.4 Explicitly excluded (LLM / scoring path, ~2,376 LOC)

`judge/` (1,069), `evaluate/` (307), `report/eval*.go` (515), `cmd/score*` (291), and the `DimensionScore`/`Scored` members of `types`. Do not port these in A4a.

## 3. Determinism decision (network caveat)

The in-scope surface is offline and deterministic **except** the `links` group's external URL validation (`links/check.go` + SSRF-guarded transport `links/safenet.go`), which makes live HTTP HEAD/GET requests. Internal relative-link checking (`structure/links.go` `CheckInternalLinks`) is offline and stays in scope.

Today's pre-push `skill-check` hits the network through the `links` group, which introduces non-determinism (remote 4xx/5xx, timeouts, 403 handling) into a git hook. Decision for the port: **external link checking is opt-in / network-gated, not part of the default deterministic gate.** This is a deliberate behaviour change from the Go tool and must be recorded for the A4a owner and reflected when A6 repoints the hooks.

## 4. Metric inventory (parity surface)

Full per-check detail (thresholds, regexes, algorithms) was captured in the S1 analysis. The load-bearing specifics the port must reproduce:

**Tokenizer**: `o200k_base` (OpenAI GPT-4o BPE). Every count is the token-id count of the text, not characters. Drives all structural thresholds below.

**Structure thresholds** (all must match exactly): SKILL.md body warn >5000 tokens; body warn >500 lines (`count("\n")+1`); reference file warn >10000 / error >25000 tokens; reference total warn >25000 / error >50000; other total warn >25000 / error >100000; skill-ratio error when `otherTotal > 25000 && standardTotal > 0 && otherTotal > standardTotal*10`. Name: `^[a-z0-9]+(-[a-z0-9]+)*$`, byte length <=64, must equal dir basename. Description: byte length <=1024, emits `description: (N chars)`. Compatibility <=500 bytes.

**Content metrics** (`content.Analyze`, floats rounded to 4dp): WordCount, CodeBlockCount, CodeBlockRatio, CodeLanguages (document order preserved), SentenceCount, ImperativeCount (48-verb set), ImperativeRatio, InformationDensity (`= imperativeRatio` with no code, else `0.5*codeBlockRatio + 0.5*imperativeRatio`), StrongMarkers (10 regexes), WeakMarkers (10 regexes), InstructionSpecificity (`strong/(strong+weak)`), SectionCount, ListItemCount.

**Contamination** (`contamination.Analyze`): ScopeBreadth, LanguageMismatch, PrimaryCategory (tie-break by first appearance order in codeLanguages), category/tool/tech string lists, ContaminationScore = `min(1.0, f1+f2+f3)` (f1 0.3 multi-interface-tool; f2 `0.4*min(weightedMismatch/3,1)`, weights app-app 1.0 / app-aux 0.25 / aux-aux 0.1; f3 `0.3*min((breadth-2)/4,1)` when breadth>2), ContaminationLevel (>=0.5 high, >=0.2 medium, else low).

`Result = {Level(Pass/Info/Warning/Error), Category, Message, File, Line}`. Exit codes (`cmd/exitcode.go`): 0 clean, 1 error, 2 warning, 3 CLI usage; `--strict` promotes warnings to exit 1.

## 5. Go to Rust mapping

| Go | Rust | Risk |
|----|------|------|
| `tiktoken-go/tokenizer` `O200kBase` | `tiktoken-rs` `o200k_base()` | **#1 risk. Must be bit-exact.** See section 7. |
| `dlclark/regexp2` (indirect) | subsumed by `fancy-regex` inside `tiktoken-rs` | Only the tokenizer split regex needs lookahead; the validator's own regexes never do. |
| stdlib `regexp` (RE2) | `regex` crate | 1:1, both RE2-style. **Compile with `(?-u)`**: Go `\w`/`\b` are ASCII, Rust `regex` defaults to Unicode. |
| `gopkg.in/yaml.v3` | `serde_yaml_ng` (base `serde_yaml` archived) | `allowed-tools` string-or-sequence via `#[serde(untagged)]` + `WasList`; raw + typed metadata double-parse; verify duplicate-key / anchor parity. |
| `spf13/cobra` | `clap` (derive) | Subcommands `validate structure`, `check`, `analyze content`; flags `--allow-dirs`, `--per-file`, `--skip-orphans`, `--strict`, `--allow-flat-layouts`, `--allow-extra-frontmatter`, `-o/--output`, `--emit-annotations`. |
| `net/http` + `safeTransport` | `reqwest`/`hyper` + DNS-resolve SSRF guard | Only if links group ported; non-deterministic by nature (see section 3). |
| `math.Round` (`util.RoundTo`) | `f64::round` | Same half-away-from-zero; low risk at 4dp. |
| `len(string)` byte lengths | `str::len()` (bytes, NOT `chars().count()`) | Matters for multibyte name/description/compatibility limits. |
| Go random map iteration | deterministic sort/`break` | The two spots it could leak are already made deterministic; replicate ordering exactly. |

## 6. Golden-corpus and tolerance spec (A4b acceptance)

**Corpus.** (a) The module's own `testdata/` as the ready-made edge-case set: `valid-skill`, `invalid-skill`, `broken-frontmatter`, `flat-skill`, `multi-skill/{alpha,beta,gamma}`, `rich-skill`, `warnings-only-skill`, `auxiliary-only-skill`, `allowed-dirs-skill`. (b) A stratified sample of tekhne's real `skills/` across the 13 domains. Freeze goldens by running the pinned Go binary (`validate structure` and `check`, JSON output) over the corpus.

**Diversity axes to guarantee**: flat vs nested; presence of references/scripts/assets; multi-skill parent; `--allow-dirs`; frontmatter valid/missing-name/missing-desc/name-not-dir/over-length/keyword-stuffed (both heuristics)/allowed-tools string-vs-list/extra-fields/metadata-map; token sizes crossing each threshold; prose-only vs code-heavy; single-language vs app-app vs app-aux contamination vs multi-interface-tool vs tech-reference prose vs breadth>2; unclosed fences (backtick, tilde, indented); internal links valid/broken/fragment/traversal/template.

**Match EXACTLY (zero tolerance):**
- Every per-file `TokenCount` (int). Validate this first, in isolation (section 7).
- Full `Results[]`: `Level`, `Category`, `File`, `Line`, plus `Message` for the two auditor-parsed strings and any `hasStructureWarning` lookups. `Errors`/`Warnings` tallies and exit code must match.
- `ContentReport` integer fields and `CodeLanguages` (order-sensitive).
- `ContaminationReport`: `ScopeBreadth`, `LanguageMismatch`, `PrimaryCategory`, `ContaminationLevel`, and the string lists.

**Match within tolerance (derived floats, 4dp): absolute <= 5e-5.** Applies to CodeBlockRatio, ImperativeRatio, InformationDensity, InstructionSpecificity, ContaminationScore, MismatchWeights. Rationale: identical integer inputs plus round-to-4 should be exactly equal; the band only absorbs FP noise and can never flip a contamination boundary (0.2 / 0.5) or the density branch.

**Boundary fixtures to build (N and N+1):** every token threshold (5000 body, 500 lines, 10000/25000 ref-file, 25000/50000 ref-total, 25000/100000 other-total); skill-ratio at `otherTotal == 25000` and `== standardTotal*10` vs `>`; name exactly 64 bytes and description exactly 1024 bytes with multibyte content; keyword-stuffing exact triggers (5 quotes, 8 comma segments, 60% short, avg-3-words boundary, abbreviation list, digit-period, capital-letter boundary); fence detection (closing longer/shorter than opening, tilde vs backtick, 3-space indent); bare-URL trailing-delimiter and entity-suffix trimming; `\b` ASCII-vs-Unicode marker counting adjacent to non-ASCII.

## 7. Token-parity go/no-go (front-loaded sub-gate)

Before committing to the full A4a port, build a token-parity harness that compares `tiktoken-rs` `o200k_base()` against `tiktoken-go/tokenizer` `O200kBase` across the entire corpus (and a stress set of code, Unicode, and edge strings). Every per-file token count must be bit-exact. This is the single gating unknown: every structural threshold and exit code hangs off it.

- **Go**: proceed with the full port.
- **No-go** (counts diverge and cannot be reconciled): escalate before A4a. Options to weigh then: pin a different tokenizer crate, vendor the exact BPE ranks, or reconsider the deterministic-count approach. Do not start A4b until this clears.

## 8. LOC budget and effort

| Segment | LOC | In A4a scope |
|---------|-----|--------------|
| Offline-deterministic core | ~2,939 | Yes |
| CLI + report rendering (gate) | ~700 | Hook parity only, not auditor parity |
| Network link group | 207 | Opt-in only (section 3) |
| LLM / scoring path | ~2,376 | No (excluded) |

Rust line ratio ~1 to 1.3x, so roughly 3,000 to 3,800 Rust LOC including CLI and JSON rendering. The algorithms are compact and pure; the cost and risk are not LOC but tokenizer parity. Rough effort: 1 to 2 days for the token-parity sub-gate, then ~2 to 3 weeks for one engineer for the full port plus the golden-corpus harness. Secondary risks: the `\b`/`\w` ASCII-vs-Unicode regex nuance and YAML edge cases.

## 9. Handoff to A4a / A4b

- A4a builds `skill-validator-rs` to this scope, front-loading the section 7 token-parity harness as its first deliverable and gate.
- A4b validates auditor grade parity against the golden corpus using the section 6 tolerances (not string equality), with the section 6 boundary list reviewed explicitly.
- A5b ships `skill-validator-rs` as a standalone binary (D3) so the pre-commit/pre-push hooks can consume it when A6 repoints them.
