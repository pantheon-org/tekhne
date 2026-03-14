# Agent Design Patterns

## Creation Phases

### Phase 1: Core Purpose (Required)

Ask these first — they shape everything else:

1. **"What should this agent do?"** — Get the core task/domain
2. **"What should trigger this agent?"** — Specific phrases, contexts, file types → becomes `description`
3. **"What expertise/persona should it have?"** — Tone, boundaries, specialization → shapes system prompt

### Phase 1.5: Research the Domain

**MUST NOT assume knowledge is current.** After understanding the broad strokes:

- Search for current best practices in the domain
- Check for updates to frameworks, tools, or APIs the agent will work with
- Look up documentation for any unfamiliar technologies mentioned
- Find examples of how experts approach similar tasks

**Example:** User wants an agent for "Next.js deployments" → Research current deployment patterns, Vercel vs self-hosted, App Router vs Pages Router, common pitfalls, etc.

### Phase 2: Capabilities

1. **"What permissions does this agent need?"**
   - **Standard**: Omit `permission` block entirely — rely on system defaults
   - **Read-Only**: Explicitly deny `write`, `edit`, `bash`
   - **Full Access**: Allow `bash: "*"` only if needed
   - **Custom**: Ask follow-ups

2. **"Should this agent use any skills?"**
   - If yes: ALWAYS configure `permission.skill` with `"*": "deny"` and explicit allows

3. **"Is this a subagent?"**
   - If yes: set `mode: subagent`
   - If no: omit `mode` (defaults to `all`) or set `mode: primary` for main-list only

### Phase 3: Details (Optional — user MAY skip)

- Specific model preference
- Custom temperature/sampling
- Maximum steps before stopping

### Phase 4: Review & Refine

1. Show the draft config and prompt, ask for feedback
2. Iterate until user is satisfied

**Key principle:** Start broad, get specific only where the user shows interest.
**Be flexible:** If the user provides lots of info upfront, skip phases that are already answered.

---

## Permission Configuration

### CRITICAL: Avoid Overengineering

- Do NOT list permissions for standard tools (`read`, `write`, `edit`, `bash`) unless the user requests restrictions or non-standard access
- Rely on system defaults for most agents
- **Skills are the exception**: ALWAYS configure `permission.skill` to whitelist specific skills

```yaml
# Standard Agent (minimal config)
permission:
  skill:
    "*": "deny"
    "my-skill": "allow"

# Restricted Agent (explicit config)
permission:
  edit: "ask"
  bash:
    "*": "deny"
  skill:
    "*": "deny"
```

### Agent Locations

| Scope | Path |
|-------|------|
| Project | `.opencode/agents/<name>.md` |
| Global | `~/.config/opencode/agents/<name>.md` |

---

## Common Agent Patterns

Use these as starting points. Each maps a common use case to the right configuration choices.

### Read-Only Analyst

Use when the agent should inspect code but NEVER modify files:

```yaml
---
description: Code quality analyst. Use when user says "review", "analyze", "check my code".
mode: all
permission:
  edit: "deny"
  bash: "deny"
---
You analyze code for quality, security, and performance issues.
You NEVER modify files directly. Provide specific, actionable findings.
```

### Background Subagent

Use for specialized tasks invoked by other agents via the Task tool:

```yaml
---
description: Test runner specialist. Use for parallel test execution during CI workflows.
mode: subagent
permission:
  bash:
    "*": "ask"
    "npm test": "allow"
    "npx jest": "allow"
  skill:
    "*": "deny"
---
You run test suites and report results. Focus on speed and clear failure summaries.
```

### Skill-Gated Expert

Use when the agent should only access specific skills:

```yaml
---
description: |-
  Database migration expert. Use when user says "write migration", "alter table",
  "create schema", "database change".
  Examples:
  - user: "add column" -> generate migration
  - user: "index performance" -> suggest index migration
permission:
  skill:
    "*": "deny"
    "supabase-postgres-best-practices": "allow"
    "conventional-commits": "allow"
---
You design safe, reversible database migrations following best practices.
```

### Deployment Gatekeeper

Use when the agent must run shell commands but only specific, safe ones:

```yaml
---
description: |-
  Deployment helper. Use when user says "deploy to staging", "push to prod", "release".
  Examples:
  - user: "deploy" -> run deployment pipeline
  - user: "rollback" -> revert last deployment
mode: subagent
permission:
  bash:
    "*": "deny"
    "git tag *": "allow"
    "npm run build": "allow"
    "npm run deploy:staging": "allow"
    "npm run deploy:production": "ask"
  skill:
    "*": "deny"
---
You handle deployments safely. Always confirm before production deploys.
```

---

## Prompt Engineering

### Recommended System Prompt Structure

```markdown
# Role and Objective
[Agent purpose and scope]

# Instructions
- Core behavioral rules
- What to always/never do

## Sub-instructions (optional)
More detailed guidance for specific areas.

# Workflow
1. First, [step]
2. Then, [step]
3. Finally, [step]

# Output Format
Specify exact format expected.

# Examples (optional)
<examples>
<example>
<input>User request</input>
<output>Expected response</output>
</example>
</examples>
```

### XML Tags (Recommended)

XML tags improve clarity and parseability across all models:

| Tag | Purpose |
|-----|---------|
| `<instructions>` | Core behavioral rules |
| `<context>` | Background information |
| `<examples>` | Few-shot demonstrations |
| `<thinking>` | Chain-of-thought reasoning |
| `<output>` | Final response format |

```xml
<instructions>
1. Analyze the code in <code> tags
2. List issues in <findings> tags
3. Suggest fixes in <recommendations> tags
</instructions>
```

### Description Field (Critical)

The `description` determines when the agent triggers.

- **Primary Agents**: Keep extremely concise (3 words max). User selects manually.
- **Any Other Agents**: Must be specific and exhaustive for correct routing.

**Template:** `[Role/Action]. Use when [triggers]. Examples: - user: "trigger" -> action`

### Prompt Altitude

Find the balance between too rigid and too vague:

| Too Rigid | Right Altitude | Too Vague |
|-----------|----------------|-----------|
| Hardcoded if-else logic | Clear heuristics + flexibility | "Be helpful" |
| "If X then always Y" | "Generally prefer X, but use judgment" | No guidance |

### Agentic Reminders

For agents that use tools in a loop, SHOULD include:

```text
# Tool Usage
If unsure about something, use tools to gather information.
Do NOT guess or make up answers.

# Planning (optional)
Think step-by-step before each action. Reflect on results before proceeding.
```

---

## Single-Agent Patterns

| Pattern | How it works | Best for |
|---------|--------------|----------|
| ReAct | Reason → Act → Observe → Repeat | Tool use with reasoning |
| Self-Refine | Generate → Critique → Refine | Quality-focused (writing, review) |
| Reflexion | Reflect on failures → Improve | Adaptive/learning tasks |

## Multi-Agent Patterns

| Pattern | How it works | Best for |
|---------|--------------|----------|
| Lead Agent | Orchestrator delegates to specialists | Multiple expertise domains |
| Router | Analyze request → Route to right agent | Diverse request types |
| Subagents | Main agent spawns focused sub-agents | Long-horizon tasks, parallel exploration |

---

## Enhancement & Troubleshooting

1. **"What's not working well?"** — Get specific symptoms
2. **"Can you show me an example where it failed?"** — Understand the gap
3. **"What should it have done instead?"** — Define success

Then propose targeted fixes:

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| Triggers too often | Description too broad | Add specific contexts |
| Misses triggers | Description too narrow | Add trigger phrases |
| Wrong outputs | Prompt ambiguous | Add explicit instructions |
| Executes dangerous commands | Loose bash permissions | Restrict with patterns |
| Uses wrong skills | No skill restrictions | Configure `permission.skill` |

MUST show proposed changes and ask for confirmation before applying.

---

## Quality Checklist

Before showing the final agent to the user:

- [ ] Asked about core purpose and triggers
- [ ] Researched the domain (MUST NOT assume knowledge is current)
- [ ] `description` has concrete trigger examples
- [ ] `mode` discussed and set appropriately
- [ ] System prompt uses second person
- [ ] Asked about tool/permission needs (MUST NOT assume)
- [ ] Output format is specified if relevant
- [ ] Showed draft to user and got feedback
- [ ] User confirmed they're happy with result

---

## Legacy Configuration

Agents may occasionally use outdated frontmatter (e.g., `tools:`, `maxSteps:`). Correct these to modern `permission:` and `steps:` fields when encountered.
