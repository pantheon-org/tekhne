# Scenario 01: File a Genuine Cleanup Row

## User Prompt

"While fixing the digest handler I noticed `packages/lambdas/src/analysis/render-html.ts`
still imports `LegacyRenderer` even though nothing calls it anymore -- eslint's
`no-unused-vars` is only a warning here, not an error, so the build doesn't
fail on it. Log this as tech debt."

## Expected Behavior

1. Confirm this is genuinely just cleanup (Rule 3) -- not a risk, shortcut, or
   open decision -- before filing. State that confirmation, don't just file.
2. Read the current last row's `#` in `docs/TECH_DEBT.md` and use `#` + 1 for
   the new row, not an assumed or arbitrary number.
3. Write a self-contained `Description`: what's wrong (unused import), where
   (the specific file), and why the existing gate didn't catch it (warning,
   not error).
4. Use `Area: Code quality` from the current taxonomy.
5. Set `Since` to today's date and `Status: Open`.
6. Size `Effort` (`S`/`M`/`L`) -- this is a one-line deletion, so `S`.
7. Run `scripts/validate-tech-debt-schema.sh` before treating the row as
   committed.

## Success Criteria

- Row filed with the next available `#` (last row's number + 1).
- `Description` is self-contained: names the file, the unused import, and why
  the existing lint gate is a warning not an error.
- `Area: Code quality`.
- `Effort: S`.
- `Status: Open`, `Since` is today's date in `YYYY-MM-DD`.
- Validation script run (or its command shown) before declaring done.
- No `Resolved` status used, and no append-only ceremony invoked.

## Failure Conditions

- Row filed without checking whether this is genuinely just cleanup.
- `#` reused incorrectly (duplicate or skipped) instead of last + 1.
- `Description` is vague ("clean up render-html.ts") without naming the
  specific unused import or why the gate missed it.
- Validation step skipped entirely.
- Row filed in `docs/RISK_REGISTER.md` instead (over-escalation) or not
  filed anywhere (under-response).
