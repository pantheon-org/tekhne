---
name: agents-md
description: Create and maintain AGENTS.md documentation for simple projects, complex monorepos, deterministic discovery, scoped instruction files, low-token navigation; use when generating AGENTS.md, updating agent docs, standardizing AI-facing project guidance.
---

# AGENTS.md Management

## When to Use

- "Create AGENTS.md for this repo."
- "Update agent documentation for this monorepo."
- "Set up hierarchical AGENTS.md files by package."
- Repository shape or tooling changed and the existing agent guidance needs a refresh.

## When Not to Use

- Pure code implementation work with no documentation update.
- One-off prompts where repository guidance files are unnecessary.
- Questions about a single function or file that need no standing project guidance.

## Principles

- Keep instructions concise, concrete, and path-specific.
- Prefer references and indices over duplicated prose.
- Optimize for nearest-file relevance in hierarchical layouts.

## Workflow

1. Discover repository shape and technologies.
Output: simple vs hierarchical documentation strategy.
2. Select structure (single root file or root + sub-files).
Output: file layout plan.
3. Generate AGENTS.md content with concrete commands and paths.
Output: actionable docs with JIT indexing.
4. Validate command correctness and duplication boundaries.
Output: clean, copy-paste-safe instruction files.
5. Re-check after major repo changes.
Output: synchronized documentation.

## Structure Decision

- Simple project: one AGENTS.md when stack and patterns are uniform.
- Complex monorepo: root AGENTS.md + scoped subdirectory AGENTS.md files.
- Consider adding subdirectory files iteratively — start minimal, expand as complexity grows.

## Minimal Template

```markdown
# Project Agent Guide

## Commands
- Build: `bun run build`
- Test: `bun run test`
- Lint: `bun run lint`

## Conventions
- [describe key patterns here]

## Key Paths
- Source: `src/`
- Tests: colocated `*.test.ts`
```

## Quick Commands

```bash
# Discovery baseline
rg --files
```

```bash
# Detect core config/tooling
rg -n "workspaces|nx|turbo|pnpm|yarn|packageManager|tsconfig|pytest|playwright" .
```

```bash
# Locate existing AGENTS files
find . -name AGENTS.md -o -name AI-DOCS.md
```

## Anti-Patterns

### NEVER assume a technology stack without discovery

**WHY:** incorrect assumptions produce unusable instructions.

**BAD:** generate React/Jest guidance without evidence.

**GOOD:** run discovery commands and map docs to the detected stack.

### NEVER dump encyclopedic content into root AGENTS.md

**WHY:** oversized docs increase token cost and reduce usability.

**BAD:** embed full framework manuals.

**GOOD:** keep root concise and link to scoped files or references.

### NEVER duplicate the same instructions across root and sub-files

**WHY:** duplication creates drift and maintenance overhead.

**BAD:** copy/paste identical conventions in every file.

**GOOD:** keep universal rules at root and package-specific rules locally.

### NEVER provide unverified commands

**WHY:** broken commands erode trust and block contributors.

**BAD:** include hypothetical commands.

**GOOD:** include only validated copy-paste commands.

### NEVER create a hierarchy for a simple, uniform project

**WHY:** extra files add navigation cost and drift risk when one file would do.

**BAD:** add a subdirectory AGENTS.md to every folder of a single-package repo.

**GOOD:** use one root AGENTS.md when stack and patterns are uniform.

### NEVER put package-specific rules in the root file

**WHY:** agents read the nearest file, so misplaced rules waste tokens and mislead other packages.

**BAD:** list UI build flags in the root file of a monorepo.

**GOOD:** keep package commands in that package's own AGENTS.md.

### NEVER use generic placeholder commands or paths

**WHY:** placeholders cannot be copy-pasted and give the reader nothing concrete.

**BAD:** write `<your-project>/src` or "run the tests".

**GOOD:** write the real path and the real command found during discovery.

### NEVER front-load every sub-file at once

**WHY:** speculative files describe complexity that does not exist yet and go stale.

**BAD:** generate an AGENTS.md for every package on the first pass.

**GOOD:** start minimal and add scoped files iteratively as complexity grows.

### NEVER leave documentation stale after major repository changes

**WHY:** instructions for removed tooling mislead agents and break their commands.

**BAD:** keep Jest commands after the project moved to another runner.

**GOOD:** re-run discovery, remove obsolete content, and document new scripts.

## Verification

```bash
bunx markdownlint-cli2 "**/AGENTS.md"
```

```bash
bun run lint
```

## References

| Topic | Reference |
| --- | --- |
| Repository discovery commands | [references/discovery-commands.md](references/discovery-commands.md) |
| What to avoid | [references/anti-patterns.md](references/anti-patterns.md) |
| API package template | [references/api-template.md](references/api-template.md) |
| Design-system template | [references/design-system-template.md](references/design-system-template.md) |
| Database package template | [references/database-template.md](references/database-template.md) |
| Testing package template | [references/testing-template.md](references/testing-template.md) |
| Troubleshooting | [references/troubleshooting.md](references/troubleshooting.md) |
