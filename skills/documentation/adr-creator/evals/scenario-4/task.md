# Task: Find Decisions That Never Got a Record

The repository has a `.context/plans/` directory where the team drops design
notes and spike write-ups. Nobody has checked in a while whether any of them
settled on a decision that was never turned into a record. `docs/adr/` holds one
record:

```text
use-postgresql-for-primary-store.md   accepted
  frontmatter `related` lists ../../.context/plans/2026-01-10-db-choice.md
```

`.context/plans/` contains:

```text
2026-01-10-db-choice.md         (the source of the Postgres record, already linked)
2026-02-03-caching-spike.md     (ends with "## Decision" ... "We will use Redis for the session cache.")
2026-02-20-log-format-notes.md  (a "## Findings" section, no stated direction)
```

## What to do

Check whether any planning document contains a decision that no record points
back at, and say what should happen next.

## Output Specification

Produce:

1. The command used to run the check.
2. Which file (if any) was flagged, and why.
3. The recommended next step for the flagged file, per
   [Deriving an ADR from an Existing Document](../../references/context-extraction.md),
   plus confirmation that the already-linked and purely observational documents
   are correctly left alone.
