# Prevent Duplicate Pages via Pre-Creation Search

## Problem/Feature Description

You have just finished reviewing the database schema for a project and have several findings to save:

- The `users` table is missing an index on `email`, causing full-table scans on login.
- Foreign key constraints are disabled on the `orders` table.
- Several `TEXT` columns should be `VARCHAR(n)` for storage efficiency.

You want to store these findings in the knowledge graph.

## Task

Attempt to save these database schema findings. During the process, demonstrate that you correctly check for duplicate pages before creating a new one.

The knowledge graph already contains a page called `analysis/database-schema` with some prior notes. Your task is to find it and append the new findings rather than creating a duplicate page.

## Expected Behaviour

- Call `search` with relevant keywords (e.g. "database", "schema") before calling `create_page`.
- Call `get_page("analysis/database-schema")` to confirm the page exists and review its current content.
- Append the new findings to the existing page using `append_blocks` or `upsert_blocks` — do NOT call `create_page`.
- Do NOT create a page like `analysis/db`, `analysis/database`, or `database-schema` (flat name) instead.
