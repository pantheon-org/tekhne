# Tool Classification Guide

Use this guide when classifying a tool during the **Classify** step of the triage workflow.

## Tag Taxonomy

| Tag | Meaning |
|---|---|
| `compression` | Reduces token count (summarisation, pruning, compaction) |
| `tiered-loading` | Priority-based injection (L0/L1/L2, lazy vs eager) |
| `token-budgeting` | Hard caps, soft priorities, eviction policies |
| `injection` | How content enters the context window |
| `cli` | Operated via command-line interface |
| `daemon` | Runs as a background process |
| `mcp-server` | Exposes capabilities via MCP protocol |
| `session-lifecycle` | Manages wake/sleep/checkpoint across sessions |
| `retrieval` | RAG / retrieval-augmented context feeding |

A tool may carry multiple tags. Prefer tags that describe what the tool **does to context** over tags that describe its delivery mechanism.

## Scope Assessment

| Verdict | Criteria |
|---------|----------|
| **In scope** | Manages what is in the active context window: compression, tiered loading, token budgeting, injection, session continuity |
| **Borderline** | Primarily long-term memory storage but has a context-window injection layer |
| **Out of scope** | Pure vector DBs, embedding services, or tools with no context-window involvement |

For borderline tools: triage and flag the overlap in REVIEWED.md; do not silently exclude or include.

## Examples

### Clearly in scope
- A library that compresses conversation history to fit a token budget → `compression`, `token-budgeting`
- A daemon that swaps context segments based on recency → `tiered-loading`, `session-lifecycle`

### Borderline
- A vector DB with a retrieval-injection pipeline → `retrieval` (note: assess whether the injection layer is first-class)

### Out of scope
- A pure embedding service with no context injection — log as out-of-scope in REVIEWED.md
