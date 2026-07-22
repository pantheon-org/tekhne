# ADR Lifecycle

An Architecture Decision Record moves through a small, well-defined set of
states. Understanding the transitions keeps the log honest and prevents the two
most common failures: editing history and orphaning superseded records.

## Status values

| Status | Meaning | Mutable after? |
| --- | --- | --- |
| `Proposed` | The decision is drafted but not yet ratified by the team. | Yes, until accepted or rejected. |
| `Accepted` | The decision is in force. Context and Decision text are now frozen. | Only the Status line. |
| `Rejected` | The proposal was considered and declined. Kept for the record. | No. |
| `Deprecated` | The decision no longer applies but was not replaced by a specific ADR. | Only the Status line. |
| `Superseded` | A later ADR replaces this one. The Status names the replacement. | No, beyond the initial supersede. |

## Transitions

```text
Proposed ──accept──▶ Accepted ──supersede──▶ Superseded by ADR-NNNN
   │                    │
   └──reject──▶ Rejected└──deprecate──▶ Deprecated
```

- **Accept**: flip `Proposed` to `Accepted` once the team ratifies the choice.
  Change nothing else.
- **Reject**: flip `Proposed` to `Rejected`. Keep the record so the rejected
  option is not proposed again without context.
- **Deprecate**: use when a decision stops applying but no single ADR replaces
  it (for example, the subsystem was removed).
- **Supersede**: the primary transition for a changed decision. It is a
  two-part, linked operation and should be done with the CLI.

## Superseding chains

Superseding never edits the meaning of the old record. It performs two changes
atomically:

1. The old ADR's Status becomes `Superseded by ADR-NNNN`, pointing forward to
   its replacement.
2. A new Accepted ADR is created that references the old one in its Technical
   Story and Context, pointing backward.

The result is a doubly-linked chain a reader can follow in either direction:

```text
ADR-0002 (Superseded by ADR-0007) ──▶ ADR-0007 (Accepted, Supersedes ADR-0002)
```

A record may be superseded only once. If ADR-0007 is itself later replaced, it
gains `Superseded by ADR-0012`; ADR-0002 is left untouched. Never re-point an
old record at a newer replacement, as that hides an intermediate decision.

## Why immutability matters

The audit value of an ADR log comes entirely from the guarantee that accepted
text does not change. A reader diffing the log across two release tags must see
new records and Status flips, never silent edits to old rationale. This is why
the tool refuses to rewrite decision text and offers only `supersede`.
