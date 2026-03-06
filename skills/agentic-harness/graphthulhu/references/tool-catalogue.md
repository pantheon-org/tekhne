# graphthulhu Tool Catalogue

Complete reference for all graphthulhu MCP tools, grouped by purpose.

---

## Status & Administration

| Tool | Purpose |
|---|---|
| `health` | Check server status, version, backend type, read-only mode, page count. Use to verify the server is alive. |
| `analysis_health` | Diagnostic health check for the graph analysis subsystem. |
| `graph_overview` | High-level overview: total pages, blocks, links, most connected pages, orphan count, namespace breakdown. Call at session start. |
| `reload` | Reload the graph from disk. Use after external edits to the vault. |

---

## Reading Pages & Blocks

| Tool | Purpose |
|---|---|
| `list_pages` | List pages with optional filters: namespace, property key, tag. Sorts by name, modified, or created. |
| `get_page` | Fetch a single page with all its blocks and properties. |
| `get_block` | Fetch a single block by UUID. |
| `get_links` | Get all pages that link to or from a given page. |
| `query_properties` | Query pages by property key/value pairs. Useful for finding pages by metadata. |

---

## Search & Discovery

| Tool | Purpose |
|---|---|
| `search` | Full-text search across all blocks. Returns matching blocks with surrounding context (parent chain and siblings). Use `compact: true` to save tokens. |
| `find_by_tag` | Find all pages tagged with a specific tag. |
| `find_connections` | Discover how two pages are connected through the link graph. Returns direct links, shortest paths, and shared connections. |
| `traverse` | Walk the graph from a starting page up to a given depth. Useful for exploring a topic cluster. |
| `knowledge_gaps` | Identify pages that are referenced but don't exist yet, or pages with sparse content. |
| `list_orphans` | List pages with no incoming or outgoing links — isolated knowledge nodes. |
| `topic_clusters` | Group pages into topic clusters based on link density. |

---

## Writing Pages & Blocks

| Tool | Purpose |
|---|---|
| `create_page` | Create a new page with optional properties and initial blocks. Use namespace/topic naming (e.g. `analysis/auth-flow`). |
| `upsert_blocks` | Batch-create blocks on a page. Supports nested children for hierarchies. Can append or prepend. |
| `append_blocks` | Append one or more blocks to an existing page. |
| `update_block` | Update the content or properties of a single block by UUID. |
| `delete_block` | Delete a single block by UUID. |
| `move_block` | Move a block to a different position or parent. |
| `bulk_update_properties` | Update properties on multiple pages at once. |
| `link_pages` | Create a bidirectional link between two pages. |
| `rename_page` | Rename a page and update all incoming links. |
| `delete_page` | Delete a page permanently. |

---

## Decisions

Decisions are tracked as `#decision`-tagged blocks on the page where context is richest.

| Tool | Purpose |
|---|---|
| `decision_create` | Create a new `DECIDE` block with a question, options, and deadline. |
| `decision_check` | List all open decisions on a page. |
| `decision_resolve` | Mark a decision as resolved with the chosen option and rationale. |
| `decision_defer` | Mark a decision as deferred with a reason. |

---

## Journal

Journal tools operate on date-keyed pages (YYYY-MM-DD format).

| Tool | Purpose |
|---|---|
| `journal_range` | Fetch journal pages within a date range. |
| `journal_search` | Search within journal pages only. |
