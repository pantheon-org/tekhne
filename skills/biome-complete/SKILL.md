---
name: biome-complete
description: Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows. Keywords: biome, biome.json, linter, formatter, eslint migration, prettier migration, check, format, CI, VCS, suppression comments.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# Biome Complete Toolchain

## When to Apply

Use this skill when the request includes:

- "set up Biome"
- "configure biome.json"
- "migrate from ESLint" or "migrate from Prettier"
- "fix lint and format drift"
- "run Biome in CI"

## When Not to Apply

Do not use this skill when the user asks for ESLint-only or Prettier-only solutions.

## Principles

1. Use one source of truth for linting and formatting.
2. Prefer deterministic commands and verifiable output.
3. Keep SKILL.md short; move deep details to `references/`.

## Deterministic Workflow

1. Confirm scope: migration, config, lint, format, or CI.
2. Initialize config if missing.
3. Run checks and capture results.
4. Apply safe autofixes.
5. Add targeted suppressions only when justified.
6. Verify commands pass in local and CI contexts.

## Quick Commands

### Initialize

```bash
bunx @biomejs/biome init
```

Expected result: `biome.json` exists.

### Check repository

```bash
bunx @biomejs/biome check .
```

Expected result: diagnostics printed with file paths.

### Apply safe fixes

```bash
bunx @biomejs/biome check . --write
```

Expected result: fixable issues are rewritten.

### Format files

```bash
bunx @biomejs/biome format . --write
```

Expected result: formatting is normalized.

### Check one file

```bash
bunx @biomejs/biome check src/index.ts
```

Expected result: file-level diagnostics only.

### Run in CI

```bash
bunx @biomejs/biome check . --error-on-warnings
```

Expected result: non-zero exit when warnings or errors exist.

## Anti-Patterns

### NEVER run Biome and ESLint on the same files

**WHY:** Competing rules create contradictory output and noisy reviews.

**BAD:** ESLint and Biome both lint `src/**/*.ts`.
**GOOD:** Route TS linting and formatting through Biome only.

**Consequence:** Duplicate diagnostics and unstable CI outcomes.

### NEVER run Prettier and Biome formatter on the same files

**WHY:** Different formatting models cause churn in every commit.

**BAD:** `prettier --write .` and `biome format . --write` in the same pipeline.
**GOOD:** Keep only `biome format . --write` for supported files.

**Consequence:** Constant formatting diffs and merge friction.

### NEVER skip `biome.json` customization after init

**WHY:** Defaults may not match repository conventions.

**BAD:** Commit default config without reviewing formatter/linter settings.
**GOOD:** Define formatter width, linter domains, and VCS ignores explicitly.

**Consequence:** Inconsistent style and avoidable lint regressions.

### NEVER blanket-ignore diagnostics to get green CI

**WHY:** Broad suppressions hide real defects and debt.

**BAD:** Disable full rule groups without rationale.
**GOOD:** Add narrow suppressions with a reason and follow-up ticket.

**Consequence:** Quality silently degrades over time.

## References

- `references/config-biome-json.md`
- `references/linting-rule-categories.md`
- `references/formatter-options.md`
- `references/migration-eslint-prettier.md`
