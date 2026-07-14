# Skills monorepo with crate-based CLIs — migration wave plan

**Ticket / ref**: [.context/investigations/monorepo-tools-distribution-2026-07-13.md](../investigations/monorepo-tools-distribution-2026-07-13.md)
**Status**: In Progress — **committed through Wave 5 (A5); Waves 6-7 pending a go/no-go gate after A5.**
**Assignee**: thomas.roche
**Revision**: v3 (14-07-2026 — interview decisions applied; see "Decisions locked" and "Revision history").

## Goal

Evolve tekhne into a skills monorepo where plain skills stay as markdown under `./skills` (installed by ecosystem installers) and tool-backed skills are bundled into crate-based Rust CLIs distributed as compiled binaries. The committed near-term objective is a Rust `skill-auditor` that is proven and distributable (through Wave 5). Beyond that gate, retire the Go toolchain (auditor + standalone `agent-ecosystem/skill-validator`), remove the Python footprint except a documented allowlist, drop the JS `validate-schema` and the TypeScript/Bun `cli/`, and add `adr` and `journal` as tool-backed crates. Keep the Astro/Starlight site. Every wave boundary leaves the repo green: no wave removes a tool, pin, or resource that a live consumer still depends on.

## Conditions of Satisfaction

- [ ] Cargo workspace builds with `common`, `skill-install`, and `skill-validator-rs` shared crates.
- [ ] `skill-auditor` runs as a Rust binary behind the same CLI surface, installs on a clean macOS/Linux machine via `curl | sh`, and registers its bundled skill via an explicit `skill-auditor skill install` subcommand — the installer only drops the binary on PATH.
- [ ] **Go/no-go gate after Wave 5** recorded before any Wave 6 retirement or tool-addition begins.
- [ ] (Post-gate) Both Go tools and the Go pins in `mise.toml` removed; all consumers point at Rust replacements.
- [ ] (Post-gate) Plain skills discoverable by tessl, skills.sh, apm, Claude plugins via committed root manifests; no agent served by `cli/` loses an install path (gap documented).
- [ ] (Post-gate) The TS `cli/` and JS `validate-schema` removed; every job has a named new owner.
- [ ] (Post-gate) Python removed **except an allowlist** of flagged research skills (PubMed, Sci-Hub, notebooklm, sci-data-extractor), each marked in SKILL.md frontmatter and listed in a root allowlist; no other `.py`, inline `python3`, pip config, or python CI matrix remains.
- [ ] (Post-gate) The 7 IaC/config validators ship as **7 independent per-tool Rust binaries**, released via a shared cargo-dist workflow template.
- [ ] (Post-gate) `adr` and `journal` ship as tool-backed crate CLIs with bundled skills.
- [ ] release-please and cargo-dist coexist under the policy chosen in R1; no stale release-please entry points at a relocated/deleted skill.
- [ ] (Post-gate) Acceptance coverage equivalent to the retired `cli/features/*.feature` cucumber suite exists against the new owners.
- [ ] Astro site exposes distinct Skills and Tools sections.
- [ ] CI green on `main` after every wave merge; each must-land-together wave has a written rollback.

## Decisions locked (interview 14-07-2026)

1. **Validators ship as one binary per validator** (7 independent CLIs), accepting the N-installer cost, mitigated by a shared cargo-dist release template (A5).
2. **Python research skills are kept but flagged**: an allowlist + SKILL.md frontmatter marker exempts them; CI python-removal and C3 skip the allowlist. Zero-Python is a soft target with this documented carve-out.
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
- **inline `python3`**: `hk.pkl` + `skill-audit.yml` (auditor JSON parse). Not a `.py` file.
- **`cli/index.ts` / `bun cli`**: `hk.pkl` (readme-sync, cli-ts-conventions, skill-frontmatter, `validate frontmatter`, `bun test cli/`, cucumber step), `docs/index.astro`, `README.md`, `CONTRIBUTING.md`, `package.json`, `AGENTS.md`, `.github/workflows/pr-cli-tests.yml`, commanderjs eval.
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
  A2 `common` crate                     (← A1)            ─┐
  B2a Astro Skills section + README-gen replacement       ─┤─► Wave 3
  C2 drop dead py + Bun glue + replace inline python3(←C1)─┘

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
  A7 `adr` crate CLI          (← A3, A5)                         ─┤
  A8 `journal` crate CLI      (← A3, A5)                         ─┤
  B5 drop TS cli/ + repoint + cucumber [must land]              ─┤─► Wave 7
     (← B1, B2a, B3, B4, A5)                                    ─┤
  C2b port 7 validators to per-tool binaries (← C1, A3)         ─┘

Wave 7 (parallel)
  C3 remove Python (except allowlist) [must land] (← C2, C2b) ─┐
  B2b Astro Tools section  (← A5, A7, A8)                      ─┘
```

Four tracks: **A** = Rust tooling (critical path), **R** = release/versioning, **B** = plain-skill distribution + docs (gates the `cli/` drop), **C** = Python elimination. B5 (drop `cli/`) is gated on **A5, not A6** — the bulk-audit owner is the auditor batch CLI available at A5 — so `cli/` can drop even if Go retirement is deferred.

## Waves & Phases

### Wave 1 — Foundation, de-risk & decisions (parallel) [committed]

> Gate: None — start immediately. No behaviour change.

- [ ] **A1** — Cargo workspace scaffold: root `Cargo.toml [workspace]`, empty `crates/{common,skill-install,skill-validator-rs}`, `dist-workspace.toml` stub, mise Rust pin, a cargo build+test CI workflow, and a prototype per-package tagged-release flow (`skill-auditor-vX.Y.Z`) to prove per-tool cargo-dist releases early — branch `feat/rust-workspace-scaffold` — model: standard
- [ ] **S1** — skill-validator spike: scope the full external validator surface (~2.5k LOC) plus the standalone validator's checks; decide Go-free (reimplement or Rust equivalent — no wrapper). Produce a golden-corpus + score-tolerance spec for A4b. **Gates A4a/A4b.** — branch `spike/skill-validator-port` — model: smart
- [ ] **R1** — release-please × cargo-dist policy: decision doc (tag namespaces, version source of truth per package type, how relocated tool-backed skills leave `release-please-config.json` + manifest). Consumed by A5/A6/A7/A8 — branch `docs/release-versioning-policy` — model: standard
- [ ] **B1** — Root ecosystem manifests (tessl `.tessl-plugin/plugin.json` `skills: ./skills/`, root `skills.toml`, `.claude-plugin/marketplace.json`, apm equivalent); verify each installer discovers `./skills` and that the ~43 agents in `cli/lib/types/agents.ts` retain an install path (document gaps) — branch `feat/root-skill-manifests` — model: standard
- [ ] **B3** — tessl publish/manage as npm scripts + docs; reconcile with / replace `.github/workflows/publish-skills.yml` — branch `feat/tessl-scripts` — model: fast
- [ ] **B4** — Drop `tools/validate-schema` (no runtime consumers) — branch `chore/drop-validate-schema` — model: fast
- [ ] **C1** — Authoritative Python + glue inventory (all `.py` incl. fluentbit/helm/ansible validators, noting k8s under `yaml-validator/` and jenkinsfile under `generator/`; `requirements.txt`; inline `python3`; vendored Python skills; dependabot pip; codeql python matrix; `scripts/` glue). **Create the research-skill allowlist and add the frontmatter exemption marker** to PubMed, Sci-Hub, notebooklm, sci-data-extractor. Source of truth for C2/C2b/C3 — branch `chore/python-inventory` — model: standard

Verification:
- [ ] `cargo build && cargo test` green; per-package tag prototype produces a release artifact.
- [ ] S1 decision doc: Go-free approach, LOC budget, golden-corpus/tolerance spec.
- [ ] R1 policy committed.
- [ ] Installers resolve `./skills`; agent-coverage gap documented.
- [ ] Allowlist + frontmatter markers in place; existing tools/CI still pass unchanged.

Rollback: all Wave 1 branches additive or delete only unused code; revert per branch.

### Wave 2 — Shared foundation & housekeeping (parallel) [committed]

> Gate: Wave 1 verified ✓

- [ ] **A2** — `common` crate: config loading, error types, structured output, fs helpers (+ tests) — branch `feat/common-crate` — model: standard
- [ ] **B2a** — Astro Skills section from `./skills`, plus a replacement for the root `README.md` generation done today by `cli/lib/readme` (~1.7k LOC) — branch `feat/docs-skills-and-readme` — model: standard
- [ ] **C2** — Drop dead Python; move thin glue to Bun per C1; replace the inline `python3` JSON parse in `hk.pkl` and `skill-audit.yml` with `jq`. Do not touch allowlisted skills — branch `chore/python-glue-and-drop` — model: standard

Verification:
- [ ] `cargo test -p common` green.
- [ ] `README.md` regenerates without `cli/`; format matches.
- [ ] No inline `python3` in hooks/CI; audit gate still parses grade.

Rollback: revert per branch; jq swap is behaviour-preserving.

### Wave 3 — skill-install & validator-rs (parallel) [committed]

> Gate: Wave 2 verified ✓ (and S1 for A4a)

- [ ] **A3** — `skill-install` crate: port agent-dir detection from `cli/lib/types/agents.ts` (~43 agents, env overrides, xdg semantics) + copy/link a bundled skill into detected dirs; table-parity test vs the TS — branch `feat/skill-install-crate` — model: standard
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

- [ ] **A5** — Auditor distribution: cargo-dist config (macOS/Linux `curl | sh`; Windows deferred), bundle the skill into `crates/skill-auditor/skill/`, add the `skill-auditor skill install` subcommand using `skill-install`. **Extract a reusable release-workflow template** for A7/A8/C2b. Update release-please config/manifest per R1. **Verify on a clean machine (no repo, no toolchain): install binary, then `skill install`.** — branch `feat/auditor-dist` — model: smart

Verification:
- [ ] Clean-machine install yields a working `skill-auditor`; `skill install` registers the skill into detected agent dirs.
- [ ] Reusable release template exists and is documented.

Rollback: additive (Go still present).

### GO / NO-GO GATE (after A5)

> Record a decision before any Wave 6 work. The auditor is now a proven, distributable Rust binary. Confirm: is the model worth propagating to retirements (A6/B5/C3) and new tools (A7/A8)? If no, stop here with the Rust auditor shipping alongside the existing stack.

### Wave 6 — Retire Go, add tools, drop the TS CLI (parallel) [post-gate]

> Gate: Wave 5 verified ✓ AND go/no-go = GO.

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| A6 | Retire both Go tools (must land together) | 2 | Pending | standard |
| A7 | Add `adr` crate CLI | 1 | Pending | standard |
| A8 | Add `journal` crate CLI | 1 | Pending | standard |
| B5 | Drop TS `cli/` + cucumber (must land together) | 1 | Pending | standard |
| C2b | Port 7 validators to per-tool binaries | 7 | Pending | smart |

- [ ] **A6** — Retire the Go toolchain **[must land together]**: (a) repoint `hk.pkl`, `package.json`, `.github/workflows/{skill-audit,codeql}.yml`, `dependabot.yml` (gomod), `README.md`, `AGENTS.md`, SKILL.md to the Rust auditor + `skill-validator-rs`, mise installing them for CI; (b) delete `tools/skill-auditor/`, `bin/skill-auditor`, `.github/workflows/skill-validate.yml`, and the `go` + `skill-validator` pins from `mise.toml`. Merge atomically — branch `chore/retire-go` — model: standard
- [ ] **A7** — `adr` crate CLI with bundled skill + installer via the A5 template — branch `feat/adr-crate` — model: standard
- [ ] **A8** — `journal` crate CLI with bundled skill + installer via the A5 template (promoted from plain skill per decision 7) — branch `feat/journal-crate` — model: standard
- [ ] **B5** — Drop TS `cli/` **[must land together]**: remove `cli/` + repoint `hk.pkl`, `package.json`, `cucumber.js`, `docs/index.astro`, `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, commanderjs eval; retire `pr-cli-tests.yml`. Owners: install fan-out → ecosystem installers; frontmatter validation → `skill-validator-rs`; README generation → B2a; bulk audit → auditor CLI; tessl → B3. **Port/replace the `cli/features/*.feature` cucumber suite** against new owners. Merge atomically — branch `chore/drop-ts-cli` — model: standard
- [ ] **C2b** — Port the 7 IaC/config validators (terraform, terragrunt, k8s, promql, gitlab-ci, azure-pipelines, jenkinsfile) to **per-tool Rust binaries** per C1; each a separate crate + installer from the A5 template. ~9.8k LOC across 7 parser domains — one branch per validator; bump gitlab-ci/jenkinsfile/azure-pipelines to `smart` — branch `feat/rust-validator-<name>` (×7) — model: smart

> A6 and B5 are each must-land-together; A7/A8/C2b are independent. A6, B5 both edit `hk.pkl`/`package.json`/`README.md`/`AGENTS.md` — sequence A6 before B5 to avoid conflicts.

Verification:
- [ ] Hooks/CI use only Rust tools; grep shows no Go `skill-auditor`/`skill-validator` or `bun cli`/`cli/index.ts` outside history.
- [ ] `curl | sh` installs `adr` and `journal` binaries + `skill install` registers their skills.
- [ ] Each validator binary installs and passes its eval scenarios.
- [ ] Acceptance coverage for install/validate/readme/tessl/audit passes.

Rollback: **A6** — revert the must-land commit to restore Go source, `bin/`, `skill-validate.yml`, mise pins. **B5** — revert to restore `cli/` + `pr-cli-tests.yml`. Tag the pre-A6 commit for one-command backout.

### Wave 7 — Python removal & tools catalogue (parallel) [post-gate]

> Gate: Wave 6 verified ✓

- [ ] **C3** — Remove Python **[must land together]**: delete the six `requirements.txt`, the dependabot pip block, the codeql python matrix, and any non-allowlisted `.py`, in the same change. **Preserve the allowlisted research skills** (they keep their Python + frontmatter marker). Verify: zero non-allowlisted `.py`, zero `python3` invocation, and only the allowlist retains Python — branch `chore/remove-python` — model: standard
- [ ] **B2b** — Astro Tools section from tool-crate metadata/README, listing all shipped CLIs (auditor, adr, journal, 7 validators) with the CLI-install story — branch `feat/docs-tools-section` — model: standard

Verification:
- [ ] Python check passes with the allowlist exception; allowlisted skills still function.
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
| `feat/skill-install-crate` | A3 | after A2 | committed |
| `feat/skill-validator-rs` | A4a | after A2 | committed |
| `feat/rust-skill-auditor` | A4b | after A4a | committed |
| `feat/auditor-dist` | A5 | after A3, A4b | committed |
| `chore/retire-go` | A6 | after A5 | post-gate |
| `feat/adr-crate` | A7 | after A5 | post-gate |
| `feat/journal-crate` | A8 | after A5 | post-gate |
| `chore/drop-ts-cli` | B5 | after A5, B2a | post-gate |
| `feat/rust-validator-<name>` (×7) | C2b | after A3 | post-gate |
| `chore/remove-python` | C3 | after C2, C2b | post-gate |
| `feat/docs-tools-section` | B2b | after A5, A7, A8 | post-gate |

Prefer stacked PRs within a track (`pr-stacker`). B and C tracks can start against `main` immediately. Multi-milestone effort; do not attach dates past the A5 gate until go/no-go.

## Definition of Done

- [ ] All wave verification gates pass; go/no-go recorded after A5.
- [ ] All branches merged to `main`; CI green on `main`.
- [ ] No Go, no TS `cli/`, and no Python except the documented allowlist.
- [ ] `skill-auditor`, `adr`, `journal`, and the 7 validator binaries install standalone (macOS/Linux) and register their skills via `skill install`.
- [ ] release-please and cargo-dist coexist under the R1 policy with no stale entries.
- [ ] Acceptance coverage equivalent to the old cucumber suite is green.
- [ ] `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, and the Astro site describe the new install/audit/publish story.

## Revision history

- **v3 (14-07-2026)** — Interview decisions applied: validators as 7 per-tool binaries; Python research skills kept via allowlist + frontmatter marker (zero-Python softened); Windows deferred (Unix-first); scope milestone-gated with an explicit go/no-go after A5; skill registration via explicit `skill install` subcommand; versioning left to the R1 spike; `journal` promoted to a tool-backed crate (A8). Re-sequenced into a committed phase (Waves 1-5) and a post-gate phase (Waves 6-7).
- **v2 (14-07-2026)** — Multi-agent review corrections: added the standalone `skill-validator` retirement, release-please×cargo-dist coexistence (R1), split A4 into A4a/A4b with a full LOC budget, corrected the Python-track premise and the `regexp2→fancy-regex` phantom, fixed the installer-registration assumption, made the golden test tolerance-based, decoupled B5 from A6, added CI/test/rollback tasks.
- **v1 (13-07-2026)** — Initial six-wave plan from the investigation.
