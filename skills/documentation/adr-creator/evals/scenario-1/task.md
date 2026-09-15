# Task: Record a New Architectural Decision

A team has just agreed to move their message ingestion pipeline from a polling
model to an event-driven model backed by a message queue. The work is on branch
`feat/adopt-event-driven-ingestion`.

The repository already has an ADR log containing one record:

```text
docs/adr/use-postgresql-for-primary-store.md
docs/adr/index.md
```

## What to do

Create the record for this decision using the `pantheon-adr` CLI, then fill in
the template with the problem, the options weighed, the chosen solution and its
rationale. Record at least one realistic alternative that was rejected. Confirm
the record is finished enough to request review.

## Output Specification

Produce:

1. The exact commands used to create and check the record.
2. The resulting file path and the slug and type recorded in its frontmatter.
3. The completed record, frontmatter and body, with all headings present and in
   order.
