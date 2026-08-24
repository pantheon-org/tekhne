# Task: Find Decisions That Never Got an ADR

The repository has a `.context/plans/` directory where the team drops design notes and spike
write-ups. Nobody has checked in a while whether any of them settled on a decision that was never
turned into an ADR. `docs/adr/` currently contains one record:

```text
docs/adr/0001-use-postgresql-for-primary-store.md   (Accepted, Source: .context/plans/2026-01-10-db-choice.md)
```

`.context/plans/` contains:

```text
2026-01-10-db-choice.md         (the source of ADR-0001, already linked)
2026-02-03-caching-spike.md     (ends with "## Decision" ... "We will use Redis for the session cache.")
2026-02-20-log-format-notes.md  (a "## Findings" section, no stated direction)
```

## What to do

Check whether any planning document contains a decision that has no ADR pointing back at it, and
say what should happen next.

## Output Specification

Produce:

1. The command used to run the check.
2. Which file (if any) was flagged, and why.
3. The recommended next step for the flagged file (extract an ADR from it, per
   [Deriving an ADR from an Existing Document](../../references/context-extraction.md)), and confirmation
   that the already-linked and purely-observational documents are correctly left alone.
