---
name: graphthulhu
description: Use and maintain the graphthulhu knowledge graph MCP to store investigations, architectural decisions, plans, and session context in a persistent Obsidian-backed vault; triggers automatically at session start, after analysis or investigations, when recording decisions, and when the user corrects stored knowledge; use when asked to remember this, save for later, take notes, persist findings, track decisions, log results, maintain context between sessions, or store knowledge.
allowed-tools: mcp__graphthulhu__health, mcp__graphthulhu__graph_overview, mcp__graphthulhu__list_pages, mcp__graphthulhu__get_page, mcp__graphthulhu__search, mcp__graphthulhu__create_page, mcp__graphthulhu__upsert_blocks, mcp__graphthulhu__append_blocks, mcp__graphthulhu__update_block, mcp__graphthulhu__delete_block, mcp__graphthulhu__link_pages, mcp__graphthulhu__find_connections, mcp__graphthulhu__find_by_tag, mcp__graphthulhu__traverse, mcp__graphthulhu__get_links, mcp__graphthulhu__decision_create, mcp__graphthulhu__decision_check, mcp__graphthulhu__decision_defer, mcp__graphthulhu__decision_resolve, mcp__graphthulhu__bulk_update_properties, mcp__graphthulhu__query_properties, mcp__graphthulhu__knowledge_gaps, mcp__graphthulhu__topic_clusters, mcp__graphthulhu__list_orphans, mcp__graphthulhu__journal_range, mcp__graphthulhu__journal_search, mcp__graphthulhu__graph_overview, mcp__graphthulhu__analysis_health, mcp__graphthulhu__reload, mcp__graphthulhu__rename_page, mcp__graphthulhu__delete_page, mcp__graphthulhu__move_block
---

# graphthulhu Knowledge Graph

Navigation hub for storing and retrieving persistent knowledge across sessions using the graphthulhu MCP.

## When to Use

Trigger automatically — without being asked — in these situations:

- **Session start**: orient by calling `graph_overview` or `list_pages` to surface relevant prior context.
- **After any investigation or analysis**: save findings as a page in the vault.
- **Architectural decisions**: use `decision_create` to record the question, options, and outcome.
- **Tooling or environment changes**: log new tools, configs, or version pins.
- **When the user corrects something**: update or remove the incorrect page immediately before continuing.
- **Research results**: store non-obvious facts that will matter next session.

## When Not to Use

- Trivial one-liner edits with no durable insight.
- Temporary debugging state that won't matter next session.
- Information that duplicates or contradicts the project's CLAUDE.md.
- Speculative conclusions drawn from reading a single file — verify first.

## Principles

- Write to build on, not to repeat. Check if a page exists before creating a new one.
- Organize by topic namespace, not chronologically.
- Keep pages focused. Long prose belongs in blocks; summaries belong in page properties.
- Link pages to make knowledge traversable, not isolated.

## Namespace Conventions

| Namespace | Purpose |
|---|---|
| `audits/` | Environment, tooling, dependency audits |
| `plans/` | Implementation plans |
| `analysis/` | Code and architecture analysis |
| `investigations/` | Exploratory research |
| `features/` | Feature specs |
| `remediation/` | Fix plans and outcomes |

Name pages as `namespace/topic` (e.g. `analysis/auth-flow`, `plans/migration-v2`).

## Workflow

For detailed steps, see [references/workflow.md](references/workflow.md).

1. **Orient** (session start): `graph_overview` → `search` → `list_pages`
2. **Retrieve**: `search` or `get_page` → `find_connections` for relationships
3. **Write** (after analysis): check exists → `create_page(namespace/topic)` → `upsert_blocks` → `link_pages`
4. **Decisions**: `decision_create` → `decision_resolve` when outcome is known
5. **Corrections**: `search` → `update_block` or `delete_page` → confirm saved before continuing

## Quick Commands

### Orient at session start

```bash
graph_overview → list_pages(namespace="plans") → search(query="<current topic>")
```

### Save investigation findings

```bash
search(query="<topic>") → [if missing] create_page(name="investigations/<topic>") → upsert_blocks(page, blocks) → link_pages
```

### Record a decision

```bash
decision_create(page="<context-page>", question="...", options=[...], deadline="YYYY-MM-DD")
# later:
decision_resolve(uuid, chosen, rationale)
```

### Find gaps in the knowledge graph

```bash
knowledge_gaps → list_orphans
```

## Anti-Patterns

### NEVER create a duplicate page without checking first

WHY: duplicate pages fragment knowledge and make retrieval unreliable.
BAD: call `create_page("analysis/auth")` without searching for existing auth analysis pages.
GOOD: call `search("auth")` and `get_page("analysis/auth-flow")` before deciding to create.

### NEVER write speculative or unverified content

WHY: wrong knowledge in the graph is harder to correct than no knowledge.
BAD: save a conclusion drawn from one file without verifying it against other sources.
GOOD: verify findings, then write with a note on confidence level if appropriate.

### NEVER skip corrections when the user provides them

WHY: repeating an error in future sessions erodes trust.
BAD: acknowledge a correction verbally and move on without updating the vault.
GOOD: call `update_block` or `delete_page` immediately, then continue.

### NEVER use flat page names without a namespace

WHY: flat names collide and make `list_pages` filtering useless.
BAD: `create_page("migration")`.
GOOD: `create_page("plans/migration-v2")`.

### NEVER over-document trivial changes

WHY: low-signal pages dilute retrieval quality.
BAD: create a page for every single file edit.
GOOD: create pages for findings, decisions, and patterns that will matter next session.

## Verification

```bash
# Check vault health and page count
mcp__graphthulhu__health
mcp__graphthulhu__graph_overview
```

```bash
# Identify gaps and orphaned pages
mcp__graphthulhu__knowledge_gaps
mcp__graphthulhu__list_orphans
```

## Reference

See [references/tool-catalogue.md](references/tool-catalogue.md) for the complete tool catalogue and [references/workflow.md](references/workflow.md) for detailed workflow steps.
