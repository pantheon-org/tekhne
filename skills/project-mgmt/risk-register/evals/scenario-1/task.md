# Scenario 1: File a New Row for a Shortcut Just Taken

## User Prompt

"We just shipped the retry sweep without a dead-letter queue -- we're deferring that to next sprint. Log it in the
risk register."

## Context Given

`docs/RISK_REGISTER.md` currently ends with:

```markdown
| # | Item | Type | Description | Risk if unaddressed | Added | Status | Date | Decision |
|---|------|------|--------------|----------------------|-------|--------|------|----------|
| 7 | SES sandbox limit | Accepted Risk | Sending is capped to verified identities only while in SES sandbox. | Digests to unverified recipients silently fail delivery. | 2026-07-20 | Open | -- | -- |
```

## Expected Behavior

1. Confirm this is genuinely a new, unresolved item (not already covered by an existing row or a documented decision)
   before filing it.
2. Assign the new row `#8` (last row's `#` + 1) -- never renumber row 7 or insert out of order.
3. Classify it as `Type: Shortcut` (an interim workaround in place now, not a `Deferred` future feature).
4. Write a self-contained `Description` and a concrete, one-sentence `Risk if unaddressed` -- not a vague
   "might be an issue" placeholder.
5. Set `Added` to today's date, `Status: Open`, `Date: --`, `Decision: --`.
6. State that both validation scripts should be run and should exit `0` with no output before committing.
7. Do not put any confidential detail (customer names, credentials) in the row.

## Success Criteria

- New row uses `#8`, not a renumbered or reused id.
- `Type` is `Shortcut`, matching "an interim workaround in place now."
- `Description` is self-contained and concrete, not vague.
- `Risk if unaddressed` states a specific, concrete failure mode (e.g. "a failed async delivery has no automatic
  retry path and is only caught by the log-derived alarms").
- `Added` is a real `YYYY-MM-DD` date; `Status` is `Open`; `Date` and `Decision` are both `--`.
- Response mentions running `scripts/validate-risk-register-append-only.sh` and
  `scripts/validate-risk-register-schema.sh` and confirming they pass before committing.
- No confidential or personal data appears in the row.

## Failure Conditions

- Row uses `#7` or any id other than `8`, or renumbers the existing row.
- `Description` restates the risk with no concrete detail ("DLQ stuff might need doing").
- `Risk if unaddressed` is vague ("could be a problem").
- `Status`/`Date`/`Decision` are inconsistent (e.g. `Status: Open` but `Date` filled in).
- No mention of running the validation scripts before committing.
