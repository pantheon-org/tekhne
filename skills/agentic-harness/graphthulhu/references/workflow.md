# graphthulhu Workflow Reference

Detailed steps for each graphthulhu workflow pattern.

---

## Orient (session start)

1. Call `graph_overview` to see total pages, namespaces, and most-connected nodes.
2. If working in a known area, call `search` with relevant keywords to surface prior context.
3. If unsure what exists, call `list_pages` filtered by the relevant namespace.

Expected output: a clear picture of what is already known before writing any new code.

---

## Retrieve

1. Use `search` for full-text discovery across all blocks.
2. Use `get_page` when you know the exact page name.
3. Use `find_connections` to understand how two topics relate.
4. Use `find_by_tag` to locate pages grouped by tag.

Expected output: relevant blocks with surrounding context.

---

## Write (after analysis or investigation)

1. Check for an existing page with `search` or `get_page`.
2. If the page exists: add new findings with `upsert_blocks` or `append_blocks`.
3. If the page is new: create it with `create_page` using the correct namespace.
4. Link the new page to related pages with `link_pages`.

Expected output: durable, findable knowledge stored in the vault.

---

## Decisions

1. When a non-trivial architectural or tooling choice arises, call `decision_create` on the page where context is richest.
2. When a decision is made, call `decision_resolve` with the chosen option and rationale.
3. Use `decision_defer` if the question is blocked or premature.
4. Use `decision_check` to review open decisions on a page.

Expected output: decisions are traceable and linked to the context that produced them.

---

## Corrections

When the user corrects stored knowledge:

1. Call `search` to find the incorrect block or page.
2. Call `update_block` to fix incorrect content, or `delete_page` if the page is wholly wrong.
3. Do not continue until the correction is saved — stale memory is worse than no memory.

Expected output: vault reflects current ground truth before the session continues.
