---
name: create-context-file
description: Create context files (plans, justifications, scratches) in .context/ directory with unique three-word IDs and frontmatter
license: MIT
compatibility: opencode
metadata:
  audience: agents
  workflow: planning, decision-making, exploration
---

## What I do
I create context files in the `.context/` directory for different purposes:
- **Plans**: `.context/plans/` - for feature/task planning
- **Justifications**: `.context/justifications/` - for decision documentation  
- **Scratches**: `.context/scratches/` - for temporary notes and exploration
Each file gets:
- A unique three-word identifier (e.g., `happy-blue-moon`)
- Frontmatter with current date and formatted title
- Content you provide

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

## When to use me
Use this skill when you need to create any type of context document:
- **Plan**: Document feature specifications, task breakdowns, project plans
- **Justification**: Document reasoning for decisions, architectural choices, approach validation
- **Scratch**: Create temporary notes, exploration documents, idea pads

The unique ID ensures no filename conflicts, and the frontmatter provides metadata for organization.

## Types available
- `plan` - For feature/task planning documents
- `justification` - For decision documentation and reasoning
- `scratch` - For temporary notes and exploration