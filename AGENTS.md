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
- `cli/`: TypeScript CLI tool for skill management, auditing, and installation.

**Note:** Generator/validator pairs are consolidated at the tool level (e.g. `terraform-generator` and `terraform-validator` share `skills/infrastructure/terraform/tile.json`).

## Domain Organization

12 domains: `ci-cd/`, `infrastructure/`, `repository-mgmt/`, `development/`, `agentic-harness/`, `testing/`, `software-engineering/`, `observability/`, `documentation/`, `package-mgmt/`, `project-mgmt/`, `specialized/`.

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
- Skills must be self-contained: no `../` paths, no absolute `skills/X/Y` paths, no `.context/` or `.agents/` references in SKILL.md (code blocks exempt).

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
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
```

Grades: **A** ≥126/140 · **B+** 119-125 · **B** 112-118 · **C/C+** <112 (blocked from publishing).

CLI shortcuts:

```bash
bun cli/index.ts audit skill <domain>/<skill-name>
bun cli/index.ts audit all
bun cli/index.ts audit status
```

## Skill Management with Tessl

```bash
bun cli/index.ts tessl manage                      # all skills
bun cli/index.ts tessl manage <domain>/<skill-name>
bun cli/index.ts tessl publish-check <tile-path>
```

See `cli/README.md` for full usage. Use `tessl skill review --optimize` for skills scoring below 90%.

## CLI TypeScript Conventions

All `cli/` files follow 7 rules (arrow functions, barrel modules, collocated tests, no nested functions, ≤150-line bodies, one export per module, ≤10 modules per dir). See `cli/CONVENTIONS.md`.

## Git Hooks

Pre-commit (`lefthook`): Biome on JS/TS/JSON, markdownlint on `.md`, YAML validation, artifact convention checks, CLI convention checks.

Do not bypass hooks unless explicitly requested.

## Multi-Agent Development

Install skills locally: `bun cli/index.ts install` or `npx skills add ./skills --all`.
See `README.md` for the full list of 41+ supported agents.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.

## Agent Rules <!-- tessl-managed -->

@.tessl/RULES.md follow the [instructions](.tessl/RULES.md)
