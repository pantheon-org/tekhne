# Mode A: Bulk Import (reference)

Turn a historical work log kept in a legacy source system into fully-formed, dated journal entries in this
repository's `YYYY/MM/` format. The source is retiring or already retired; the entries are the durable record
going forward.

## What counts as a legacy source system

Anything that held a running work log outside this repository and is being retired or migrated away from: a wiki
page (Confluence, Notion, a plain internal wiki), a shared document, a chat channel's pinned notes, or an export
from a previous tool entirely. Confluence is one common example - a page holding a "weekly journal" with dated
sections - but the workflow below does not assume Confluence specifically.

## Fetching the source

Read access to the legacy system is whatever that system offers: its API, an export/download, or an MCP
connector if one is configured (for a Confluence-backed source this is typically a `confluence_get_page`-style
tool). Large pages often spill to a tool-result file rather than returning inline; parse the markdown body out of
that file rather than loading the raw tool result into context, e.g.:

```bash
jq -r '.result | fromjson | .metadata.content.value' page.json > source.md
```

## Splitting into entries

1. Identify the source's own section boundaries - most legacy logs are broken into dated chunks (a weekly
   heading, a daily heading, a per-meeting heading). Derive the year from the nearest month/year header when a
   section heading omits it.
2. Generate one file per non-empty section at `YYYY/MM/YYYY-MM-DD-imported-entry.md`, dated by that section's
   date. Preserve bullets verbatim; normalise only formatting (list-marker style, whitespace). Do not add
   interpretation, summarisation, or content the source did not contain - this is a faithful transcription, not a
   rewrite.
3. Use `scripts/import-legacy-entries.sh` as the generator for a source that follows the common "level-3 heading
   naming a date" shape. For a source with a different heading convention (a table, a flat list, front-matter per
   page), write a small ad hoc script or do the split by hand - do not force the source into the script's
   assumptions if it does not fit.
4. Skip empty sections. A week/period with nothing recorded produces no file.

## Frontmatter and structure

Every generated entry uses the base `journal-entry-creator` frontmatter and section set (`## Session Overview`,
`## Compliance`, `## Tags` at minimum - see that skill's `journal-entry.yaml` template). A minimal example:

```yaml
---
title: "Imported Entry - July 6, 2020"
date: 2020-07-06
authors:
  - Alex Rivera
tags:
  - imported
  - legacy-import
  - "2020"
source: "https://example.atlassian.net/wiki/spaces/TEAM/pages/12345/Weekly+Journal"
status: published
---
```

`source` is the legacy system's URL for the page/export the entry came from - keep it even after that system is
retired, as the historical provenance record.

## Validate, format, commit as one unit

```bash
bash skills/documentation/journal-entry-creator/scripts/validate-journal-entry.sh <generated-files>
npx prettier --write <generated-files>
npx markdownlint-cli2 --fix <generated-files>
```

Commit the import as one atomic commit before starting Mode B enrichment on any of the imported entries - a clean
import boundary makes it obvious which content is untouched, verbatim history versus later reconstructed detail.

## What this mode does not do

- It does not interpret or summarise. Content is transcribed, not rewritten.
- It does not reach into an issue tracker for ticket detail - that is Mode B, run afterwards, per ticket, on
  entries this mode produced.
- It does not touch links to other systems that have since moved - that is Mode C.
