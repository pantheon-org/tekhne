---
name: opencode-design-agents
description: "Create and refine OpenCode agents via guided Q&A. Use when creating an agent file, improving an existing agent, configuring AGENTS.md frontmatter, setting subagent_type, defining allowed-tools, restricting permissions, designing agent routing, writing a system prompt, configuring mode: subagent, setting a permission block, or working with the Task tool delegation pattern. Examples: 'Create an agent for code reviews', 'My agent ignores context', 'Add a database expert subagent', 'Make my agent faster', 'Agent triggers on wrong requests'."
---

# Agent Architect

Create and refine OpenCode agents through a guided Q&A process.

## Quick Start

Create `.opencode/agents/<name>.md`:

```markdown
---
description: Code review specialist. Use when user says "review", "check my code", "find bugs".
---

You are an expert code reviewer. Analyze code for bugs, security issues, and performance problems.
Provide specific, actionable feedback with line references.
```

For subagents (Task tool only), add `mode: subagent`. For permission restrictions, add a `permission:` block:

```yaml
---
mode: subagent
permission:
  bash:
    "*": "ask"
    "npm test": "allow"
---
```

Run `bun run opencode` and type `/agents` to confirm the new agent appears in the list.

## Mindset

**An agent is a markdown file with YAML frontmatter.** Three decisions drive every design:

| Decision | Why it matters |
|----------|----------------|
| **Scope** (`description`) | Controls when the agent triggers — too vague = never fires, too broad = fires on everything |
| **Mode** (`mode:`) | `all` = visible everywhere; `subagent` = Task-tool only; `primary` = main list only |
| **Permissions** (`permission:`) | Standard tools need no config; skills ALWAYS need explicit allowlists |

**When to use**: You need a distinct scope, persona, or permission set that differs from the default.

**When NOT to use**: Do not create an agent just to run a single command — use a slash command instead. Do not create a subagent when you just need a custom system prompt — use `AGENTS.md`.

## Creation Workflow

Ask questions first. Follow phases in [`references/agent-patterns.md`](references/agent-patterns.md):

1. **Phase 1 — Core Purpose**: What does it do? What triggers it? What persona?
2. **Phase 1.5 — Research**: Look up current best practices for the domain.
3. **Phase 2 — Capabilities**: Permissions, skills, mode (subagent vs primary vs all).
4. **Phase 3 — Review**: Show draft, iterate until user approves.

## Anti-Patterns

NEVER use `bash: { "*": "allow" }`. WHY: Allows destructive commands like `rm -rf`. Use `"*": "ask"` plus an explicit allowlist.
```yaml
# BAD
permission:
  bash:
    "*": "allow"
# GOOD
permission:
  bash:
    "*": "ask"
    "npm test": "allow"
    "git status": "allow"
```

NEVER write system prompts in third person. WHY: The model reads its own system prompt — third person is confusing and degrades instruction-following.
```markdown
# BAD — "The assistant analyzes code..."
# GOOD — "You analyze code for bugs, security issues, and performance problems."
```

NEVER add `permission:` block for standard tools. WHY: It creates noise and can accidentally restrict access to built-in capabilities. Only use `permission:` when you need to restrict or explicitly allow skills.

| Anti-Pattern | Fix |
|---|---|
| Vague `description` | Add concrete trigger examples |
| Missing `mode: subagent` on subagents | Set it explicitly |
| `skill: { "my-skill": "allow" }` without `"*": "deny"` | Deny `"*"` first |

## Eval Scenarios

- [Scenario 0: Create specialized agent with permissions](evals/scenario-0/task.md)
- [Scenario 1: Configure skill permission allowlist](evals/scenario-1/task.md)
- [Scenario 2: Fix broken agent triggers and system prompt](evals/scenario-2/task.md)

## References

- [`references/agent-patterns.md`](references/agent-patterns.md) — Creation phases, patterns, prompt engineering, enhancement/troubleshooting
- [`references/opencode-config.md`](references/opencode-config.md) — Full frontmatter schema, tools, permissions reference
