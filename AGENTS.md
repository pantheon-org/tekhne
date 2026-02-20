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

## Validation Commands

Run these from repository root:

```bash
bunx @biomejs/biome check .
bunx markdownlint-cli2 "**/*.md"
```

## Git Hooks

- Pre-commit uses `lefthook` and runs:
- Biome checks on staged JS/TS/JSON files (with `--write`).
- markdownlint on staged `.md` files.
- YAML parse validation with `yq` when available.

Do not bypass hooks unless explicitly requested by the user.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.
