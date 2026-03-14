---
name: opencode-design-commands
description: "Create custom /slash commands for repetitive tasks in OpenCode. Use when user wants to create a command, automate a workflow, make a shortcut, add a slash command, set up automation, create a /test, /review, /deploy, /commit, /fix, /build, /lint, /spellcheck, /migrate, or /ci command. Also use when writing a command file, configuring command frontmatter, using $ARGUMENTS placeholders, setting agent: plan, using subtask: true, injecting shell output with !cmd, referencing files with the at-file syntax in a command template, or choosing between a slash command, agent, and skill."
---

# Command Creator

## Quick Start

Create `.opencode/commands/<name>.md`:

```markdown
---
description: Run tests and fix all failures automatically
---

Run the full test suite, identify any failures, and fix them.

1. Run `npm test`
2. For each failure, identify root cause and fix
3. Re-run to verify all pass
```

Invoke it in OpenCode: `/test`

Run `bun run opencode` to start, then type `/<name>` to invoke your command.

## Mindset

**Slash commands are user-to-agent shortcuts** — markdown files with an instruction template. User types `/test` → OpenCode substitutes placeholders → agent executes.

**Decision rule**:
- Human wants a shortcut → **Command**
- Agent needs standing instructions → **Skill**
- Task needs a persona → **Agent**

**When to use**: Repetitive multi-step tasks that need a one-word trigger.
**When NOT to use**: Do not create a command for one-off tasks — just type the instruction directly. Do not use commands for interactive workflows; commands are one-shot.
Modes: no `agent:` = default build (read/write); `agent: plan` = read-only; `subtask: true` = fire-and-forget.

## Template Placeholders

`$ARGUMENTS` — all args; `$1`/`$2` — positional; `!cmd` — shell output injection; `@filename` — include file content.

```bash
# Dynamic context injection example
!git log --oneline -5
```

```markdown
---
description: Deploy to staging with context
agent: plan
subtask: true
---
Deploy $ARGUMENTS using the steps in docs/deploy.md.
!git log --oneline -3
```

## Anti-Patterns

NEVER use `$ARGUMENTS` more than once in a command body. WHY: The placeholder is substituted literally — using it twice creates duplicate content that confuses the agent.
```markdown
# BAD — duplicate content
Review $ARGUMENTS. Focus on $ARGUMENTS for security issues.

# GOOD — wrap in XML once
<target>$ARGUMENTS</target>
Review the target file for security issues and bugs.
```

NEVER tell the agent to "run /command" inside a command body. WHY: Agents cannot invoke slash commands from within a command — only users can type `/cmd`.
```markdown
# BAD
After fixing, run /test to verify.

# GOOD
After fixing, run `npm test` to verify.
```

NEVER use `agent: plan` for commands that write files. WHY: The plan agent is read-only and will refuse any write operation, silently failing the command.
```markdown
# BAD
---
agent: plan
---
Create a new file called output.md with the results.
# GOOD — remove agent: plan for write operations
```

## References

- [`references/command-patterns.md`](references/command-patterns.md) — Creation phases, example commands (/test, /review, /commit, /deploy, /spellcheck), pattern library
- [`references/frontmatter-reference.md`](references/frontmatter-reference.md) — Complete frontmatter fields and placeholder reference

**Eval Scenarios**: [Scenario 0: Deploy](evals/scenario-0/task.md) | [Scenario 1: Spellcheck](evals/scenario-1/task.md) | [Scenario 2: Fix anti-patterns](evals/scenario-2/task.md)
