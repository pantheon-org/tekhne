# Tekhne Agent Collaboration Guide

This file defines how LLM agents should work in this repository.

## Project Purpose

- Maintain a curated collection of reusable agent skills under `skills/`.
- Keep skill content modular, discoverable, and easy to lint/validate.
- Prefer small, targeted edits over broad rewrites.

## Repository Map

- `skills/<skill-name>/SKILL.md`: Primary entry point for a skill.
- `skills/<skill-name>/AGENTS.md`: Optional deep navigation for that skill.
- `skills/<skill-name>/references/`: Focused reference documents.
- `skills/<skill-name>/scripts/`: Utility scripts used by a skill.
- `skills/<skill-name>/templates/`: Reusable templates (mostly YAML).
- `skills/<skill-name>/schemas/`: JSON schemas for validation.
- `.context/`: Working notes and generated analysis artifacts.

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
- Files under `skills/<skill-name>/templates/` must use YAML extensions (`.yaml` or `.yml`).
- Files under `skills/<skill-name>/schemas/` must be JSON Schema files named `*.schema.json` and include a `"$schema"` URL from `json-schema.org`.
- Files under `skills/<skill-name>/scripts/` must be portable shell scripts (`.sh`) with `#!/usr/bin/env sh`.

### Tessl Registry Preparation

When preparing skills for [Tessl](https://tessl.io/) submission, follow these additional guidelines:

- **Performance focus**: Write skills that provide measurable effectiveness improvements
- **Agent agnostic**: Avoid features specific to individual AI assistants
- **Quality threshold**: Target A-grade scores (≥108 points) using skill-quality-auditor
- **Single responsibility**: Each skill should solve one well-defined problem domain
- **Comprehensive examples**: Include practical use cases with expected outcomes
- **Cross-platform testing**: Validate functionality across multiple agents when possible
- **Clear success metrics**: Define what "effective use" looks like for evaluation

## Validation Commands

Run these from repository root:

```bash
bunx @biomejs/biome check .
bunx markdownlint-cli2 "**/*.md"
```

For artifact convention checks (`templates/`, `schemas/`, `scripts/`), use the `skill-quality-auditor` workflow documented in `skills/skill-quality-auditor/SKILL.md`.

## Git Hooks

- Pre-commit uses `lefthook` and runs:
- Biome checks on staged JS/TS/JSON files (with `--write`).
- markdownlint on staged `.md` files.
- YAML parse validation with `yq` when available.
- Skill artifact conventions for `templates/`, `schemas/`, and `scripts/`.

Do not bypass hooks unless explicitly requested by the user.

## Multi-Agent Development

This repository follows the [Agent Skills specification](https://agentskills.io) for cross-harness compatibility.

### Syncing Skills to Your Harness

Contributors can sync local skills to their development environment:

```bash
# Sync all skills to all detected agents
npx skills add ./skills --all

# Sync to specific agents
npx skills add ./skills -a claude-code -a cursor -a gemini-cli

# Sync globally for cross-project access
npx skills add ./skills --all -g
```

### Cross-Agent Compatibility

When authoring skills for multiple agents:

- Use standard `SKILL.md` frontmatter format (compatible with all harnesses)
- The `allowed-tools` field is agent-specific; check support before using
- Avoid agent-specific features in shared skills
- Test with `npx skills add ./skills --dry-run` before committing

### Supported Harnesses

See README.md for the full list of 41+ supported agents via `npx skills`.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.
