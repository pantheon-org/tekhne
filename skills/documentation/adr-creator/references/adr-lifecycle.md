# ADR Lifecycle

An Architecture Decision Record moves through a small, well-defined set of
states. Understanding the transitions keeps the log honest and prevents the two
most common failures: editing history, and orphaning superseded records.

Status lives in the record's own YAML frontmatter, alongside a `history` list
that grows every time the status moves. Both are written by the CLI. Editing
`status:` by hand skips the history entry, which is the one thing the log exists
to be trustworthy about.

## Status values

The CLI accepts exactly five statuses. Anything else is rejected, naming the
valid values.

| Status | Meaning | Set by |
| --- | --- | --- |
| `proposed` | The record exists and is being written. Assigned at creation. | `create` |
| `review-requested` | The record scored 80 or above and is waiting on a human. | `review <slug>` |
| `accepted` | The decision is in force. Its prose is now frozen. | `update <slug> -s accepted` |
| `deprecated` | The decision no longer applies, and no single record replaced it. | `update <slug> -s deprecated` |
| `superseded` | A later record replaces this one, and names it. | `update <slug> --superseded-by <new>` |

There is no `rejected` status. A decision that was considered and declined is
recorded as `deprecated`, or left at `proposed` with the reasons written into
the prose, depending on whether it should count in the accepted-decision totals
that `status` reports.

## Transitions

```text
proposed ──review──▶ review-requested ──update -s accepted──▶ accepted
                                                                 │
                                          ┌──────────────────────┤
                                          ▼                      ▼
                                    deprecated              superseded
```

- **Review** is the only gated transition. `review <slug>` re-runs the
  completeness check and refuses below 80, printing the failing rules and
  changing nothing.
- **Accept** with `update <slug> -s accepted` once the team ratifies the choice.
- **Deprecate** when a decision stops applying but no single record replaces it,
  for example because the subsystem was removed.
- **Supersede** is the transition for a changed decision, and takes two
  commands, in order.

`update -s` will move a record from any status to any other: the CLI records
every move but does not police the order, so the discipline is yours.

## The history trail

Each status change appends an entry to the record's frontmatter:

```yaml
history:
- from: proposed
  to: review-requested
  at: 2026-09-15T08:03:10Z
- from: review-requested
  to: accepted
  at: 2026-09-15T09:14:02Z
```

This is the audit trail: a reviewer can see that the record reached
`review-requested` before it was accepted, and when. Setting a status to the
value it already holds appends nothing, so the trail carries no noise.

## Superseding

Superseding never edits the meaning of the old record. It is two steps:

1. **Create the replacement.** `pantheon-adr create <new-slug> -d "..."` on its
   own branch, filled and checked like any other record. The CLI will not do
   this as part of the supersede, and refuses `--superseded-by` pointing at a
   record that does not exist.
2. **Retire the old one.** `pantheon-adr update <old-slug> --superseded-by
   <new-slug>` sets the old record's status to `superseded`, writes
   `superseded_by` into its frontmatter, appends the transition, and writes
   `supersedes` onto the replacement.

```yaml
# the old record
status: superseded
superseded_by: adopt-grafana-tempo

# the replacement
supersedes: adopt-opentelemetry
```

The chain is navigable from either end without a second command. Neither
record's prose is touched, so the superseded decision still reads exactly as it
did when it was in force. That is the point: a reader can see what was once
correct and why it stopped being so.

A record should be superseded only once. If the replacement is itself later
replaced, it gains its own `superseded_by`; the original is left alone. Never
re-point an old record at a newer replacement, because that hides the
intermediate decision.

## Why immutability matters

The audit value of the log comes entirely from the guarantee that accepted text
does not change. A reader diffing `docs/adr` across two release tags must see
new records, new frontmatter and status flips, never silent edits to old
rationale. This is why `update` is restricted to metadata, and why the prose is
only ever added to by a new record.
