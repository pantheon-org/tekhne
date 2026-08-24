# Scenario 2: Resolve an Existing Entry

## User Prompt

"Row 4 in the risk register (the missing SQS DLQ) is fixed now that ADR-018 landed. Mark it resolved."

## Context Given

```markdown
| 4 | No DLQ for async invokes | Deferred | Failed async EventBridge invocations of the digest Lambda have no dead-letter queue, so a crash is silently lost. | A transient failure during a cadence run disappears with no record and no retry. | 2026-07-10 | Open | -- | -- |
```

## Expected Behavior

1. Find row 4 by its `#`.
2. Change `Status` to `Resolved`, `Date` to today's date, and `Decision` to a self-contained sentence describing how
   it was resolved (e.g. referencing the SQS DLQ added under ADR-018) -- all three cells changed in the same edit.
3. Do not touch `Item`, `Type`, `Description`, `Risk if unaddressed`, or `Added` -- if the original `Description` is
   now slightly misleading, add a short clarifying clause rather than rewriting it.
4. Do not delete the row or renumber it.
5. Re-run `scripts/validate-risk-register-schema.sh` and confirm it passes now that all three lifecycle cells moved
   together.

## Success Criteria

- `Status` becomes `Resolved`, `Date` becomes a real date, `Decision` becomes a non-empty, self-contained sentence --
  all three changed together, not just one or two.
- `Item`, `Type`, `Description`, `Risk if unaddressed`, and `Added` are left unchanged (a clarifying clause appended
  to `Description` is acceptable; a rewrite is not).
- Row `#4` keeps its `#`; it is not deleted, renumbered, or reordered.
- Response confirms `scripts/validate-risk-register-schema.sh` was re-run and passed.

## Failure Conditions

- Only `Status` is changed while `Date` or `Decision` are left as `--`.
- `Description`, `Type`, `Added`, or `Risk if unaddressed` are rewritten rather than left as-is.
- The row is deleted or renumbered instead of resolved in place.
- No mention of re-running the schema validation script.
