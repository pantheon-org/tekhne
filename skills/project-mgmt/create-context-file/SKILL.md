---
name: create-context-file
description: Creates structured context files (plans, justifications, scratches) in the .context/ directory with unique three-word IDs and frontmatter metadata. Use when starting a new task and needing to document a plan or task breakdown, when recording decision rationale or architectural choices as a justification, when saving temporary notes or exploratory thinking as a scratch, or when the user asks to "create a plan", "document my thinking", "save context", "track decisions", or "keep notes" in a structured project context directory.
license: MIT
compatibility: opencode
metadata:
  version: "0.1.0"
  audience: agents
  workflow: planning, decision-making, exploration
---

# create-context-file

## What I do

I create context files in the `.context/` directory for three purposes:
- **Plans** (`.context/plans/`): task breakdowns, feature specs, project plans
- **Justifications** (`.context/justifications/`): decision rationale, architectural choices, approach validation
- **Scratches** (`.context/scratches/`): temporary notes, exploratory ideas, idea pads

Each file gets a unique three-word identifier (e.g., `happy-blue-moon`), ensuring no filename conflicts, plus frontmatter with the current date and a formatted title.

## How to use

Run with type, slug, and content:
```bash
bunx .context/skills/create-context-file/scripts/create-context-file.ts --type plan "feature-name" "Plan content here"
```

Or use heredoc for multi-line content:
```bash
bunx .context/skills/create-context-file/scripts/create-context-file.ts --type justification "decision-name" << HEREDOC
Multi-line
justification content
goes here
HEREDOC
```

## File format

Files are created as: `{three-word-id}-{slug}.md`
Example: `happy-blue-moon-feature-name.md`

The file includes frontmatter:
```markdown
---
date: 2026-01-13
title: Feature Name
---
---
Your content here
```

## When to Use

Use create-context-file when:
- **Starting new work**: Document plan before writing code (prevents scope creep)
- **Decision points**: Record why you chose approach A over B (future reference)
- **Exploratory thinking**: Save scratch notes during research (avoid losing insights)
- **User requests**: "create a plan", "document this decision", "save my notes"

**Do NOT use for:**
- **Code comments**: Put technical notes in source files directly
- **External docs**: Use proper documentation systems (Confluence, Notion)
- **Long-term specs**: Use README.md or `/docs` for permanent documentation
- **Task tracking**: Use issue trackers, not context files

## File Type Selection Guide

| Type | Purpose | Lifecycle | Example Use Cases |
|------|---------|-----------|-------------------|
| **plan** | Task breakdown before execution | Delete after task complete | Feature specs, refactoring roadmaps, migration plans |
| **justification** | Record decision rationale | Keep indefinitely | Architecture choices, library selection, approach validation |
| **scratch** | Temporary exploratory notes | Delete after conclusion | Research notes, debugging hypotheses, idea capture |

**Decision rule:** If you'll reference it later → **justification**. If it's temporary → **scratch**. If it guides execution → **plan**.

## Anti-Patterns

### NEVER create context files without reading existing .context/ structure first

- **WHY**: duplicate or misplaced files violate organization conventions and confuse searches.
- **BAD**: immediately run `create-context-file.ts --type plan "my-plan"` without checking what exists.
- **GOOD**: `ls .context/plans/` first to see existing plans, avoid duplicates, follow naming patterns.

### NEVER use generic slugs like "notes" or "todo"

- **WHY**: collisions are likely when multiple tasks use vague names; three-word IDs aren't enough if slugs duplicate.
- **BAD**: `--type scratch "notes"` → creates `happy-blue-moon-notes.md` but another task may have `sad-red-sky-notes.md` causing confusion.
- **GOOD**: specific slugs tied to task: `--type scratch "api-refactor-notes"` or `--type plan "user-auth-migration"`.

### NEVER mix content types in one file

- **WHY**: plans, justifications, and scratches have different lifecycles and audiences; mixing creates cleanup confusion.
- **BAD**: `--type plan "feature"` but content includes justification rationale AND scratch exploration.
- **GOOD**: separate files — `--type plan "feature-implementation"`, `--type justification "why-graphql"`, `--type scratch "api-alternatives"`.

### NEVER skip frontmatter or use invalid date formats

- **WHY**: frontmatter powers metadata queries and date sorting; invalid formats break tooling.
- **BAD**: manually edit file to remove frontmatter or use `date: Jan 13th` (invalid).
- **GOOD**: script-generated frontmatter with `date: 2026-01-13` (ISO 8601) always included.

## References
