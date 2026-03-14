# Command Patterns Reference

Common patterns and templates for OpenCode slash commands.

## Pattern: Analysis Command (Read-Only)

Use `agent: plan` for commands that should never modify files.

```markdown
---
description: Analyze code quality
agent: plan
---

<summary>
You MUST analyze $ARGUMENTS for code quality issues.
You SHOULD check for bugs, security issues, and anti-patterns.
You MUST provide a prioritized list of improvements.
</summary>

<target>
$ARGUMENTS
</target>

<objective>
Provide actionable feedback without making any changes.
</objective>
```

## Pattern: Fire-and-Forget (Background Task)

Use `subtask: true` for tasks that run independently and produce a single output.

```markdown
---
description: Generate test coverage report
subtask: true
model: anthropic/claude-3-5-haiku-20241022
---

<summary>
You MUST generate a test coverage report.
You SHOULD identify untested code paths.
You MUST output a markdown summary.
</summary>

<context>
!`npm run test -- --coverage --json 2>/dev/null | tail -100`
</context>

<objective>
Analyze coverage and produce a markdown report of untested paths.
</objective>
```

## Pattern: Contextual Command (Dynamic Injection)

Use `!cmd` to inject live shell output and `@file` for file content.

```markdown
---
description: Review recent changes
---

<summary>
You MUST review recent git changes.
You SHOULD identify potential issues or improvements.
You MUST provide actionable feedback.
</summary>

<context>
!`git log --oneline -10`
!`git diff HEAD~5`
</context>

<objective>
Review the changes above and provide specific, actionable feedback.
</objective>
```

## Pattern: Multi-Argument Command

Use `$1`, `$2` when two distinct pieces of information are needed.

```markdown
---
description: Migrate component to new pattern
---

<summary>
You MUST migrate $1 from $2 pattern to the new pattern.
You SHOULD preserve existing behavior.
You MUST run tests after migration.
</summary>

<target>
Component: $1
From pattern: $2
</target>

<objective>
Perform the migration and verify tests still pass.
</objective>
```

Usage: `/migrate UserCard class-component`

## Pattern: Scope-Limited Command

Use `@file` references to constrain context to relevant files.

```markdown
---
description: Generate API client from schema
---

<summary>
You MUST generate a TypeScript API client.
You SHOULD follow patterns in existing client code.
You MUST export typed functions for each endpoint.
</summary>

<schema>
@openapi.yaml
</schema>

<existing_client>
@src/api/client.ts
</existing_client>

<objective>
Generate TypeScript functions for each endpoint following existing patterns.
</objective>
```

## Decision Guide: Which Pattern to Use

| Requirement | Pattern | Key Setting |
|-------------|---------|-------------|
| Read-only analysis | Analysis | `agent: plan` |
| Background processing | Fire-and-forget | `subtask: true` |
| Needs live data | Contextual | `!cmd` injection |
| Two input variables | Multi-argument | `$1`, `$2` |
| Schema or config driven | Scope-limited | `@file` reference |
| Default (interactive) | None | Omit all flags |

## Naming Conventions

| Command Name | Convention | Example |
|-------------|------------|---------|
| File → `/filename` | Kebab-case | `code-review.md` → `/code-review` |
| Short action verbs | Prefer imperative | `/test`, `/review`, `/deploy` |
| Qualified actions | Noun-verb | `/db-migrate`, `/api-generate` |

## Best Practices Summary

1. **Single responsibility** — One command does one thing
2. **Use XML blocks** — Separate objectives, context, and user input
3. **Inject, don't hardcode** — `!cmd` for dynamic data, `@file` for configs
4. **Match agent to purpose** — `plan` for analysis, default for changes
5. **Start with `$ARGUMENTS`** — Only use `$1`/`$2` when order matters
