---
name: rules-management
description: "Manage project-level agent behavioural rules recorded in a single rules file. Use when the user says 'new rule', 'add a rule', 'record that', 'make a rule that', 'note this', 'take note of', 'new instruction', or asks to codify an agent directive. Read the rules file to check existing rules before adding a new one. DO NOT use for ephemeral notes, one-off instructions, or instructions that belong in the project's own agent-facing docs. Triggers: 'new rule', 'add a rule', 'record that', 'make a rule', 'codify this', 'rule about', 'create a directive', 'note this', 'take note of', 'new instruction'."
---

# Rules Management

Manage project-level agent behavioural rules.

## Prerequisites

- The rules file (path shown in Quick Start below) must exist in the repository root. If it is missing, create it with a `# Agent Rules` heading first.
- Only invoke this skill when the user explicitly requests a new rule -- do not infer rule creation from unrelated conversation.
- Depends on the user providing a clear behavioural directive; if the instruction is vague, ask for clarification before writing.
- The rule schema (`assets/schemas/rule-entry.schema.json`) and template (`assets/templates/rule-entry.yaml`) define the structured title/directive/rationale shape a rule entry MUST have.

## Quick Start

```bash
# Read current rules
cat .claude/RULES.md

# Validate the rules file against the schema before or after editing it
./scripts/validate-rules.sh validate .claude/RULES.md

# Generate a correctly-formatted entry from the template instead of hand-writing one
./scripts/validate-rules.sh generate "<title>" "<directive>" "<rationale>"
```

→ Produces the full current ruleset for duplicate checking, and a schema-validated way to both read and write entries.

## Where Rules Live

All agent rules reside in a single file (path shown in Quick Start above). This is the authoritative source that all agents read before acting in this repository.

## When to Use

Create a new rule whenever the user says something like:

- "New rule: ..."
- "Add a rule that ..."
- "Record that ..."
- "Make a rule about ..."
- "Codify this as a rule"
- "Note this" / "Take note of ..."
- "New instruction: ..."

The user's instruction becomes the rule body. You MUST:

1. Read the existing rules file first (see Quick Start) -- check every `### Rule:` heading to avoid duplicates
2. If a rule with the same directive already exists, inform the user and do NOT create a duplicate
3. Append the new rule following the format below -- PREFER `scripts/validate-rules.sh generate` over hand-typing the three-line block
4. Inform the user you've added it
5. Confirm by running `scripts/validate-rules.sh validate` before treating the change as done -- TYPICALLY this is the same command as the Quick Start validation step

## Rule Format

Every rule entry MUST include:

```markdown
### Rule: <short imperative title>

**Directive:** <clear actionable instruction -- prefer ALWAYS/NEVER phrasing>

**Rationale:** <why this rule exists -- one or two sentences>
```

After appending, run `scripts/validate-rules.sh validate` to verify the entry appears correctly and the file still has valid structure -- RECOMMENDED over eyeballing a raw `cat` of the file, since the script checks against the schema instead of just human judgment.

## When NOT to Use

- Ephemeral notes or one-off instructions -- use a context file or tell the user directly
- Instructions that belong in AGENTS.md (repo map, workflow, tool conventions)
- Personal preferences that don't affect agent behaviour
- "Remember this" or "save this for later" -- route to vault-capture instead

## Anti-Patterns

**NEVER** add a rule without reading the existing rules file first.
**WHY:** Blind appending creates duplicate or conflicting directives, violating the single-source-of-truth contract and confusing future agents.
**SYMPTOM:** Two rules with near-identical directives, or a new rule that quietly contradicts one added months earlier.
**BAD:**

```markdown
readFile(rulesFile, append=true)
```

**GOOD:**

```markdown
rules = readFile(rulesFile)
if hasDuplicate(rules, userDirective):
    informUser("This rule already exists")
else:
    append(rules, newEntry)
```

**CONSEQUENCE:** The rules file grows contradictory or redundant entries, and future agents cannot tell which directive is authoritative.

**NEVER** accept a vague user instruction as the rule body.
**WHY:** A rule like "use good logging" is not actionable. Every rule must have a precise ALWAYS/NEVER directive and a rationale.
**BAD:**

```markdown
- Don't use bad logging
```

**GOOD:**

```markdown
### Rule: Always use structured logging

**Directive:** ALWAYS use structured logging (JSON) for production services.

**Rationale:** Structured logs enable aggregation, search, and alerting.
```

**CONSEQUENCE:** An unactionable rule cannot be checked or enforced, so it is silently ignored the first time it is inconvenient.

**NEVER** create a rule for memory or ephemeral notes.
**WHY:** Rules are behavioural directives binding all agents. Ephemeral notes belong in a context file or vault, not in the rules file.
**BAD:** Appending "remember we tried Postgres once and it was slow" as a rule entry.
**GOOD:** Route it to `vault-capture` or a dated context finding instead, and only add a rule if it implies a binding, checkable directive.
**CONSEQUENCE:** The rules file grows unbounded with noise, reducing signal-to-noise ratio and causing agents to skip reading it.

## Mindset

- Rules are binding for all agents in this repository -- be precise and unambiguous
- Prefer ALWAYS/NEVER phrasing for directives (e.g., "NEVER recommend a dependency without checking the registry")
- Always include a rationale so future readers understand why the rule exists
- Read existing rules before adding -- no duplicates
- Rules should be few and high-signal -- avoid documenting the obvious
- If the user asks for a rule that overlaps with an existing one, suggest amending rather than duplicating
- PREFER merging related rules into a single entry TYPICALLY over scattering them across multiple entries

## References

- [`rule-format.md`](references/rule-format.md) -- the three-part Rule/Directive/Rationale structure, plus title, directive, and rationale conventions
- [`rule-entry.schema.json`](assets/schemas/rule-entry.schema.json) -- the JSON Schema enforcing the structured shape of a rule entry
- [`rule-entry.yaml`](assets/templates/rule-entry.yaml) -- the template consumed by `scripts/validate-rules.sh generate`
