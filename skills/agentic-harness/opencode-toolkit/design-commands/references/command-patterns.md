# Command Patterns Reference

Common patterns, creation workflow, and example commands for OpenCode slash commands.

---

## Creation Workflow

**Command creation is conversational, not transactional.**

- MUST NOT assume what the user wants — ask
- SHOULD start simple, add complexity only if needed
- MUST show drafts and iterate based on feedback

### Phase 1: Understand the Task

1. **"What task do you want to automate?"**
   - Get the repetitive prompt/workflow
   - Examples: "run tests", "review this file", "create a component"

2. **"Can you show me how you'd normally ask for this?"**
   - Get the actual prompt they'd type
   - This becomes the template body

### Phase 2: Inputs & Routing

1. **"Does it need arguments?"**
   - If they mention "this file", "a name", "the function" → yes
   - `/command foo bar` → `$ARGUMENTS` = "foo bar", `$1` = "foo", `$2` = "bar"

2. **"Should it make changes or just analyze?"**
   - Changes → default agent (build)
   - Analysis only → set `agent: plan`

3. **"Should it run as a background subtask?"**
   - Long-running or parallel work → set `subtask: true`
   - Interactive or quick → leave unset

4. **"Project-specific or use everywhere?"**
   - Project → `.opencode/commands/`
   - Global → `~/.config/opencode/commands/`

### Phase 3: Review & Refine

1. Show the draft command, ask for feedback
2. Iterate until user is satisfied

**Be flexible:** If user provides lots of info upfront, adapt — MUST NOT rigidly ask every question.

---

## Command File Format

```markdown
---
description: 3-word command summary
agent: build # Optional: build, plan, or custom agent
model: provider/model-id # Optional: override model
subtask: true # Optional: run as subagent
---

<summary>
Line 1: You MUST [purpose].
Line 2: You SHOULD [inputs].
Line 3: You MUST [outcome].
</summary>

<user_guidelines>
$ARGUMENTS
</user_guidelines>

Template body goes here.
```

### Frontmatter Options

| Field | Purpose | Required |
|-------|---------|----------|
| `description` | Shown in command list | Recommended |
| `agent` | Route to specific agent | No |
| `model` | Override model | No |
| `subtask` | Force subagent invocation | No |

### Command Locations

| Scope | Path |
|-------|------|
| Project | `.opencode/commands/<name>.md` |
| Global | `~/.config/opencode/commands/<name>.md` |

---

## Example Commands

### /test — Run Tests and Fix Failures

```markdown
---
description: Run tests and fix failures
---

<summary>
You MUST run the full test suite.
You SHOULD identify and fix failures.
You MUST re-verify fixes.
</summary>

<objective>
You MUST run the full test suite, find the root cause of any failures, and fix the issue.
</objective>

1. Show the failing test
2. Identify the root cause
3. Fix the issue
4. Re-run to verify
```

### /review — Code Review (Read-Only)

```markdown
---
description: Review code for issues
agent: plan
---

<summary>
You MUST review $ARGUMENTS for issues.
You SHOULD check for bugs, security, and performance.
You MUST provide actionable feedback.
</summary>

<target>
$ARGUMENTS
</target>

<objective>
You MUST review the file above for bugs, security, and performance issues and provide actionable feedback without making changes.
</objective>
```

### /commit — Smart Commit with Conventional Prefix

```markdown
---
description: Stage and commit with conventional prefix
---

<summary>
You MUST analyze changes and stage files.
You SHOULD choose a conventional commit prefix.
You MUST commit with a concise message.
</summary>

<context>
!`git status`
!`git diff`
</context>

<objective>
You MUST analyze changes, stage relevant files, and commit with a concise message.
</objective>

1. Analyze all changes
2. Choose appropriate prefix: docs:, feat:, fix:, refactor:, test:, ci:
3. Write concise commit message (imperative mood)
4. Stage relevant files and commit
```

### /spellcheck — Check Spelling in Markdown

```markdown
---
description: Check spelling in markdown files
subtask: true
---

<summary>
You MUST find unstaged markdown files.
You SHOULD check them for spelling errors.
You MUST report any found errors.
</summary>

<context>
!`git diff --name-only | grep -E '\.md$'`
</context>

<objective>
You MUST check for and report any spelling errors found in unstaged markdown files.
</objective>
```

### /issues — Search GitHub Issues

```markdown
---
description: Search GitHub issues
model: anthropic/claude-3-5-haiku-20241022
subtask: true
---

<summary>
You MUST search GitHub issues for $ARGUMENTS.
You SHOULD limit results to the top 10.
You MUST summarize the relevant issues.
</summary>

<context>
!`gh issue list --search "$ARGUMENTS" --limit 10`
</context>

<objective>
You MUST summarize the top 10 relevant GitHub issues found for $ARGUMENTS.
</objective>
```

### /component — Create React Component

```markdown
---
description: Create a new React component
---

<summary>
You MUST create a React component named $1.
You SHOULD include TypeScript props and basic styling.
You MUST provide a unit test file.
</summary>

<objective>
You MUST create a React component named $1 in $2 with TypeScript props interface, basic styling, and a unit test file.
</objective>
```

Usage: `/component UserProfile src/components`

### /migrate — Database Migration

```markdown
---
description: Generate database migration
---

<summary>
You MUST read the Prisma schema.
You SHOULD generate a migration for $ARGUMENTS.
You MUST ensure the migration matches the schema.
</summary>

<context>
@prisma/schema.prisma
</context>

<objective>
You MUST generate a Prisma migration for $ARGUMENTS that matches the provided schema.
</objective>
```

---

## Shell Commands in Templates

Use the `!cmd` syntax to inject bash output:

```markdown
Review recent changes:

!`git log --oneline -10`

Suggest improvements.
```

## File References

Use `@` to include file content:

```markdown
Given the schema in @prisma/schema.prisma, generate a migration for $ARGUMENTS.
```

---

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
