# Task: Record a New Architectural Decision

A team has just agreed to move their message ingestion pipeline from a polling
model to an event-driven model backed by a message queue. They want this
captured as an Architecture Decision Record in the repository's `docs/adr`
directory.

The repository already contains one ADR:

```text
docs/adr/0001-use-postgresql-for-primary-store.md
```

## What to do

Create a new ADR for the decision "Adopt event-driven ingestion" using the
`adr` CLI, then fill in the template with the decision's context, the decision
itself, and its consequences. Record at least one realistic alternative that
was rejected.

## Output Specification

Produce:

1. The command used to create the record.
2. The resulting file path.
3. The completed ADR content, following the house template with all headings
   present and in order.
