# Tekhne Agent Collaboration Guide

This file defines how LLM agents should work in this repository.

## Project Purpose

- Maintain a curated collection of reusable agent skills under `skills/`.
- Keep skill content modular, discoverable, and easy to lint/validate.
- Prefer small, targeted edits over broad rewrites.

## Repository Map

- `skills/<domain>/<skill-name>/SKILL.md`: Primary entry point for a skill.
- `skills/<domain>/<tool>/{generator,validator}/SKILL.md`: Generator/validator pairs (consolidated under `<tool>/tile.json`).
- `skills/<domain>/<skill-name>/AGENTS.md`: Optional deep navigation for that skill.
- `skills/<domain>/<skill-name>/references/`: Focused reference documents.
- `skills/<domain>/<skill-name>/scripts/`: Utility scripts used by a skill.
- `skills/<domain>/<skill-name>/templates/`: Reusable templates (mostly YAML).
- `skills/<domain>/<skill-name>/schemas/`: JSON schemas for validation.
- `.context/`: Working notes and generated analysis artifacts.
- `crates/`: Rust tools (`skill-auditor`, `skill-validator-rs`, `adr`, `journal`) for auditing, validation, and skill distribution.
- `scripts/catalog/`: TypeScript catalog generator for `README.md` and the docs tiles page.

**Note:** Generator/validator pairs are consolidated at the tool level (e.g. `terraform-generator` and `terraform-validator` share `skills/infrastructure/terraform/tile.json`).

## Domain Organization

13 domains: `ci-cd/`, `infrastructure/`, `repository-mgmt/`, `development/`, `agentic-harness/`, `testing/`, `software-engineering/`, `observability/`, `documentation/`, `package-mgmt/`, `project-mgmt/`, `specialized/`, `languages/`.

See `skills/agentic-harness/skill-quality-auditor/references/skill-taxonomy.md` for classification criteria.

## Required Workflow For Agents

1. Read the relevant `SKILL.md` before editing files in that skill.
2. Load only the minimum additional files needed for the task.
3. Reuse existing templates/scripts instead of recreating content.
4. Keep changes scoped to the user request; do not refactor unrelated skills.
5. Run validation commands before finishing.

## Authoring Rules

- Use kebab-case for skill directory names.
- Keep instructions explicit, actionable, and deterministic.
- Prefer short sections and predictable headings.
- Store deep details in `references/` and keep `SKILL.md` as a navigation hub.
- Use Markdown fenced code blocks with language tags when applicable.
- Use ASCII unless a file already requires Unicode.
- `templates/`: YAML extensions (`.yaml` or `.yml`) only.
- `schemas/`: JSON Schema files named `*.schema.json` with a `"$schema"` URL.
- `scripts/`: Executable scripts with proper shebangs (sh/bash/bun/node/python3), chosen by the order in Language Preference below.
- Skills must be self-contained: no `../` paths, no absolute `skills/X/Y` paths, no `.context/` or `.agents/` references in SKILL.md (fenced code blocks and inline code spans are exempt).

### Language Preference

When writing code or scripts for this repository, use the first option that fits:

1. **Rust**, in the crates under `crates/`.
2. **Bun with TypeScript**, when Rust is not a good fit.
3. **Shell**, for thin glue only.
4. **Python**, as a last resort.

Python is kept out by a guardrail: any tracked `.py` file outside a directory listed in `python-allowlist.txt` fails the `python-allowlist` check (`scripts/check-python-allowlist.sh`, run locally by the `hk` hook and in CI), so new Python needs a reviewed allowlist entry.

### Skill Standards

- **Agent agnostic**: Avoid features specific to individual AI assistants.
- **Quality threshold**: Target A-grade (>=126/140) using `skill-quality-auditor`.
- **Single responsibility**: Each skill solves one well-defined problem domain.

## Validation Commands

```bash
bunx @biomejs/biome check .
mise run lint   # rumdl check . (config: .rumdl.toml)
```

Frontmatter limits enforced by `skill-validator-rs validate structure`: `description` over 1024 bytes is an error, and one within 32 bytes of it warns so the ceiling is visible before the next trigger phrase breaches it. An unquoted `description` containing `": "` or a line ending in `:` fails YAML parsing, and the error says so; quote the value or use a block scalar (`description: |-`).

## Pull Requests

Open pull requests from `.github/pull_request_template.md` and keep its three headings, `## What`, `## Why` and `## Notes`, in that order. Use a conventional-commit title (`type(scope): lowercase summary`), put verification evidence and anything left for later under Notes, and end with `Refs #N` for partial work or `Closes #N` when the pull request finishes the issue.

Never include an agent session link, session ID or session trailer (for example `Claude-Session:`) in commit messages, pull request descriptions or comments. A session URL identifies a private session and must not be published.

## Skill Quality Audits

Run before publishing or committing major changes. See `skills/agentic-harness/skill-quality-auditor/SKILL.md` for full workflow.

```bash
pantheon-skill-auditor evaluate <domain>/<skill-name> --json --store
```

Grades: **A** ≥126/140 · **B+** 119-125 · **B** 112-118 · **C/C+** <112 (blocked from publishing).

The pull request Skill Audit grades every skill that has any changed file, not only a changed `SKILL.md`, and audits the same skills at the base commit so the scores compare. A skill graded below B blocks the pull request only if it is new or scores lower than at the base commit. A skill that is already below B and not lowered passes with a warning and is tracked by one open GitHub issue (`scripts/skill-audit/classify.sh`, `file-issues.sh`), linked to the pull request and listed in the Skill Audit comment. Nothing closes the issue automatically; close it when the grade is fixed.

### Trigger phrase drift

A rewrite can drop a phrase from a skill's `description` that auto-invocation relied on while the grade stays flat. The Skill Audit comment lists each trigger phrase a changed skill dropped compared with the base commit. It is advisory and never fails the run: a drop is often a deliberate consolidation, so confirm the description still names the intent or restore the phrase. Trigger phrases are the comma-separated clauses after `Use when` and after `Keywords:`; a description with neither marker is not checked, and a new skill has nothing to compare. A base phrase counts as kept when its lowercased text still appears anywhere in the new description and `when_to_use`, so reordering or rewrapping is not a drop. Run it locally against a checkout of the base:

```bash
skill-validator-rs analyze triggers skills/<domain>/<skill> --base <base-checkout>/skills/<domain>/<skill>
```

### Skill listing budget

Every listing-eligible skill's name and description share one budget in the model's context (`skillListingBudgetFraction`, 1% of the context window; the documented fallback is 8000 characters). When it overflows, descriptions of the least-invoked skills are dropped with no signal, so each description you add spends it for every other skill. `skill-validator-rs analyze listing` estimates the aggregate. It is advisory and always exits 0; the Skill Audit workflow prints it in the job summary.

```bash
mise run audit:listing                        # documented 8000-char fallback
mise run audit:listing -- --fraction 0.01     # band at 200k and 1M tokens
mise run audit:listing -- --context-tokens 1000000
mise run audit:listing -- --from-settings     # this machine's Claude settings
```

`--budget-chars` fixes the budget outright; `--chars-per-token` (default 4) changes the derivation. Precedence is `--budget-chars`, then `--context-tokens`, then `--fraction` alone (a band), then the 8000 fallback. `--from-settings` fills only what the flags leave unset: `SLASH_COMMAND_TOOL_CHAR_BUDGET`, then `skillListingBudgetFraction` from the first of project `.claude/settings.local.json`, project `.claude/settings.json`, and `~/.claude/settings.json` (`CLAUDE_CONFIG_DIR` relocates it). Managed policy settings are not read. A skill with `disable-model-invocation: true` spends none of it, and a description is capped at 1536 characters per entry. The 4 chars/token ratio and the 200k and 1M windows are assumptions of the estimate, not documented values.

Build the auditor from source with `mise run build:skill-auditor` (a shortcut for `cargo build --release -p pantheon-skill-auditor`), then invoke `target/release/pantheon-skill-auditor evaluate`.

## Skills distributed via crate installers

Three skills are embedded into Rust crates at build time and distributed only by
their crate's `install` command:

- `documentation/journal-entry-creator` (embedded in the `journal` crate)
- `documentation/adr-creator` (embedded in the `adr` crate)
- `agentic-harness/skill-quality-auditor` (embedded in the `skill-auditor` crate)

These have `"private": true` in their `.tekhne/plugin.json` and are omitted
from `release-please-config.json` and `.release-please-manifest.json`, so the crate
version is canonical. Continue running evals, audits, and quality tooling on their
`SKILL.md` as normal. Install them with `pantheon-journal skill install`,
`pantheon-adr skill install`, or `pantheon-skill-auditor skill install`.

**A released binary only has the `SKILL.md` that existed when it was built.** Editing
`skills/documentation/adr-creator/SKILL.md` (or either of the other two) does not change
what a `PATH`-installed release binary installs until a new release is cut — the content
is embedded at compile time, not read from disk. During development, build from source
and install from that build instead of waiting on the PR → merge → release loop:

```bash
mise run install-skill:adr             # cargo build --release -p pantheon-adr && pantheon-adr skill install
mise run install-skill:journal         # same, for pantheon-journal
mise run install-skill:skill-auditor   # same, for pantheon-skill-auditor
```

Each builds the crate and immediately runs its `skill install`, so the copies it writes
(`~/.claude/skills/<name>`, and the other installed agents' skill dirs) reflect your
working tree, not the last release. `mise run build:adr` / `build:journal` /
`build:skill-auditor` do the build step alone, for when you just need the binary at
`target/release/`.

## Git Hooks

Pre-commit (`hk`, configured in `hk.pkl`): Biome on JS/TS/JSON, rumdl on `.md`, YAML validation, artifact convention checks, skill structure validation, and the Python allowlist guardrail. Pre-push runs unit tests (`bun test scripts/`), integration tests (cucumber), and skill quality gates. Hooks are installed via `hk install` (run automatically by `bun install`); `hk` and its tools are pinned in `mise.toml`. The Python allowlist guardrail (`scripts/check-python-allowlist.sh`) is also enforced in CI by the Python Allowlist workflow, so a stray `.py` outside `python-allowlist.txt` cannot land by skipping the local hook.

Do not bypass hooks unless explicitly requested.

## Multi-Agent Development

Install skills locally with an ecosystem installer (`npx skills add ./skills --all`), or let a bundled tool install its own companion skill (`pantheon-skill-auditor skill install`, `pantheon-adr skill install`, `pantheon-journal skill install`).
See `README.md` for the full list of 41+ supported agents.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.
