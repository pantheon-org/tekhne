# Frontmatter and Placeholder Reference

Complete reference for command file frontmatter fields and template placeholders.

## Frontmatter Fields

```yaml
---
description: "Short summary shown in /help list"  # RECOMMENDED
agent: build                   # Optional: build | plan | <custom-agent-name>
model: provider/model-id       # Optional: override default model
subtask: true                  # Optional: run as background subagent
---
```

### `description`

Shown in the `/help` command list. Write as a 3-7 word imperative phrase.

Good: `Run tests and fix failures`
Bad: `This command runs tests`

### `agent`

Routes the command to a specific agent:

| Value | Behavior |
|-------|----------|
| (omitted) | Default build agent — can read, write, execute |
| `build` | Explicit build agent — same as default |
| `plan` | Read-only analysis agent — cannot modify files |
| `<name>` | Any named agent from `.opencode/agent/<name>.md` |

**Rule**: Use `agent: plan` whenever the command should never write files.

### `model`

Overrides the default model for this command only.

```yaml
model: anthropic/claude-3-5-haiku-20241022   # Fast, cheap — for simple tasks
model: anthropic/claude-sonnet-4-5-20250514  # Balanced
model: openai/gpt-4o                          # Alternative provider
```

Use a cheaper/faster model for subtasks that don't need full reasoning.

### `subtask`

When `true`, the command runs as a background subagent:
- Does NOT show intermediate messages
- Returns a single final output
- Cannot interact with the user during execution

Use for: lint checks, spell checks, coverage reports, search tasks.
Do NOT use for: step-by-step workflows, anything requiring user input.

---

## Template Placeholders

### `$ARGUMENTS`

Everything passed after the command name, as a single string.

```
/review src/api.ts --depth=deep
```
→ `$ARGUMENTS` = `"src/api.ts --depth=deep"`

**Rules:**
- Use ONCE per template (not repeated)
- Wrap in an XML block to keep objective clean
- Use when argument structure doesn't matter

### `$1`, `$2`, `$3`

Positional arguments split by whitespace.

```
/migrate UserCard class-component
```
→ `$1` = `"UserCard"`, `$2` = `"class-component"`

**Use when:** You need to distinguish two independent inputs (e.g., source and target).

### `!cmd`

Injects the output of a shell command at template load time.

```markdown
!`git log --oneline -20`
!`git status`
!`cat package.json | jq .dependencies`
```

**Rules:**
- Backtick-wrapped command: `` !`command` ``
- Executes in the current project directory
- Output is injected inline before the agent sees the prompt
- Use for: git state, file listings, environment info, build output

### `@filename`

Injects the full content of a file.

```markdown
@src/schema.ts
@prisma/schema.prisma
@.env.example
```

**Rules:**
- Path relative to project root
- Content injected inline
- Use for: schemas, configs, reference files the agent needs

---

## XML Block Conventions

The recommended template structure uses XML blocks to organize content:

| Block | Purpose |
|-------|---------|
| `<summary>` | 3-line MUST/SHOULD/MUST summary of intent |
| `<context>` | Dynamic data injected via `!cmd` or `@file` |
| `<target>` | User-provided input (`$ARGUMENTS`, `$1`, `$2`) |
| `<objective>` | Explicit success condition for the agent |
| `<constraints>` | What NOT to do (optional) |

### Template Skeleton

```markdown
---
description: Do the thing
---

<summary>
You MUST [primary action].
You SHOULD [secondary action].
You MUST [success condition].
</summary>

<context>
!`some-command`
@some/file.ts
</context>

<target>
$ARGUMENTS
</target>

<objective>
You MUST accomplish [goal] using the context and target above.
</objective>
```
