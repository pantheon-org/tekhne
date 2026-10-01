---
name: biome-complete
description: Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows.
---

# Biome Complete Toolchain

## When to Use

Use this skill when the request includes:

- "set up Biome"
- "configure biome.json"
- "migrate from ESLint" or "migrate from Prettier"
- "fix lint and format drift"
- "run Biome in CI"
- "tune rule severity" or add a targeted suppression

## When Not to Use

- The user asks for ESLint-only or Prettier-only solutions.
- The work is about a language or file type Biome does not support.
- The request concerns type checking or test running rather than linting and formatting.

## Principles

1. Use one source of truth for linting and formatting.
2. Prefer deterministic commands and verifiable output.
3. Keep SKILL.md short; move deep details to `references/`.

## Deterministic Workflow

1. Confirm scope: migration, config, lint, format, or CI.
2. Initialize config if missing: `bunx @biomejs/biome init`.
3. Run checks and capture results: `bunx @biomejs/biome check .`.
   - **Verify:** Confirm diagnostics are printed. If command fails, check Node.js version (≥14) and file permissions.
4. Apply safe autofixes: `bunx @biomejs/biome check . --write`.
   - **Verify:** Re-run `biome check .` and confirm reduced error count. If errors persist, review unsupported rules or migration conflicts in `references/migration-eslint-prettier.md`.
5. Add targeted suppressions only when justified.
6. Verify commands pass in local and CI contexts: `bunx @biomejs/biome check . --error-on-warnings`.
   - **Verify:** Exit code 0 indicates success. If non-zero, review remaining diagnostics and address or document exceptions.

**Error Recovery:** If `biome check` fails after migration, isolate conflicting rules by temporarily disabling rule groups in `biome.json` linter section, then re-enable one group at a time to identify the source.

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

### NEVER leave ESLint or Prettier wired in after migrating

**WHY:** leftover scripts and dev dependencies keep two sources of truth alive and invite the conflicts the migration was meant to remove.

**BAD:** `package.json` still has `eslint` and `prettier` scripts and packages beside the new Biome scripts.

**GOOD:** remove the old scripts and dev dependencies and record the removal in the migration notes.

### NEVER run the CI check without `--error-on-warnings`

**WHY:** without the flag, warnings pass silently and the CI result no longer matches the strict local expectation.

**BAD:** `bunx @biomejs/biome check .` as the CI step.

**GOOD:** `bunx @biomejs/biome check . --error-on-warnings`.

### NEVER add a suppression without a reason

**WHY:** an unexplained suppression cannot be reviewed or retired, so it becomes permanent debt.

**BAD:** a bare ignore comment with no explanation.

**GOOD:** an inline `biome-ignore` comment naming the rule, the reason and the follow-up ticket.

### NEVER re-enable every rule group at once when isolating a failure

**WHY:** enabling all groups together hides which one conflicts, so the source of the failure is never found.

**BAD:** disable the whole linter, then switch everything back on in one step.

**GOOD:** disable rule groups in the linter section, then re-enable one group at a time until the failure returns.

### NEVER declare success without re-running the check

**WHY:** a fix that was never verified may leave errors behind, and an unverified exit code says nothing about CI.

**BAD:** applying `--write` and reporting the repository clean.

**GOOD:** re-run `biome check .`, confirm the error count dropped, and confirm exit code 0 with `--error-on-warnings`.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Configuration | [references/config-biome-json.md](references/config-biome-json.md) | Full biome.json schema, formatter width, linter domains and VCS ignore patterns |
| Lint rules | [references/linting-rule-categories.md](references/linting-rule-categories.md) | Rule groups, severity levels and category-level enable or disable patterns |
| Formatter | [references/formatter-options.md](references/formatter-options.md) | Indent style, line width, quote style and language-specific overrides |
| Migration | [references/migration-eslint-prettier.md](references/migration-eslint-prettier.md) | Step-by-step migration, rule mapping and conflict resolution |
