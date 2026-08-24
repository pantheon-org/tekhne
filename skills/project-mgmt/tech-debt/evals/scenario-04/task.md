# Scenario 04: Item Already Tracked Elsewhere

## User Prompt

"aislop already flagged an unused import in the delivery retry sweep as a
baseline finding in .aislop/baseline.json. Also add it to docs/TECH_DEBT.md
so it doesn't get lost."

## Expected Behavior

1. Recognize that `aislop`'s baseline (`.aislop/baseline.json`) already turns
   this into a scored, regression-tracked inventory item -- this is
   explicitly listed in When NOT to Use.
2. Decline to duplicate the same finding into `docs/TECH_DEBT.md`.
3. Explain that this doc is not a second home for an item type `aislop`'s
   baseline already owns.
4. Point the user back to `.aislop/baseline.json` as the system of record,
   rather than creating a duplicate low-visibility copy.

## Success Criteria

- Agent identifies the item as already covered by `aislop`'s baseline, not
  eligible for a tech-debt row.
- Agent declines to file a duplicate row in `docs/TECH_DEBT.md`.
- Agent states the reason (avoiding a second, disconnected home for the same
  item type) rather than silently refusing.
- Agent points back to `.aislop/baseline.json`.

## Failure Conditions

- Agent files the aislop finding as a new tech-debt row without checking for
  the baseline overlap.
- Agent files it and also leaves the baseline entry as the only other record,
  creating two disconnected trackers for the same item.
- Agent refuses without explaining why or without pointing back to
  `.aislop/baseline.json`.
