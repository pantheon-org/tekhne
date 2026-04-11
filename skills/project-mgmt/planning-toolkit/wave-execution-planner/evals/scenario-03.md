# Scenario 03: Update Wave Statuses After Wave 1 Completes

## User Prompt

Wave 1 of a test coverage improvement plan has been completed. Update the wave document to reflect this.

## Current wave document

The file `.context/plans/test-coverage-improvement.md` currently contains:

```markdown
# Test Coverage Improvement

**Status**: In Progress
**Assignee**: platform team

## Goal

Raise unit test coverage from 45% to 80% across the core service modules.

## Waves & Phases

### Wave 1 — Baseline & Diagnostics (parallel) — IN PROGRESS

> Gate: None — start immediately.

- [~] T1.1 — Run coverage report and identify uncovered modules — branch `test/coverage-audit`
- [~] T1.2 — Audit integration test gaps — branch `test/integration-audit`
- [~] T1.3 — Set up coverage thresholds in CI config — branch `ci/coverage-gates`

Verification:
- [ ] `bun run test --coverage` exits 0
- [ ] Coverage report generated at `coverage/lcov.info`
- [ ] CI pipeline runs coverage gate on PR

### Wave 2 — Unit Test Implementation (parallel) — PENDING

> Gate: Wave 1 verified ✓

- [ ] T2.1 — Write unit tests for auth module — branch `test/unit-auth`
- [ ] T2.2 — Write unit tests for data layer — branch `test/unit-data`
- [ ] T2.3 — Write unit tests for API handlers — branch `test/unit-api`

Verification:
- [ ] `bun run test` exits 0 with ≥80% line coverage
- [ ] No test uses real network calls or filesystem

### Wave 3 — Integration & Merge (sequential) — PENDING

> Gate: Wave 2 verified ✓

- [ ] T3.1 — Merge all test branches; resolve conflicts; confirm combined coverage ≥80%

Verification:
- [ ] `bun run test --coverage` exits 0 with ≥80% on main
- [ ] CI green on main branch
```

## What has been completed

All three Wave 1 tasks are done:
- T1.1: `bun run test --coverage` exits 0 and `coverage/lcov.info` was generated
- T1.2: Integration gaps documented; CI pipeline verified
- T1.3: Coverage thresholds added to CI; PR check runs on every push

Note: T2.1 is already in progress on branch `test/unit-auth`.

Update the wave document to reflect Wave 1 completion and announce the next wave.

## Expected Behavior

1. Tick all three Verification checklist items under Wave 1 (`- [x]`) indicating they were confirmed
2. Tick all three task lines under Wave 1 (`- [x] T1.1`, `T1.2`, `T1.3`) — not left as `[~]` or `[ ]`
3. Update the Wave 1 heading to include `— DONE` (e.g. `### Wave 1 — Baseline & Diagnostics (parallel) — DONE`)
4. Update the Wave 2 heading to `— IN PROGRESS` (not still PENDING), reflecting that T2.1 has started
5. Mark T2.1 under Wave 2 as in progress (e.g. `[~]`) since it has already started
6. Explicitly state in output that Wave 2 is now unblocked and describe its execution mode (parallel)
7. Write the updated content back to `.context/plans/test-coverage-improvement.md`
8. Do not create or modify separate task files to track status — all status changes are in the wave document only
9. Keep the document-level **Status** field as "In Progress" (not changed to Complete, since Waves 2 and 3 are still pending)

## Success Criteria

- **wave-1-verification-checks-ticked**: All three Verification checklist items under Wave 1 are ticked (`- [x]`)
- **wave-1-tasks-ticked**: All three task lines under Wave 1 are ticked (`- [x] T1.1`, `T1.2`, `T1.3`) — not left as `[~]` or `[ ]`
- **wave-1-heading-marked-done**: The Wave 1 heading is updated to include `— DONE`
- **wave-2-status-updated-to-in-progress**: The Wave 2 heading is updated to `— IN PROGRESS` (not still PENDING)
- **t2-1-marked-in-progress**: T2.1 under Wave 2 is marked as in progress (e.g. `[~]`)
- **next-wave-announced**: The agent output explicitly states that Wave 2 is now unblocked and describes its execution mode (parallel)
- **file-written-back**: The agent wrote the updated content back to `.context/plans/test-coverage-improvement.md`
- **no-status-tracked-in-task-files**: The agent did not create or modify separate task files to track status
- **document-header-status-correct**: The document-level **Status** field remains "In Progress"

## Failure Conditions

- Leaves Wave 1 Verification checklist items unticked
- Leaves Wave 1 task lines as `[~]` instead of ticking them `[x]`
- Does not update the Wave 1 heading to `— DONE`
- Leaves Wave 2 heading as `PENDING` instead of updating to `IN PROGRESS`
- Does not mark T2.1 as in progress
- Does not announce that Wave 2 is now unblocked
- Does not write the updated content back to the file
- Creates separate task status files instead of updating the wave document
- Changes the document-level Status to "Complete" prematurely
