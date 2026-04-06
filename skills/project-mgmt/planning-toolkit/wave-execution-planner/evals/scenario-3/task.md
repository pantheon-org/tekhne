# Task: Update wave statuses after Wave 1 completes

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

## What to do

Update the wave document to reflect Wave 1 completion and announce the next wave.
Note: T2.1 is already in progress on branch `test/unit-auth`.
