---
name: agents-md
description: Create and maintain AGENTS.md documentation for simple projects and complex monorepos with deterministic discovery, scoped instruction files, and low-token navigation patterns; use when generating AGENTS.md, updating agent docs, or standardizing AI-facing project guidance.
---

# AGENTS.md Management

## When to Use

- "Create AGENTS.md for this repo."
- "Update agent documentation for this monorepo."
- "Set up hierarchical AGENTS.md files by package."

## When Not to Use

- Pure code implementation work with no documentation update.
- One-off prompts where repository guidance files are unnecessary.

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

WHY: incorrect assumptions produce unusable instructions.
BAD: generate React/Jest guidance without evidence. GOOD: run discovery commands and map docs to detected stack.

### NEVER dump encyclopedic content into root AGENTS.md

WHY: oversized docs increase token cost and reduce usability.
BAD: embed full framework manuals. GOOD: keep root concise and link to scoped files/references.

### NEVER duplicate the same instructions across root and sub-files

WHY: duplication creates drift and maintenance overhead.
BAD: copy/paste identical conventions in every file. GOOD: keep universal rules at root and package-specific rules locally.

### NEVER provide unverified commands

WHY: broken commands erode trust and block contributors.
BAD: include hypothetical commands. GOOD: include only validated copy-paste commands.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Repository discovery commands | [references/discovery-commands.md](references/discovery-commands.md) |
| What to avoid | [references/anti-patterns.md](references/anti-patterns.md) |
| API package template | [references/api-template.md](references/api-template.md) |
| Design-system template | [references/design-system-template.md](references/design-system-template.md) |
| Database package template | [references/database-template.md](references/database-template.md) |
| Testing package template | [references/testing-template.md](references/testing-template.md) |
| Troubleshooting | [references/troubleshooting.md](references/troubleshooting.md) |

## Verification

```bash
bunx markdownlint-cli2 "**/AGENTS.md" "skills/agents-md/**/*.md"
sh skills/skill-quality-auditor/scripts/evaluate.sh agents-md --json
```
