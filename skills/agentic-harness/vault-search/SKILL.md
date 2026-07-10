---
name: vault-search
description: >
  Search persistent memory for relevant context — including architectural
  decisions, resolved bugs, and implementation patterns — when the user asks
  about past decisions, when debugging a recurring issue, when uncertain whether
  something was already decided, or before re-implementing something that may
  have been done before. Use when the user says "do you remember…", "what did
  we decide about…", or "have we solved this before".
allowed-tools: "Bash"
---

# vault-search

Retrieve relevant memories from the vault using hybrid BM25 + vector search.

## Mindset

Search before you implement, design, or debug — not after. A 2-second search
that surfaces a prior constraint saves hours of work in the wrong direction.
When search returns empty, say so honestly; never fabricate a memory.

## When to use

- Before starting a non-trivial task: search for related prior decisions
- When the user asks "do you remember…" or "what did we decide about…"
- Before proposing an approach: check for existing constraints or patterns
- When a bug appears familiar: search for past workarounds

## How to use

```bash
vault-cli search "<query>" [--top-k <n>] [--project <id>]
```

Exits 1 with "No results." if nothing is found.

## Query construction

- Use **noun phrases**, not questions: `"auth token expiry"` not `"how does auth token expiry work?"`
- Use `--project` to narrow scope when working in a known project context
- If the first query returns nothing, broaden: remove specific version numbers or
  error codes and retry with a higher-level term (e.g. `"postgres connection"` instead of `"ECONNREFUSED 5432"`)

## Handling no results

If the search exits with "No results.":
1. Retry with a broader synonym query
2. If still empty, tell the user no prior memory exists on this topic
3. Never invent or guess at a prior decision — proceed fresh and offer to capture the new decision

## Examples

```bash
vault-cli search "authentication approach"
vault-cli search "database migration strategy" --project my-app
vault-cli search "recurring TypeScript error" --top-k 3
vault-cli search "postgres connection refused" --project api
```

## Output format

Each result shows:
```
1. [category] summary (date)
   tier: episodic | strength: 0.87 | score: 0.0312
   content preview...
```

## Never

- **Never skip search before a non-trivial implementation task** — prior constraints may invalidate your approach before you write a line
- **Never fabricate a memory** when search returns empty — report "No results found" and proceed without invented context
- **Never search with a full sentence or question** — BM25 scores individual terms; verbose queries dilute signal
