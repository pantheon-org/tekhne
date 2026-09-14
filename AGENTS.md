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
- `scripts/`: Executable scripts with proper shebangs (sh/bash/python3/bun/node).
- Skills must be self-contained: no `../` paths, no absolute `skills/X/Y` paths, no `.context/` or `.agents/` references in SKILL.md (fenced code blocks and inline code spans are exempt).

### Tessl Registry Preparation

- **Agent agnostic**: Avoid features specific to individual AI assistants.
- **Quality threshold**: Target A-grade (>=126/140) using `skill-quality-auditor`.
- **Single responsibility**: Each skill solves one well-defined problem domain.

## Validation Commands

```bash
bunx @biomejs/biome check .
bunx markdownlint-cli2 "**/*.md"
```

## Skill Quality Audits

Run before publishing or committing major changes. See `skills/agentic-harness/skill-quality-auditor/SKILL.md` for full workflow.

```bash
pantheon-skill-auditor evaluate <domain>/<skill-name> --json --store
```

Grades: **A** ≥126/140 · **B+** 119-125 · **B** 112-118 · **C/C+** <112 (blocked from publishing).

Build the auditor from source with `bun run build:skill-auditor` (a shortcut for `cargo build --release -p pantheon-skill-auditor`), then invoke `target/release/pantheon-skill-auditor evaluate`.

## Skill Management with Tessl

The Tessl registry lifecycle runs through the `tessl` CLI directly, wrapped by npm scripts:

```bash
bun run tessl:import      # tessl skill import
bun run tessl:lint        # tessl plugin lint
bun run tessl:review      # tessl review run
bun run tessl:publish     # tessl plugin publish
```

Use `tessl skill review --optimize` for skills scoring below 90%. The former bulk `tessl manage` and `publish-check` commands were retired with the TypeScript CLI.

### Skills distributed via crate installers (not the registry)

Three skills are embedded into Rust crates at build time and distributed only by
their crate's `install` command, never through the Tessl registry:

- `documentation/journal-entry-creator` (embedded in the `journal` crate)
- `documentation/adr-creator` (embedded in the `adr` crate)
- `agentic-harness/skill-quality-auditor` (embedded in the `skill-auditor` crate)

These have `"private": true` in their `.tessl-plugin/plugin.json` and are omitted
from `release-please-config.json` and `.release-please-manifest.json`, so the crate
version is canonical and `tessl:publish` skips them. Their previously published
registry versions were archived with `tessl plugin archive`. Continue running
evals, audits, and quality tooling on their `SKILL.md` as normal; only the registry
publish step is retired. Install them with `pantheon-journal skill install`, `adr skill
install`, or `pantheon-skill-auditor skill install`.

## Commit Conventions

Pull requests are squash-merged, so the PR title becomes the commit message
on `main` and is what release-please parses to pick the version bump and
write the changelog. A malformed title produces a wrong bump or no release
at all, so CI checks it.

Format: `<type>[optional scope][optional !]: <description>`, description
starting lowercase. Use `!` for a breaking change and explain it in a
`BREAKING CHANGE:` footer.

Allowed types, which are release-please's defaults:

```text
build  chore  ci  deps  docs  feat  fix  perf  refactor  revert  style  test
```

A type outside this list is ignored by release-please: the commit lands, no
changelog entry appears, and no version bump happens. That is why the gate
rejects them rather than letting them through.

The canonical list lives in `.github/commit-types.txt`. The gate's regex in
`.github/workflows/conventional-commits.yml` duplicates it on purpose, since
the gate runs on `pull_request_target` and should not read repository files
in that context. `scripts/check-commit-types.sh` fails if the two drift
apart, and `scripts/test-commit-title-regex.sh` exercises the regex against
real titles. Both run as pre-commit steps.

Intermediate commit subjects on a branch are not checked, because the
squash-merge discards them.

## cargo-deny licence failures

`cargo-deny` runs as a hard gate in Rust CI: advisories, licences, bans and
sources. A dependency whose licence is not in `deny.toml`'s allowlist fails
the build rather than warning, so the first time this fires it will be
blocking a pull request.

The allowlist is deliberately minimal (`Apache-2.0`, `MIT`, `Unicode-3.0`,
`ISC`). It records what the tree actually needs rather than a broad
permissive set, so a new licence appearing is a real signal, not noise.

When the gate fails on a licence:

1. **Read which crate pulled it in.** cargo-deny names the crate and its
   dependency path. An unexpected licence deep in a transitive chain is
   worth more scrutiny than one on a direct dependency you just added.
2. **Check whether it is actually required.** An SPDX `OR` expression means
   you may pick either side, so `MIT OR GPL-3.0` is satisfied by MIT and
   needs no change. Only an `AND` makes both binding. This is why
   `Unicode-3.0` is on the list and `BSL-1.0` is not.
3. **Decide: exempt or reject.**
   - *Exempt* if the licence is permissive and compatible with this
     repository shipping ISC-licensed binaries. Add it to `deny.toml`'s
     `allow` list **with a comment naming the crate that needs it**, so the
     entry can be removed when that dependency goes.
   - *Reject* if it is copyleft, ambiguous, or unlicensed. Find an
     alternative crate, or drop the feature.
4. **Escalate anything you are unsure about.** Licence compatibility is a
   legal question, not an engineering one. If the answer is not obvious from
   the two categories above, route it rather than guessing.

Never silence the gate by widening the allowlist to a broad category, and
never add `continue-on-error` to the workflow step. The advisory-only
posture is exactly how the previous `cargo audit` step ended up never
blocking anything.

## Git Hooks

Pre-commit (`hk`, configured in `hk.pkl`): Biome on JS/TS/JSON, markdownlint on `.md`, YAML validation, artifact convention checks, skill structure validation, and the Python allowlist guardrail. Pre-push runs unit tests (`bun test scripts/`), integration tests (cucumber), and skill quality gates. Hooks are installed via `hk install` (run automatically by `bun install`); `hk` and its tools are pinned in `mise.toml`. The Python allowlist guardrail (`scripts/check-python-allowlist.sh`) is also enforced in CI by the Python Allowlist workflow, so a stray `.py` outside `python-allowlist.txt` cannot land by skipping the local hook.

Do not bypass hooks unless explicitly requested.

## Multi-Agent Development

Install skills locally with an ecosystem installer (`npx skills add ./skills --all`), or let a bundled tool install its own companion skill (`pantheon-skill-auditor skill install`, `pantheon-adr skill install`, `pantheon-journal skill install`).
See `README.md` for the full list of 41+ supported agents.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.

## Agent Rules <!-- tessl-managed -->

@.tessl/RULES.md follow the [instructions](.tessl/RULES.md)
