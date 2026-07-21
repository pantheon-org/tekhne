# Skills monorepo with crate-based CLIs — migration wave plan

**Ticket / ref**: [.context/investigations/monorepo-tools-distribution-2026-07-13.md](../investigations/monorepo-tools-distribution-2026-07-13.md)
**Status**: In Progress. Committed through Wave 5 (A5); Waves 6-7 pending a go/no-go gate after A5. v4 folds in multi-agent review findings (21-07-2026) with open decisions D1-D4 resolved by interview; see "v4 review findings" and "Open decisions (v4)".
**Assignee**: thomas.roche
**Revision**: v4 (21-07-2026, adversarial review corrections applied; see "Revision history").

## v4 review findings (21-07-2026)

A four-lens adversarial review (technical feasibility, risk/execution realism, completeness, dependency/sequencing) ran against the live repo. Waves 1-5 (the committed Rust critical path) were confirmed sound and correctly sequenced. The defects cluster in the post-gate retirement waves (6-7) and are corrected inline below. Highest-impact items, with the count of independent reviewers who flagged each:

- **Python allowlist was 4, should be 6** (3 reviewers): `google-scholar-search` and `semantic-scholar-search` also ship `requirements.txt` + scripts and would be broken by C3. Added to the allowlist (D1).
- **C3 orphaned 4-5 live skills** (4 reviewers): `fluentbit` (validator + generator), `helm/validator`, `ansible/validator`, `k8s/debug`, and the jenkinsfile *generator* Python sat outside the original "7 validators", yet C3 would delete them with no owner. C2b/C3 scope reconciled; fate resolved in D2 and D5.
- **C2b undersized and mis-framed** (3 reviewers): real footprint is ~14.2k LOC, not 9.8k, and these are wrappers around external binaries (checkov, tflint, tfsec, terraform, promtool), not self-contained parsers. This ultimately led to D5: validators become plain skills, not Rust crates.
- **`pr-skill-checks.yml` was invisible** (1 reviewer, high impact): the largest workflow, running the two required merge gates, is now listed as a consumer and added to A6/B3 repoint scope.
- **CodeQL branch-protection coupling unowned** (2 reviewers): per PR #126, removing Go/Python from the CodeQL matrix needs a lockstep `main` ruleset change. New task A6c owns it.
- **`cli sync` / `cli uninstall` had no successor owner** (2 reviewers): B5 owner table completed.

Also corrected: validator sizing (S1 ~3k deterministic subset / 6.6k full module; C2b ~14.2k), the cargo-dist model (one generated workspace `release.yml`, not a hand-authored template), `jq` pinning, A6/B5 joint rollback, the NO-GO end-states, Astro paths (`docs/src/pages/`), the `@cliffy/command` reference, and the agent count (42).

## Open decisions (v4), resolved 21-07-2026 (interview)

- **D1, research allowlist membership. RESOLVED: add both.** `google-scholar-search` and `semantic-scholar-search` join the allowlist (six exempt research skills). Reflected in decision 2, CoS, and C1.
- **D2, fate of fluentbit / helm / ansible / k8s-debug / jenkinsfile-generator Python. RESOLVED: keep as plain skills that shell out.** Each keeps invoking its external binary (checkov, tflint, tfsec, promtool, and so on); no Rust port. Requirement: each such skill must **preflight-check that its expected external tool is present and tell the user which tool is missing** when it is not. Only genuinely dead Python is removed. **Refined by D5:** the external-linter wrappers among these become bash runners (Python removed), but the generators (jenkinsfile, fluentbit) have no external tool to defer to and instead keep allowlisted Python. Reflected in C2b/C3.
- **D3, `skill-validator-rs` distribution. RESOLVED: add A5b.** cargo-dist ships a standalone `skill-validator-rs` binary + mise pin, so B5's frontmatter hook has a real tool and B5 does not depend on A6.
- **D4, `tile.json` reality. RESOLVED: stale config, fix in R1.** Investigated 21-07-2026: no repo skill has a `tile.json` (the 3 on disk are external plugins under gitignored `.tessl/plugins/`). PR #93 migrated all 72 skills to `.tessl-plugin/plugin.json`, but `release-please-config.json` still points **all 61** `extra-files` entries at the removed `tile.json` (`$.version`). So release-please is **not** syncing the version into the published tessl `plugin.json` (e.g. terraform `plugin.json` = 1.0.1 is bumped only via the manifest, not the extra-file). R1 action: repoint all 61 `extra-files` from `tile.json` to `.tessl-plugin/plugin.json` (same `$.version` jsonpath, which exists) and reconcile any version drift, or drop the entries if version sync moves elsewhere.
- **D5, validators as Rust binaries (revises decision 1). RESOLVED: plain skills, not crates.** Validators wrap external linters or carry custom rule logic; neither fits the self-contained-binary model, and a Rust binary would still require the external tool installed. Two sub-categories, classified per tool in C2b: **(i) external-linter wrappers** (terraform, terragrunt, k8s/yaml-validator, helm, ansible, fluentbit-validator, k8s/debug) become bash-runner plain skills with a tool-presence check, Python removed; **(ii) custom-logic validators/generators** (promql best-practices, gitlab-ci, azure-pipelines, jenkinsfile-generator, fluentbit-generator) keep allowlisted Python, since their logic has no external tool to delegate to. Consequences: decision 1 reversed; C2b becomes a plain-skill conversion (moved to Wave 2, out of the Rust track); the allowlist gains the category-(ii) skills; no cargo-dist validator crates; Wave 6 shrinks.

## Goal

Evolve tekhne into a skills monorepo where plain skills stay as markdown under `./skills` (installed by ecosystem installers) and tool-backed skills are bundled into crate-based Rust CLIs distributed as compiled binaries. The committed near-term objective is a Rust `skill-auditor` that is proven and distributable (through Wave 5). Beyond that gate, retire the Go toolchain (auditor + standalone `agent-ecosystem/skill-validator`), remove the Python footprint except a documented allowlist, drop the JS `validate-schema` and the TypeScript/Bun `cli/`, and add `adr` and `journal` as tool-backed crates. Keep the Astro/Starlight site. Every wave boundary leaves the repo green: no wave removes a tool, pin, or resource that a live consumer still depends on.

## Conditions of Satisfaction

- [ ] Cargo workspace builds with `common`, `skill-install`, and `skill-validator-rs` shared crates.
- [ ] `skill-auditor` runs as a Rust binary behind the same CLI surface, installs on a clean macOS/Linux machine via `curl | sh`, and registers its bundled skill via an explicit `skill-auditor skill install` subcommand — the installer only drops the binary on PATH.
- [ ] **Go/no-go gate after Wave 5** recorded before any Wave 6 retirement or tool-addition begins.
- [ ] (Post-gate) Both Go tools and the Go pins in `mise.toml` removed; all consumers point at Rust replacements.
- [ ] (Post-gate) Plain skills discoverable by tessl, skills.sh, apm, Claude plugins via committed root manifests; no agent served by `cli/` loses an install path (gap documented); every `cli/` command (audit, tessl, install, uninstall, readme, sync, validate) has a named successor owner.
- [ ] (Post-gate) The TS `cli/` and JS `validate-schema` removed; every job has a named new owner.
- [ ] (Post-gate) Python removed **except an allowlist** covering the six research skills (D1) and the custom-logic validator/generator skills (D5), each marked in SKILL.md frontmatter and listed in a root allowlist; no other `.py`, inline `python3`, pip config, or python CI matrix remains outside the allowlist.
- [ ] The IaC/config validators are plain skills (D5): external-linter wrappers ship a bash runner with a tool-presence check; custom-logic validators/generators keep allowlisted Python. No validator Rust binaries.
- [ ] (Post-gate) `adr` and `journal` ship as tool-backed crate CLIs with bundled skills.
- [ ] release-please and cargo-dist coexist under the policy chosen in R1; no stale release-please entry points at a relocated/deleted skill.
- [ ] (Post-gate) Acceptance coverage equivalent to the retired `cli/features/*.feature` cucumber suite exists against the new owners.
- [ ] Astro site exposes distinct Skills and Tools sections.
- [ ] CI green on `main` after every wave merge; each must-land-together wave has a written rollback.
- [ ] (Post-gate) The `main` code-scanning ruleset and required status checks are updated in lockstep when Go/Python leave the CodeQL matrix and when `skill-validate.yml` is removed (per PR #126), so no PR is blocked by "configurations not found" and no required check goes unreported.

## Decisions locked (interview 14-07-2026)

1. **Validators are plain skills, not Rust crates** (revised 21-07-2026, see D5). They wrap external linters or carry custom rule logic, so the self-contained-binary model never fit. External-linter wrappers become bash-runner plain skills with a tool-presence check; custom rule engines and generators with no external delegate stay as allowlisted Python. No per-tool binaries, no N-installer cost.
2. **Some Python is kept but flagged**: an allowlist + SKILL.md frontmatter marker exempts it; CI python-removal and C3 skip the allowlist. Two categories: the six research skills (D1) and the custom-logic validator/generator skills (D5, e.g. promql, gitlab-ci, azure-pipelines, jenkinsfile, fluentbit). Zero-Python is a soft target with this documented carve-out.
3. **Windows/PowerShell is deferred** — build Unix-first (macOS/Linux); cargo-dist can add the Windows target later.
4. **Scope is milestone-gated through A5** — commit Waves 1-5; re-decide retirements and tool-additions at the gate.
5. **Skill registration is an explicit `skill install` subcommand** (no first-run self-registration).
6. **Versioning policy (release-please vs cargo-dist) is decided in the R1 spike**, not pre-committed.
7. **`journal` becomes a tool-backed crate** (A8), alongside `adr` (A7).

## Non-goals / explicitly out of scope

- **Windows/PowerShell** distribution (deferred, see decision 3).
- **MCP stdio-server exposure** of the tool binaries (trivial add-on per the investigation; a follow-on).
- Any change to plain-skill *content*.

## Key runtime consumers (grounding for must-land-together waves)

Confirmed by grep during review.

- **`skill-auditor`**: `hk.pkl`, `package.json`, `.github/workflows/{skill-audit,codeql}.yml`, `.github/dependabot.yml` (gomod), `README.md`, `AGENTS.md`, co-located SKILL.md.
- **standalone `skill-validator`** (second Go tool): `mise.toml` pin, `hk.pkl` pre-commit `skill-validate` + pre-push `skill-check`, `.github/workflows/skill-validate.yml`, `skill-audit.yml` (`go install`).
- **`pr-skill-checks.yml`** (largest workflow, ~12KB): runs the two required merge gates (Skill Validator + Skill Audit) plus Tessl Plugin Lint and label-gated Tessl Review. Consumes the standalone validator, the auditor, and tessl. Must be repointed in A6/B3.
- **inline `python3`**: `hk.pkl` (:97) + `skill-audit.yml` (:70-71) auditor JSON parse. Not a `.py` file.
- **`cli/index.ts` / `bun cli`** (7 commands: audit, tessl, install, uninstall, readme, sync, validate; built on `@cliffy/command`, not Commander): `hk.pkl` (readme-sync, cli-ts-conventions, skill-frontmatter, `validate frontmatter`, `bun test cli/`, cucumber step), `docs/src/pages/index.astro` (hardcoded `bun cli/index.ts install …` demo at :68), `README.md`, `CONTRIBUTING.md`, `package.json`, `AGENTS.md`, `.github/workflows/pr-cli-tests.yml`.
- **Python ecosystem surfaces**: six `skills/documentation/research/*/requirements.txt`, dependabot pip block, codeql python matrix, vendored Python skills in `skills-lock.json`/`tessl.json`.

## Dependency Graph

```text
=== COMMITTED PHASE ===
Wave 1 (parallel) — foundation, de-risk, decisions; no behaviour change
  A1 workspace scaffold + Rust CI + dist prototype ─┐
  S1 validator spike (Go-free; golden corpus spec)  ─┤
  R1 release-please x cargo-dist policy decision     ─┤
  B1 root manifests + agent-coverage check           ─┤─► Wave 2
  B3 tessl scripts (reconcile publish-skills.yml)    ─┤
  B4 drop validate-schema (no consumers)             ─┤
  C1 Python/glue inventory + research allowlist      ─┘

Wave 2 (parallel)
  A2 `common` crate                     (← A1)             ─┐
  B2a Astro Skills section + README-gen replacement        ─┤─► Wave 3
  C2 drop dead py + Bun glue + replace inline python3 (←C1) ─┤
  C2b validators → plain skills per D5  (← C1)             ─┘

Wave 3 (parallel)
  A3 `skill-install` crate   (← A2)      ─┐
  A4a `skill-validator-rs`   (← A2, S1)  ─┘─► Wave 4

Wave 4 (sequential)
  A4b Rust skill-auditor on A4a (← A4a) ─► Wave 5

Wave 5 (sequential)
  A5 auditor distribution + `skill install` subcommand + clean-machine verify (← A3, A4b)

===================  GO / NO-GO GATE (after A5)  ===================

=== POST-GATE PHASE (pending re-decision) ===
Wave 6 (parallel; 2 must-land-together workstreams)
  A6 retire Go auditor + standalone validator [must land] (← A5) ─┐
  A7 `adr` crate CLI          (← A3, A5)                         ─┤─► Wave 7
  A8 `journal` crate CLI      (← A3, A5)                         ─┤
  B5 drop TS cli/ + repoint + cucumber [must land]              ─┤
     (← B1, B2a, B3, B4, A5)                                    ─┘

Wave 7 (parallel)
  C3 remove Python (except allowlist) [must land] (← C2, C2b) ─┐
  B2b Astro Tools section  (← A5, A7, A8)                      ─┘
```

Four tracks: **A** = Rust tooling (critical path), **R** = release/versioning, **B** = plain-skill distribution + docs (gates the `cli/` drop), **C** = Python elimination. B5 (drop `cli/`) is gated on **A5, not A6** — the bulk-audit owner is the auditor batch CLI available at A5 — so `cli/` can drop even if Go retirement is deferred.

## Waves & Phases

### Wave 1 — Foundation, de-risk & decisions (parallel) [committed]

> Gate: None — start immediately. No behaviour change.

- [x] **A1** — Cargo workspace scaffold: root `Cargo.toml [workspace]`, empty `crates/{common,skill-install,skill-validator-rs}`, `dist-workspace.toml` stub, mise Rust pin, a cargo build+test CI workflow, and a prototype per-package tagged-release flow (`skill-auditor-vX.Y.Z`) to prove per-tool cargo-dist releases early — branch `feat/rust-workspace-scaffold` — model: standard — **DONE (commit `bb1cc86`): fmt/clippy/build/test green; the release workflow is a dormant prototype whose artifact output is verified at A5 (cargo-dist not yet installed).**
- [ ] **S1** — skill-validator spike: the validator is an external Go module (`agent-ecosystem/skill-validator@v1.5.6`, ~6.6k non-test LOC across ~14 packages). The deterministic gate subset the hooks need (`validate`/`check`/`structure`/`content`, excluding the `score`/`judge` LLM path) is ~3k non-test LOC. Scope A4a to that subset and explicitly exclude `score`/`judge`. Decide Go-free (reimplement, no wrapper). Produce a golden-corpus + score-tolerance spec for A4b. **Gates A4a/A4b.** — branch `spike/skill-validator-port` — model: smart — **DONE (commit `80492a87`): spec at `.context/plans/skill-validator-port-spec.md`; `o200k_base` token-parity go/no-go front-loaded as the A4a gate; external link-checking made opt-in (behaviour change for A6).**
- [ ] **R1** — release-please × cargo-dist policy: decision doc (tag namespaces, version source of truth per package type, how relocated tool-backed skills leave `release-please-config.json` + manifest). **Also fix the stale extra-files (D4): repoint all 61 `tile.json` extra-files to `.tessl-plugin/plugin.json` `$.version` (or remove them), and reconcile any version drift between the manifest and the published `plugin.json`.** Consumed by A5/A6/A7/A8 — branch `docs/release-versioning-policy` — model: standard
- [ ] **B1** — Root ecosystem manifests (tessl `.tessl-plugin/plugin.json` `skills: ./skills/`, root `skills.toml`, `.claude-plugin/marketplace.json`, apm equivalent); verify each installer discovers `./skills` and that the 42 agents in `cli/lib/types/agents.ts` retain an install path (document gaps). Coordinate the root-vs-per-skill publish-unit model with B3 (they disagree today: B1 introduces a root manifest, B3 rewrites the per-skill `publish-skills.yml`) — branch `feat/root-skill-manifests` — model: standard
- [ ] **B3** — tessl publish/manage as npm scripts + docs; reconcile with / replace `.github/workflows/publish-skills.yml` — branch `feat/tessl-scripts` — model: fast
- [ ] **B4** — Drop `tools/validate-schema` (no runtime consumers) — branch `chore/drop-validate-schema` — model: fast
- [ ] **C1** — Authoritative Python + glue inventory (all `.py` incl. fluentbit validator+generator, helm/ansible validators, `k8s/debug/pod_diagnostics.py` distinct from `k8s/yaml-validator`, and jenkinsfile `generator/` scripts; `requirements.txt` (eight: six under research/, two vendored under `.agents/skills/`); inline `python3`; vendored Python skills; dependabot pip; codeql python matrix; `scripts/` glue). **Create the research-skill allowlist and add the frontmatter exemption marker** to the six research skills per D1: PubMed, Sci-Hub, notebooklm, sci-data-extractor, google-scholar-search, semantic-scholar-search. Confirm `.agents/skills/**` vendored copies (pubmed-search, sci-hub-search) are carved out of the C3 verification. Source of truth for C2/C2b/C3 — branch `chore/python-inventory` — model: standard

Verification:
- [ ] `cargo build && cargo test` green (A1 ✓); per-package tag prototype produces a release artifact (deferred to A5).
- [ ] S1 decision doc: Go-free approach, LOC budget, golden-corpus/tolerance spec.
- [ ] R1 policy committed.
- [ ] Installers resolve `./skills`; agent-coverage gap documented.
- [ ] Allowlist + frontmatter markers in place; existing tools/CI still pass unchanged.

Rollback: all Wave 1 branches additive or delete only unused code; revert per branch.

### Wave 2 — Shared foundation & housekeeping (parallel) [committed]

> Gate: Wave 1 verified ✓

- [ ] **A2** — `common` crate: config loading, error types, structured output, fs helpers (+ tests) — branch `feat/common-crate` — model: standard
- [ ] **B2a** — Astro Skills section from `./skills`, plus a replacement for the root `README.md` generation done today by `cli/lib/readme` (~1.7k LOC) — branch `feat/docs-skills-and-readme` — model: standard
- [ ] **C2** — Drop dead Python; move thin glue to Bun per C1; replace the inline `python3` JSON parse in `hk.pkl` (:97) and `skill-audit.yml` (:70-71) with `jq`, and **add `jq` to `mise.toml`** (only `yq` is pinned today) so the audit gate stays behaviour-preserving on contributor machines. Do not touch allowlisted skills — branch `chore/python-glue-and-drop` — model: standard
- [ ] **C2b** — Convert validators to plain skills per D5. Classify each tool: **(i) external-linter wrappers** (terraform, terragrunt, k8s/yaml-validator, helm, ansible, fluentbit-validator, k8s/debug) get a bash runner that preflight-checks its external tool (checkov/tflint/tfsec/kubeconform/helm/ansible-lint/fluent-bit/kubectl) is on PATH and tells the user what is missing, then shells out; their Python wrapper is removed. **(ii) custom-logic validators/generators** (promql, gitlab-ci, azure-pipelines, jenkinsfile-generator, fluentbit-generator) keep their Python and are added to the allowlist + frontmatter marker. No Rust crates. Behaviour-preserving: same validation, new runner — branch `chore/validators-to-plain-skills` — model: standard

Verification:
- [ ] `cargo test -p common` green.
- [ ] Each validator skill runs; external-linter wrappers report a clear message when their tool is absent; category-(ii) skills are allowlisted + frontmatter-marked.
- [ ] `README.md` regenerates without `cli/`; format matches.
- [ ] No inline `python3` in hooks/CI; audit gate still parses grade.

Rollback: revert per branch; jq swap is behaviour-preserving.

### Wave 3 — skill-install & validator-rs (parallel) [committed]

> Gate: Wave 2 verified ✓ (and S1 for A4a)

- [ ] **A3** — `skill-install` crate: port agent-dir detection from `cli/lib/types/agents.ts` (42 agents, env overrides, xdg semantics) + copy/link a bundled skill into detected dirs; table-parity test vs the TS — branch `feat/skill-install-crate` — model: standard
- [ ] **A4a** — `skill-validator-rs` crate: reimplement the validator surface per S1 (structure metrics, content analysis, token counts); deterministic/offline. Also the future replacement for the standalone validator gate — branch `feat/skill-validator-rs` — model: smart

Verification:
- [ ] `cargo test -p skill-install` and `-p skill-validator-rs` green.
- [ ] `skill-install` parity test matches the TS table.
- [ ] `skill-validator-rs` reproduces the Go validator metrics within tolerance on the golden corpus.

Rollback: additive crates; nothing repointed.

### Wave 4 — Auditor rewrite (sequential) [committed]

> Gate: Wave 3 verified ✓

- [ ] **A4b** — Rust `skill-auditor` on `skill-validator-rs`: port the 9-dimension scorer, reporter, store (+ tests) behind the same CLI surface. Dep mapping: stdlib `regexp` → `regex`, `tiktoken-go` → `tiktoken-rs` (subsumes the BPE split regex; **no fancy-regex**), `yaml.v3` → `serde_yaml`, `cobra` → `clap`. **Leave the Go binary in place.** — branch `feat/rust-skill-auditor` — model: smart

Verification:
- [ ] Auditor **grades** match the Go auditor within the S1 tolerance across the golden corpus, with a reviewed boundary-case list (not string equality).
- [ ] Go tools still work — nothing repointed.

Rollback: additive; Go remains authoritative.

### Wave 5 — Distribution (sequential) [committed]

> Gate: Wave 4 verified ✓

- [ ] **A5** — Auditor distribution: run `dist init` to generate the workspace `release.yml` (macOS/Linux `curl | sh`; Windows deferred), bundle the skill into `crates/skill-auditor/skill/`, add the `skill-auditor skill install` subcommand using `skill-install`. Note the cargo-dist model: it generates and owns one workspace `release.yml` that is regenerated when crates are added (A7/A8), not a hand-authored reusable template. Update release-please config/manifest per R1. **Verify on a clean machine (no repo, no toolchain): install binary, then `skill install`.** — branch `feat/auditor-dist` — model: smart

Verification:
- [ ] Clean-machine install yields a working `skill-auditor`; `skill install` registers the skill into detected agent dirs.
- [ ] Generated workspace `release.yml` exists and the regenerate-on-new-crate flow is documented.

Rollback: additive (Go still present).

### GO / NO-GO GATE (after A5)

> Record a decision before any Wave 6 work. The auditor is now a proven, distributable Rust binary. Confirm: is the model worth propagating to retirements (A6/B5/C3) and new tools (A7/A8)? Note that "stop here" is not a clean state: after Wave 5 the repo carries a permanent dual stack (two validators, two auditors via A4a/A4b, two README generators via B2a) while Go, TS `cli/`, and Python all remain authoritative. A NO-GO must therefore choose one of: (a) accept and maintain that dual stack indefinitely, or (b) schedule a tear-down of Waves 2-5 (its own effort estimate required). Do not treat the gate as a free off-ramp.

### Wave 6 — Retire Go, add tools, drop the TS CLI (parallel) [post-gate]

> Gate: Wave 5 verified ✓ AND go/no-go = GO.

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| A6 | Retire both Go tools (must land together) | 2 | Pending | standard |
| A6c | Update `main` CodeQL ruleset + required checks (lands with A6, and again with C3) | 1 | Pending | standard |
| A5b | Ship standalone `skill-validator-rs` binary (per D3) | 1 | Pending | smart |
| A7 | Add `adr` crate CLI | 1 | Pending | standard |
| A8 | Add `journal` crate CLI | 1 | Pending | standard |
| B5 | Drop TS `cli/` + cucumber (must land together) | 1 | Pending | standard |

- [ ] **A6** — Retire the Go toolchain **[must land together]**: (a) repoint `hk.pkl` (incl. removing the now-dead `go-mod-tidy` step that globs `tools/skill-auditor/**/*.go`), `package.json`, `.github/workflows/{skill-audit,codeql,pr-skill-checks}.yml`, `dependabot.yml` (gomod), `README.md`, `AGENTS.md`, SKILL.md to the Rust auditor + `skill-validator-rs`, mise installing them for CI; in `codeql.yml` remove the `go` matrix entry, the `setup-go` step, the `go build` step, and the `tools/skill-auditor/go.sum` cache path; (b) delete `tools/skill-auditor/`, `bin/skill-auditor`, `.github/workflows/skill-validate.yml`, and the `go` + `skill-validator` pins from `mise.toml`. Requires A6c to land in lockstep. Merge atomically — branch `chore/retire-go` — model: standard
- [ ] **A6c** — Branch-protection / CodeQL ruleset update **[lands with A6, and again with C3]**: when `go` (A6) and later `python` (C3) leave the CodeQL matrix, update `main`'s `code_scanning` ruleset so PRs are not blocked by "configurations not found" (per PR #126); when `skill-validate.yml` is deleted (A6), re-map the required status checks so "Skill Validator" is not left permanently unreported. Also decide the Rust static-analysis story (CodeQL has no Rust support) — branch `chore/codeql-ruleset` — model: standard
- [ ] **A7** — `adr` crate CLI with bundled skill + installer via the A5 template — branch `feat/adr-crate` — model: standard
- [ ] **A8** — `journal` crate CLI with bundled skill + installer via the A5 template (promoted from plain skill per decision 7) — branch `feat/journal-crate` — model: standard
- [ ] **B5** — Drop TS `cli/` **[must land together]**: remove `cli/` + repoint `hk.pkl`, `package.json`, `cucumber.js`, `docs/src/pages/index.astro` (incl. the hardcoded install demo at :68), `README.md`, `CONTRIBUTING.md`, `AGENTS.md`; retire `pr-cli-tests.yml`. Owners for all 7 commands: install fan-out → ecosystem installers; **uninstall → ecosystem installers or explicit retire-with-rationale**; frontmatter validation → `skill-validator-rs` (needs a distribution path, see D3); README generation → B2a; bulk audit → auditor CLI; tessl → B3; **sync (sync opencode) → named owner TBD; do not delete `cli/` until it is assigned**. **Port/replace the `cli/features/*.feature` cucumber suite** (audit, install, readme, sync, tessl, validate) against new owners; note several owners are external installers, so those tests may become network-dependent. Merge atomically — branch `chore/drop-ts-cli` — model: standard
> C2b (validators → plain skills) moved to Wave 2 per D5; it no longer depends on the Rust track. The full rationale and per-tool classification live in D5 and the Wave 2 C2b task.

> A6 and B5 are each must-land-together; A7/A8 are independent. A6 and B5 both edit `hk.pkl`/`package.json`/`README.md`/`AGENTS.md`, so land A6 before B5. Because they share those files, rollback is a joint reverse-order procedure (revert B5 first, then A6): once B5 has landed, A6 cannot be reverted alone. Consider merging A6+B5 as one atomic super-change since they share the same blast radius.

Verification:
- [ ] Hooks/CI use only Rust tools; grep shows no Go `skill-auditor`/`skill-validator` or `bun cli`/`cli/index.ts` outside history.
- [ ] `curl | sh` installs `adr` and `journal` binaries + `skill install` registers their skills.
- [ ] Acceptance coverage for audit/install/readme/sync/tessl/validate passes (sync only once its owner is assigned).

Rollback: joint and reverse-order. Tag both the pre-A6 and pre-B5 commits. Revert **B5** first (restore `cli/` + `pr-cli-tests.yml`), then **A6** (restore Go source, `bin/`, `skill-validate.yml`, mise pins, and revert A6c's ruleset change). Once B5 has landed on the shared files, A6 cannot be reverted in isolation. Merging A6+B5 atomically removes this interlock.

### Wave 7 — Python removal & tools catalogue (parallel) [post-gate]

> Gate: Wave 6 verified ✓

- [ ] **C3** — Remove Python **[must land together]**: **narrow** (do not delete) the dependabot pip block and the codeql `python` matrix to the allowlist (six research skills per D1 plus the custom-logic validator/generator skills per D5), so retained Python keeps dependency + scan coverage; delete `requirements.txt` and `.py` only for the external-linter-wrapper validators once C2b has converted them to bash plain skills with a tooling-presence check. **Preserve the allowlisted skills** and **carve out `.agents/skills/**`** vendored copies from the verification. Requires A6c to land the CodeQL-matrix ruleset change. Verify: every remaining `.py` is allowlisted, no `python3` invocation outside the allowlist, and pip/CodeQL still cover the retained Python — branch `chore/remove-python` — model: standard
- [ ] **B2b** — Astro Tools section (under `docs/src/pages/`) from tool-crate metadata/README, listing the shipped CLIs (auditor, adr, journal) with the CLI-install story; validators appear in the Skills section, not here (D5) — branch `feat/docs-tools-section` — model: standard

Verification:
- [ ] Python check passes with the allowlist + D2 plain-skill exceptions; allowlisted skills still function; pip/CodeQL still track retained Python.
- [ ] Astro renders Skills + Tools sections; Tools lists every shipped CLI.

Rollback: **C3** — revert the must-land commit. **B2b** — docs-only revert.

## Branch Strategy

| Branch | Tracks | Base | Phase |
|--------|--------|------|-------|
| `feat/rust-workspace-scaffold` | A1 | `main` | committed |
| `spike/skill-validator-port` | S1 | `main` | committed |
| `docs/release-versioning-policy` | R1 | `main` | committed |
| `feat/root-skill-manifests` | B1 | `main` | committed |
| `feat/tessl-scripts` | B3 | `main` | committed |
| `chore/drop-validate-schema` | B4 | `main` | committed |
| `chore/python-inventory` | C1 | `main` | committed |
| `feat/common-crate` | A2 | after A1 | committed |
| `feat/docs-skills-and-readme` | B2a | `main` | committed |
| `chore/python-glue-and-drop` | C2 | after C1 | committed |
| `chore/validators-to-plain-skills` | C2b | after C1 | committed |
| `feat/skill-install-crate` | A3 | after A2 | committed |
| `feat/skill-validator-rs` | A4a | after A2 | committed |
| `feat/rust-skill-auditor` | A4b | after A4a | committed |
| `feat/auditor-dist` | A5 | after A3, A4b | committed |
| `feat/skill-validator-rs-dist` | A5b | after A4a (per D3) | post-gate |
| `chore/retire-go` | A6 | after A5 | post-gate |
| `chore/codeql-ruleset` | A6c | with A6/C3 | post-gate |
| `feat/adr-crate` | A7 | after A5 | post-gate |
| `feat/journal-crate` | A8 | after A5 | post-gate |
| `chore/drop-ts-cli` | B5 | after A5, B2a | post-gate |
| `chore/remove-python` | C3 | after C2, C2b | post-gate |
| `feat/docs-tools-section` | B2b | after A5, A7, A8 | post-gate |

Prefer stacked PRs within a track (`pr-stacker`). B and C tracks can start against `main` immediately. Multi-milestone effort; do not attach dates past the A5 gate until go/no-go.

## Definition of Done

- [ ] All wave verification gates pass; go/no-go recorded after A5.
- [ ] All branches merged to `main`; CI green on `main`.
- [ ] No Go, no TS `cli/`, and no Python except the documented allowlist (six research skills plus the D5 validator/generator skills).
- [ ] `skill-auditor`, `adr`, and `journal` install standalone (macOS/Linux) and register their skills via `skill install`; validators ship as plain skills (D5).
- [ ] release-please and cargo-dist coexist under the R1 policy with no stale entries.
- [ ] Acceptance coverage equivalent to the old cucumber suite is green.
- [ ] `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, and the Astro site describe the new install/audit/publish story.

## Revision history

- **v4 (21-07-2026)** — Adversarial four-lens review corrections. Research allowlist widened to six (D1); C2b/C3 scope reconciled with the full Python-backed inventory incl. fluentbit/helm/ansible/k8s-debug/jenkinsfile-generator (D2); `skill-validator-rs` distribution gap for B5 flagged with A5b (D3); `tile.json` existence to verify (D4); `pr-skill-checks.yml` added as a consumer and to A6/B3; new A6c owns the CodeQL/branch-protection ruleset change (PR #126); cargo-dist reframed as a generated workspace `release.yml`; validator sizing corrected (S1 ~3k/6.6k, C2b ~14.2k); `cli` sync/uninstall owners added to B5; `jq` pinned in C2; A6/B5 rollback made a joint reverse-order procedure; NO-GO end-states made concrete; Astro paths corrected to `docs/src/pages/`; `@cliffy/command` corrected (not Commander); agent count 42. Open decisions D1-D4 resolved via interview 21-07-2026 (allowlist of six; five extra tools kept as shell-out plain skills with a tooling-presence check; A5b standalone validator binary; and the confirmed-stale `tile.json` extra-files repointed to `.tessl-plugin/plugin.json` in R1). **D5 (21-07-2026) reverses decision 1**: all validators become plain skills (external-linter wrappers as bash runners with a tool-presence check; custom-logic validators/generators as allowlisted Python), C2b becomes a plain-skill conversion moved to Wave 2, and the validator cargo-dist crates are dropped.
- **v3 (14-07-2026)** — Interview decisions applied: validators as 7 per-tool binaries; Python research skills kept via allowlist + frontmatter marker (zero-Python softened); Windows deferred (Unix-first); scope milestone-gated with an explicit go/no-go after A5; skill registration via explicit `skill install` subcommand; versioning left to the R1 spike; `journal` promoted to a tool-backed crate (A8). Re-sequenced into a committed phase (Waves 1-5) and a post-gate phase (Waves 6-7).
- **v2 (14-07-2026)** — Multi-agent review corrections: added the standalone `skill-validator` retirement, release-please×cargo-dist coexistence (R1), split A4 into A4a/A4b with a full LOC budget, corrected the Python-track premise and the `regexp2→fancy-regex` phantom, fixed the installer-registration assumption, made the golden test tolerance-based, decoupled B5 from A6, added CI/test/rollback tasks.
- **v1 (13-07-2026)** — Initial six-wave plan from the investigation.
