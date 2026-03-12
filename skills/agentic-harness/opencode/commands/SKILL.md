---
name: command-creator
description: |-
  Create custom /slash commands for repetitive tasks in OpenCode. Use when user wants to create a command, automate a workflow, make a shortcut, add a slash command, set up automation, create a /test, /review, /deploy, /commit, /fix, or /build command.

  Examples:
  - user: "Make a /test command that runs pytest" → create command file with test instructions
  - user: "Add a /review command for PRs" → design prompt to fetch diff and review
  - user: "Automate this repetitive task" → identify pattern, create parameterized command
  - user: "Make a /deploy command" → encode build/test/deploy workflow in command
  - user: "Create a slash command to fix lint errors" → create /fix-lint command
  - user: "I want to run my CI checks with a command" → create /ci command
---

# Command Creator

Create custom slash commands for repetitive tasks in OpenCode.

## Quick Start

Create a command file at `.opencode/command/<name>.md`:

```markdown
---
description: Run tests and fix failures
---

You MUST run the full test suite, identify any failures, and fix them.

1. Run `npm test`
2. For each failure, identify root cause and fix
3. Re-run to verify all pass
```

Then invoke it in OpenCode: `/test`

For commands that need arguments (e.g. `/review src/api.ts`):

```markdown
---
description: Review file for bugs
agent: plan
---

You MUST review $ARGUMENTS for bugs, security issues, and performance problems.
```

---

## Mental Model

**Slash commands are user-to-agent shortcuts.** They are markdown files containing an instruction template. When a user types `/test`, OpenCode loads the template, substitutes placeholders, and sends the result to the agent as a message.

Think of a command as a **reusable prompt with variables**:

- The **file name** determines the slash command name (`test.md` → `/test`)
- The **frontmatter** sets agent, model, and execution mode
- The **body** is the instruction template (with `$ARGUMENTS`, `!commands`, `@files`)
- The **agent executes** the body — it cannot invoke other slash commands

**When to use each mode:**
- No `agent:` → default build agent (can read/write files)
- `agent: plan` → read-only analysis (no file writes)
- `subtask: true` → background execution, single output (fire-and-forget)

### Commands vs Skills vs Agents

Commands, skills, and agents are complementary — each has a distinct purpose:

| Type | Triggered by | Purpose |
|------|-------------|---------|
| `/command` | User typing in terminal | Shortcut for a user workflow (test, deploy, review) |
| Skill | Agent loading it | Reusable instructions an agent follows |
| Agent | Task routing or user selection | Specialized persona for a domain |

**Rule**: Use a command when a human wants a shortcut. Use a skill when an agent needs instructions. Use an agent when specialization is needed.

---

**Command creation is conversational, not transactional.**

- MUST NOT assume what the user wants—ask
- SHOULD start simple, add complexity only if needed
- MUST show drafts and iterate based on feedback

## Question Tool

**Batching:** Use the `question` tool for 2+ related questions. Single questions → plain text.

**Syntax:** `header` ≤12 chars, `label` 1-5 words, add "(Recommended)" to default.

When to ask: Vague request ("make a command"), or multiple implementation approaches exist.

## Command Locations

| Scope   | Path                                   |
| ------- | -------------------------------------- |
| Project | `.opencode/command/<name>.md`          |
| Global  | `~/.config/opencode/command/<name>.md` |

## Template Placeholders (CRITICAL)

- **Single Insertion**: Each placeholder (`$ARGUMENTS`, `$1`, `@filename`, etc.) MUST be inserted ONLY ONCE in the command body.
- **Dedicated Blocks**: Use dedicated XML blocks for user inputs and file contents to keep the objective and instructions clean.
- **Preference**: Prefer using a single argument (`$ARGUMENTS`) over complex positional ones unless strictly necessary.

BAD: "Do $ARGUMENTS and then check $ARGUMENTS for errors."

GOOD:
```markdown
<user_guidelines>
$ARGUMENTS
</user_guidelines>

<context>
@src/schema.ts
</context>

<objective>
Analyze findings based on user guidelines and the provided schema.
</objective>
```

| Placeholder      | Description            | Example                             |
| ---------------- | ---------------------- | ----------------------------------- |
| `$ARGUMENTS`     | All arguments passed   | `/cmd foo bar` → "foo bar"          |
| `$1`, `$2`, `$3` | Positional arguments   | `/cmd foo bar` → $1="foo", $2="bar" |
| `!command`       | Shell output injection | `!ls -F`                            |
| `@filename`      | Include file content   | `@src/index.ts`                     |

## Terminology Standard

- **Direct Address**: You MUST use "you" (referring to the agent executing the command) instead of "the agent" or "opencode".

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

## Frontmatter Options

| Field         | Purpose                   | Required    |
| ------------- | ------------------------- | ----------- |
| `description` | Shown in command list     | RECOMMENDED |
| `agent`       | Route to specific agent   | No          |
| `model`       | Override model            | No          |
| `subtask`     | Force subagent invocation | No          |

## Conceptual Boundary (CRITICAL)

- **Commands are for USERS**: They are high-level shortcuts for humans to trigger an agent with specific context.
- **Agents CANNOT use commands**: Agents lack a terminal interface to "type" commands. They execute the *template body* provided by the command.
- **One-Way Street**: User -> Command -> Agent.
- **NEVER** tell an agent to use a command "proactively"—they literally cannot.
- If logic needs to be shared, put it in a **Skill** (script/instruction), not a Command.

## Shell Commands in Templates

Use the !`command` syntax to inject bash output into your prompt. The shell execution is triggered by an exclamation mark followed by the command wrapped in backticks:

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

## Phase 1: Understand the Task

1. **"What task do you want to automate?"**
   - Get the repetitive prompt/workflow
   - Examples: "run tests", "review this file", "create a component"

2. **"Can you show me how you'd normally ask for this?"**
   - Get the actual prompt they'd type
   - This becomes the template body

## Phase 2: Inputs & Routing

1. **"Does it need arguments?"**
   - If they mention "this file", "a name", "the function" → yes
   - Explain: `/command foo bar` → `$ARGUMENTS` = "foo bar", `$1` = "foo", `$2` = "bar"

2. **"Should it make changes or just analyze?"**
   - Changes → default agent (build)
   - Analysis only → set `agent: plan`

3. **"Should it run as a background subtask?"**
   - Long-running or parallel work → set `subtask: true`
   - Interactive or quick → leave unset

4. **"Project-specific or use everywhere?"**
   - Project → `.opencode/command/`
   - Global → `~/.config/opencode/command/`

## Phase 3: Review & Refine

1. **Show the draft command, ask for feedback**
   - "Here's what I've created. Want to adjust anything?"
   - Iterate until user is satisfied

**Be flexible:** If user provides lots of info upfront, adapt—MUST NOT rigidly ask every question.

## Example Commands

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

## /review - Code Review (Read-Only)

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

<objective>
You MUST review $ARGUMENTS for bugs, security, and performance issues and provide actionable feedback without making changes.
</objective>

- Bugs and edge cases
- Security issues
- Performance problems
```

## /commit - Smart Commit with Prefixes

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

## /spellcheck - Check Spelling

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

Check spelling in all unstaged markdown files:

<context>
!`git diff --name-only | grep -E '\.md$'`
</context>

<objective>
You MUST check for and report any spelling errors found in unstaged markdown files.
</objective>
```

## /issues - Search GitHub Issues

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

Search GitHub issues matching: $ARGUMENTS

<context>
!`gh issue list --search "$ARGUMENTS" --limit 10`
</context>

<objective>
You MUST summarize the top 10 relevant GitHub issues found for $ARGUMENTS.
</objective>
```

## /component - Create Component

```markdown
---
description: Create a new React component
---

<summary>
You MUST create a React component named $1.
You SHOULD include TypeScript props and basic styling.
You MUST provide a unit test file.
</summary>

Create a React component named $1 in $2 with:

<objective>
You MUST create a React component with TypeScript props and basic styling, and provide a corresponding unit test file.
</objective>

- TypeScript props interface
- Basic styling
- Unit test file
```

Usage: `/component UserProfile src/components`

## /migrate - Database Migration

```markdown
---
description: Generate database migration
---

<summary>
You MUST read the Prisma schema.
You SHOULD generate a migration for $ARGUMENTS.
You MUST ensure the migration matches the schema.
</summary>

Given the schema in @prisma/schema.prisma:

<context>
@prisma/schema.prisma
</context>

<objective>
You MUST generate a Prisma migration for $ARGUMENTS that matches the provided schema.
</objective>
```

---

## Anti-Patterns

### 1. Using `$ARGUMENTS` multiple times in one template

**WHY:** Each placeholder is inserted once. NEVER use `$ARGUMENTS` twice — it creates duplicate content and confuses the agent about the intent. ALWAYS wrap it in an XML block.

**Bad**:
```markdown
Review $ARGUMENTS for bugs. After checking $ARGUMENTS, create a report.
```

**Good**:
```markdown
<target>
$ARGUMENTS
</target>

Review the file above for bugs and create a report.
```

---

### 2. Telling an agent to "use /command" in another command

**WHY:** Agents NEVER can invoke slash commands. NEVER instruct an agent to "use /command" — commands are user-to-agent shortcuts only. Agents execute the template body and have no terminal interface.

**Bad**:
```markdown
Analyze the code, then use /test to verify your changes.
```

**Good**:
```markdown
Analyze the code, then run `npm test` to verify your changes.
```

---

### 3. Using `agent: plan` for commands that make changes

**WHY:** The `plan` agent is read-only. NEVER set `agent: plan` for commands that write files, create commits, or run destructive commands — the `plan` agent will refuse.

**Bad**:
```markdown
---
agent: plan
---
Fix all TypeScript errors in $ARGUMENTS.
```

**Good**: Omit `agent:` (default build agent) or use `agent: build` explicitly for commands that modify files.

---

### 4. Hardcoding dynamic values instead of using `!cmd`

**WHY:** Hardcoded paths, branch names, or dates are stale as soon as the project changes.

**Bad**:
```markdown
Review changes from branch feature/my-feature since last week.
```

**Good**:
```markdown
Review the following recent changes:

!`git log --oneline -10`
!`git diff HEAD~5`
```

---

### 5. Using `subtask: true` for interactive commands

**WHY:** Subtasks run as background subagents and do not show intermediate output in the main session. Commands that need back-and-forth with the user (like multi-step workflows) will appear to hang.

**Bad**:
```markdown
---
subtask: true
---
Walk me through debugging this issue step by step.
```

**Good**: Only use `subtask: true` for fire-and-forget tasks (linting, spell-checking, reporting) that produce a single final output.

---

### 6. Using `$ARGUMENTS` in `agent: plan` commands without an explicit objective

**WHY:** The `plan` agent is instructed to never write files. If your command body only says `$ARGUMENTS` with no surrounding context, the agent has no clear instruction and may refuse to act or provide a generic response.

**Bad**:
```markdown
---
agent: plan
---

$ARGUMENTS
```

**Good**:
```markdown
---
description: Analyze code quality
agent: plan
---

<target>
$ARGUMENTS
</target>

<objective>
You MUST analyze the code in the target above for bugs, security vulnerabilities, and performance issues. Provide a prioritized, actionable report.
</objective>
```

---

### 7. Expecting commands to chain or pipeline into each other

**WHY:** Commands are one-shot prompts. NEVER expect `/command-a` to invoke `/command-b`. There is no mechanism to chain commands. ALWAYS encode all steps in a single command body if a multi-step workflow is needed.

**Bad**: Creating separate `/lint` and `/test` commands and trying to call them in sequence via another command.

**Good**:
```markdown
---
description: Lint then test
---

1. Run `npm run lint` and fix any errors
2. Run `npm test` and fix any failures
3. Report the final status of both
```

---

### 8. Placing the command file at the project root instead of `.opencode/command/`

**WHY:** OpenCode NEVER discovers command files outside `.opencode/command/` or `~/.config/opencode/command/`. NEVER place command files at the project root — they are silently ignored.

**Bad**:
```
my-project/
  test.md          ← Not discovered
  review.md        ← Not discovered
```

**Good**:
```
my-project/
  .opencode/command/
    test.md        ← Discovered as /test
    review.md      ← Discovered as /review
```

---

### 9. Using `subtask: true` with commands that need multiple back-and-forth steps or file changes

**WHY:** Subtask agents return a single output and do not stream intermediate steps. Commands that need to make multiple file changes, run iterative fixes, or ask follow-up questions will silently complete after the first step without showing progress.

**Bad**:
```markdown
---
subtask: true
---
Refactor the entire auth module to use the new provider pattern.
```

**Good**: Only use `subtask: true` for atomic, read-only, or single-output tasks:
```markdown
---
description: Check markdown spelling
subtask: true
model: anthropic/claude-3-5-haiku-20241022
---
Check all `.md` files changed in the last commit for spelling errors and report them.
```

---

| Do                                    | Don't                                 |
| ------------------------------------- | ------------------------------------- |
| Keep templates focused                | Cram multiple tasks in one            |
| Use `$1`, `$2` for structured args    | Rely only on $ARGUMENTS for multi-arg |
| Use !`cmd` to gather context          | Hardcode dynamic values               |
| Use `@file` for config/schema refs    | Copy-paste file contents              |
| Set `agent: plan` for read-only       | Forget agent for reviews              |
| Set `subtask: true` for parallel work | Block main session unnecessarily      |

## References

- [command-patterns.md](references/command-patterns.md) - Pattern library: analysis, fire-and-forget, contextual, multi-argument, scope-limited
- [frontmatter-reference.md](references/frontmatter-reference.md) - Complete frontmatter fields and placeholder reference

## Eval Scenarios

- [Scenario 0: Create /deploy command with arguments and dynamic context](evals/scenario-0/task.md)
- [Scenario 1: Create /spellcheck fire-and-forget command](evals/scenario-1/task.md)
- [Scenario 2: Identify and fix anti-patterns in a broken command](evals/scenario-2/task.md)
