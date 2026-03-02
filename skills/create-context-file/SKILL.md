---
name: create-context-file
description: Creates structured context files (plans, justifications, scratches) in the .context/ directory with unique three-word IDs and frontmatter metadata. Use when starting a new task and needing to document a plan or task breakdown, when recording decision rationale or architectural choices as a justification, when saving temporary notes or exploratory thinking as a scratch, or when the user asks to "create a plan", "document my thinking", "save context", "track decisions", or "keep notes" in a structured project context directory.
license: MIT
compatibility: opencode
metadata:
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
Your content here
```
