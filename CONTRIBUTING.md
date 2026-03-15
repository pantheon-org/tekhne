# Contributing to Tekhne

## Quick start

```bash
# Install dependencies
bun install

# Run linting
bunx @biomejs/biome check .

# Run markdown lint
bunx markdownlint-cli2 "**/*.md"
```

Pre-commit hooks (via `lefthook`) run Biome, markdownlint, YAML validation, and
CLI TypeScript convention checks automatically on staged files.

## Adding or editing skills

1. Read `AGENTS.md` for domain organisation and authoring rules.
2. Read the existing `SKILL.md` in the skill directory before editing it.
3. Run the quality audit before publishing:

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
```

Target B-grade (112/140) minimum; A-grade (126/140) for publication.

## CLI (`cli/`)

The CLI is a TypeScript project built with Bun and Commander.js.

### TypeScript conventions

All files under `cli/` follow **7 enforced rules** documented in full in
[`cli/CONVENTIONS.md`](cli/CONVENTIONS.md):

| # | Rule |
|---|------|
| 1 | Arrow functions only |
| 2 | Barrel modules (`index.ts` per subdirectory) |
| 3 | Collocated tests (`foo.test.ts` beside `foo.ts`) |
| 4 | No internal (nested) functions |
| 5 | Function body ≤ 150 lines |
| 6 | One exported function per module |
| 7 | Directory organisation by layer |

### Library directory structure

Modules inside `cli/lib/<domain>/` are organised into responsibility layers.
Imports only flow **downward**:

```
types/       ← pure type definitions and constants
    ↓
parsing/     ← pure string/value parsing (no I/O)
    ↓
discovery/   ← filesystem traversal, metadata reading
    ↓
sections/    ← boundary detection for structured documents
    ↓
rendering/   ← markdown builders and output generators
```

A module may import from the same layer (`./module-name`) or a lower one
(`../types`, `../parsing`, …). It must never import upward.

### Running tests

```bash
bun test
```

### Validating CLI conventions

```bash
bun cli/scripts/validate-ts-conventions.ts
```

## Commit style

Follow [Conventional Commits](https://www.conventionalcommits.org/):
`feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`.
