---
name: vault-capture
description: >
  Captures an important insight, decision, constraint, pattern, or discovery to
  persistent memory via the vault CLI — distinct from general note-taking in that
  entries are tagged, tiered, and retrievable across future sessions. Use when the
  user says "save this", "remember this", "note for later", "don't forget", or
  "keep this in mind", or when the conversation produces something worth
  preserving: architectural decisions, recurring bugs, hard-won constraints,
  naming conventions, or user preferences.
allowed-tools: "Bash"
---

# vault-capture

Persist a memory to the vault so it can be retrieved in future sessions.

## Mindset

Before capturing, answer two questions:

1. **Is this time-bound?** If yes → `episodic`. If no, continue.
2. **Is this a how-to process?** If yes → `procedural`. If no → `semantic`.

Always capture with `--project` when working in a project context. Always add
`--tags` with at least two relevant keywords — they are the primary BM25 signal
for future retrieval.

## When to use

Use whenever the conversation produces:
- A confirmed decision ("we're using X instead of Y")
- A constraint that must not be violated
- A recurring bug or workaround
- A project convention or naming pattern
- A user preference expressed explicitly

## How to use

```bash
vault-cli capture --text "<memory content>" [--tier episodic|semantic|procedural] [--project <id>] [--tags tag1,tag2]
```

Always double-quote the `--text` value to handle apostrophes and special characters.

Or pipe from stdin:
```bash
echo "<memory content>" | vault-cli capture
```

## Tier selection

| Question | Answer | Tier |
|---|---|---|
| Is this tied to today's session or a specific event? | Yes | `episodic` (default) |
| Is this a rule, fact, or convention that should persist indefinitely? | Yes | `semantic` |
| Is this a step-by-step process or how-to? | Yes | `procedural` |

## Examples

```bash
vault-cli capture --text "Decided to use Bun instead of Node for all scripts. Reason: workspace protocol support." --tier semantic --tags bun,tooling --project vault-core

vault-cli capture --text "Never use --force-push on main. CI enforces branch protection." --tier procedural --tags git,ci

vault-cli capture --text "Auth service fails silently when DB not ready — add depends_on with condition: service_healthy in docker-compose." --tier episodic --tags docker,auth,bug --project api
```

## Never

- **Never omit `--tier`** for a rule or convention — defaulting to `episodic` means it will decay and be consolidated away
- **Never capture without `--tags`** — untagged memories score poorly in BM25 retrieval
- **Never omit `--project`** when working inside a project — global memories pollute cross-project searches
- **Never use single quotes around `--text`** — apostrophes in content will break the shell command
