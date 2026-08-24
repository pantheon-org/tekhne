---
name: journal-entry-import
description:
  "Reconstructs journal entries from external systems: bulk-imports a historical work log from a legacy source
  system such as a deprecated Confluence page into dated entries, enriches ticket references with self-contained
  detail entries reconstructed from an issue tracker, and annotates links to systems that have since moved such as
  a Bitbucket-to-GitLab migration. Use when asked to import an old journal, migrate a log into the journal format,
  turn ticket mentions into entries, expand a log with ticket detail, or point old links at a new host. This is
  not for repairing frontmatter on entries that already exist in this format, which is the journal CLI's own
  backfill command. Produces validated dated entries compatible with journal-entry-creator, complete with
  bidirectional cross-links and annotated external links."
---

# Journal Entry Import

Reconstruct journal entries from systems outside this repository: a bulk import of a historical work log, ticket
references enriched into self-contained detail entries, and links repaired to point at systems that have since
moved. Three independent modes, each callable on its own: **Mode A** (bulk import), **Mode B** (ticket
enrichment), **Mode C** (external link migration). A common sequence runs Mode A first, then Mode B on selected
tickets from the imported entries, then Mode C whenever an imported link points at a system that has since moved
- but nothing requires running them in that order or all three together.

## Mindset

These entries reconstruct history that already happened somewhere else; the job is transcription and synthesis,
not authorship. **Reconstruct, do not invent** - every claim traces to the source log, the ticket, or its
comments. Each detail entry must **stand alone**: a reader understands the task without opening the ticket or the
original log. **Synthesise, don't transcribe** - a reconstructed discussion reads as what was decided and why,
never as a comment-by-comment transcript. Follow the [Data and Privacy Policy](references/data-and-privacy-policy.md)
on every entry regardless of mode. At batch scale, **trust the filesystem over an agent's self-report**, and
recompute remaining work from the repository on every resume rather than from a saved list.

Know when this is the wrong tool: a brand-new entry documenting today's work is `journal-entry-creator`'s job, not
this skill's. Repairing missing `title`/`date` frontmatter keys on entries that already exist in the repo's format
is the journal CLI's own one-pass `pantheon-journal backfill` command - a different problem from reconstructing
entries that do not exist yet, and this skill deliberately avoids that name to keep the two apart.

## Prerequisites

This skill produces entries in the `journal-entry-creator` format and depends on it for the base frontmatter
schema, the tag taxonomy, and the structural validator
(`journal-entry-creator/scripts/validate-journal-entry.sh`). Read that skill's `SKILL.md` first if you have not
used it before - this skill assumes its conventions (triple-synced dates, single-H1 format, tag consistency) and
does not repeat them.

Confirm the companion CLI is present before using any `pantheon-journal` subcommand this skill references (e.g.
`lint`, `index`):

```bash
pantheon-journal --version
```

Each mode also needs read access to the relevant external system, however that access is provided in the current
environment - a direct API, an export/download, or an MCP connector:

- **Mode A** - read access to the legacy source system (e.g. a `confluence_get_page`-style tool for a
  Confluence-backed source, or simply an exported markdown/HTML file).
- **Mode B** - read access to the issue tracker, including comments (e.g. a `jira_get_issue`-style tool).
- **Mode C** - read access to the migration target's code host to resolve new project locations (e.g. a GitLab or
  GitHub MCP connector), plus whatever the source host's archived URLs already give you.

`bash`, `awk` (GNU awk / `gawk` - the bundled scripts use its 3-arg `match()` extension), `prettier`, and
`markdownlint-cli2` (the repo's markdown toolchain) round out the requirements.

## When to Use

- **Mode A** - a historical work log (a legacy wiki page, an export, a retiring tool's log) needs importing into
  `YYYY/MM/` entries.
- **Mode B** - a ticket reference in an existing entry warrants a dated, self-contained detail entry reconstructed
  from the issue tracker (description + comments), cross-linked to that entry.
- **Mode C** - entries link to a system that has moved (a code host migration, a tracker instance change) and the
  links should point at the current home without losing the historical record.

## When Not to Use

- For a normal new journal entry documenting current work, use `journal-entry-creator` directly.
- For repairing missing `title`/`date` frontmatter on entries that already exist in this repo's format, use
  `pantheon-journal backfill` - a one-pass batch repair, not a reconstruction from an external source.
- Do not enrich a ticket from a system the available tools cannot reach (an ITSM ticket, a support-desk case, a
  tracker instance the connector is not granted); leave it as a plain link instead of guessing at its content.
- Do not create a detail entry for a ticket with nothing to narrate - see the "enough data" bar in
  [Mode B Ticket Enrichment](references/mode-b-ticket-enrichment.md).
- Do not rewrite a migrated link in place - annotate it (Mode C); see
  [Mode C External Link Migration](references/mode-c-link-migration.md).

## Frontmatter Schema

Bulk-imported entries (Mode A) use the plain `journal-entry-creator` frontmatter - nothing extra. Ticket-detail
entries (Mode B) follow
[`assets/schemas/ticket-detail-frontmatter.schema.json`](assets/schemas/ticket-detail-frontmatter.schema.json), a
superset of the base schema:

```yaml
---
title: "PROJ-2058 - Move to fetching credentials at run time"
date: 2020-07-06
authors:
  - Alex Rivera
tags:
  - proj-2058
  - credentials
  - ticket-detail
  - "2020"
source: "https://example.atlassian.net/browse/PROJ-2058"
status: published
---
```

Field rules: `title` starts with the ticket key; `date` matches the filename; `tags` include the key lowercased,
topic tags, `ticket-detail`, and the year as a quoted string, and must match the `## Tags` section; `source` is
the tracker's permalink to the ticket. `ticket-detail` extends `journal-entry-creator`'s tag taxonomy type facet
alongside `troubleshooting`, `learning`, and `ticket-refinement`. **Never** set `refinement_ticket` on a
ticket-detail entry (see Anti-Patterns).

## Procedure

### Mode A - bulk import

1. Fetch the source. Large pages often spill to a tool-result file; parse the markdown body out of it rather than
   loading the raw result into context.
2. Split into the source's own dated sections (a weekly heading, a daily heading); derive the year from the
   nearest month/year header when a section omits it.
3. Generate one file per non-empty section at `YYYY/MM/YYYY-MM-DD-imported-entry.md`, dated by that section's
   date. Preserve content verbatim; normalise only formatting. Use `scripts/import-legacy-entries.sh` as the
   generator for a source with a "level-3 heading naming a date" shape.
4. Validate, format, lint every file. Commit the import as one atomic commit before starting Mode B enrichment on
   any of it.

Full detail: [Mode A Bulk Import](references/mode-a-bulk-import.md).

### Mode B - ticket enrichment

1. Verify one ticket key resolves on the configured tracker instance. Skip references from unreachable systems.
2. Fetch the ticket with comments (`fields` including `comment`).
3. Apply the "enough data" bar; skip and log thin tickets rather than inventing detail.
4. Date by the ticket's resolution/closed date (fallbacks: last-updated, then the date of the log entry that
   mentioned it). Write the detail entry from the template.
5. Add the bidirectional cross-link as plain markdown - never a frontmatter supersession field.
6. Validate with **both** validators, then commit per year.

Full detail: [Mode B Ticket Enrichment](references/mode-b-ticket-enrichment.md) (includes the fan-out pattern and
pitfalls for enriching many tickets at once).

### Mode C - external link migration

1. Extract the distinct external repos/links; resolve each to its current home (search by name and obvious
   renames, across the whole target group including deep subgroups and any aggregate/notebook repos).
2. Annotate, do not replace: keep the original archived URL and append the resolved current home. Deep links (PR
   numbers, commits) do not translate - point to the repo root.
3. For a large set, produce a mapping table (with a confidence rating) for sign-off before editing.

Full detail: [Mode C External Link Migration](references/mode-c-link-migration.md).

## Validation

Run for every file created or edited, then commit only when clean:

```bash
npx prettier --write <files>
npx markdownlint-cli2 --fix <files>
# imported entries and any other base-schema entry:
bash skills/documentation/journal-entry-creator/scripts/validate-journal-entry.sh <files>
# ticket-detail entries (runs the base validator too):
bash skills/documentation/journal-entry-import/scripts/validate-ticket-detail.sh <detail-files>
```

## Anti-Patterns

### NEVER set `refinement_ticket` on a ticket-detail entry

- **WHY:** `journal-entry-creator`'s base validator then requires a `## Proposed Ticket Description` section,
  which is for an in-progress ticket-refinement draft - it does not apply to a closed historical ticket.
- **BAD:** `refinement_ticket: PROJ-2058` in a ticket-detail entry's frontmatter.
- **GOOD:** put the ticket link in `source` and the metadata block only.

### NEVER invent a frontmatter field to express supersession for a cross-link

- **WHY:** tekhne's base frontmatter schema has no continuation/supersession field. A detail entry elaborates a
  log entry; it does not replace or hide it, so there is nothing to mark superseded.
- **BAD:** adding an ad hoc `continues_from` / `continued_by` field to link a detail entry back to its log entry.
- **GOOD:** plain-markdown bidirectional links (`## Related Entries` plus a nested "Detailed entry:" bullet).

### NEVER transcribe the ticket thread comment-by-comment

- **WHY:** a "he said, she said" list is noise; the reader wants what was decided and how it resolved.
- **BAD:** a `## Decisions and Resolution` section that quotes every comment in order.
- **GOOD:** a synthesised narrative, with names only where they carry weight, plus the decisive-contribution
  callout when the author was the pivot.

### NEVER write credentials, tokens, or bulk personal identifiers into an entry

- **WHY:** the repo must not accumulate secrets or end-user/customer personal data.
- **BAD:** pasting a raw access token or a list of account numbers into the entry body.
- **GOOD:** mask credentials; summarise personal identifiers and raw data dumps out, noting it in
  `## Compliance`. See the [Data and Privacy Policy](references/data-and-privacy-policy.md).

### NEVER commit a batch on the strength of an agent's "written" status alone

- **WHY:** an agent can report "written" without having written the file, and resolution-date dating can land a
  file in a year directory other than the one the batch started in - staging only that year silently drops it.
- **BAD:** committing because every fan-out agent returned `status: "written"`, and `git add`-ing only the
  current year.
- **GOOD:** filter results to files that actually exist on disk, and stage every year directory the batch touched.

### NEVER invent a current-home link for a repo that was not migrated

- **WHY:** a repo-mapping manifest entry that still points at the old host's URL means the repo was not migrated;
  a fabricated new-host link 404s.
- **BAD:** annotating a repo with a guessed GitLab/GitHub URL because "it's probably there too".
- **GOOD:** leave unmigrated links as archived originals; only annotate a real project confirmed present on the
  target host.

## References

- `journal-entry-creator` (sibling skill, same `documentation/` domain) - base frontmatter, tag taxonomy, and the structural validator; read before using this skill at all
- [Mode A Bulk Import](references/mode-a-bulk-import.md) - splitting a legacy source and generating verbatim dated entries
- [Mode B Ticket Enrichment](references/mode-b-ticket-enrichment.md) - reachability, dating, cross-links, and the batch fan-out pattern with its pitfalls
- [Mode C External Link Migration](references/mode-c-link-migration.md) - annotating links to a code host or tracker that has moved
- [Data and Privacy Policy](references/data-and-privacy-policy.md) - what to keep, mask, or summarise out on every entry
- [ticket-detail-frontmatter.schema.json](assets/schemas/ticket-detail-frontmatter.schema.json) - the ticket-detail frontmatter contract
- [ticket-detail-template.yaml](assets/templates/ticket-detail-template.yaml) - the full ticket-detail entry structure
