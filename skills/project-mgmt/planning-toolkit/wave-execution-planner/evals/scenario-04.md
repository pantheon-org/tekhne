# Scenario 04: Start Wave 2 While Wave 1 Verification Is Incomplete

## User Prompt

The wave document for a data pipeline refactor lives at `.context/plans/pipeline-refactor.md`. Wave 1 has two of three verification checks confirmed, but the third is still open. The user wants to start Wave 2 immediately.

## Current wave document

```markdown
# Data Pipeline Refactor

**Status**: In Progress
**Assignee**: data-platform team

## Waves & Phases

### Wave 1 — Extract shared utilities (parallel) — IN PROGRESS

> Gate: None — start immediately.

- [x] T1.1 — Extract `lib/transform.ts` — branch `refactor/pipeline-lib-transform`
- [x] T1.2 — Extract `lib/validate.ts` — branch `refactor/pipeline-lib-validate`
- [x] T1.3 — Extract `lib/fetch.ts` — branch `refactor/pipeline-lib-fetch`

Verification:
- [x] `bun run typecheck` exits 0 across all three extracted modules
- [x] All three branches merged; no import errors in the main codebase
- [ ] End-to-end smoke test passes: `bun run test:e2e pipeline` exits 0

### Wave 2 — Implement pipeline stages (parallel) — PENDING

> Gate: Wave 1 verified ✓

- [ ] T2.1 — Implement `stages/ingest.ts` — branch `refactor/pipeline-stage-ingest`
- [ ] T2.2 — Implement `stages/transform.ts` — branch `refactor/pipeline-stage-transform`
- [ ] T2.3 — Implement `stages/emit.ts` — branch `refactor/pipeline-stage-emit`

Verification:
- [ ] `bun run test` exits 0 on all pipeline stage branches
- [ ] No direct imports of old pipeline entry point remain
```

## User request

The e2e test is flaky and unlikely to fail for real reasons. T2.1 is already coded up. Can you mark Wave 2 as in-progress and start T2.1?

## Expected Behavior

1. Explicitly refuse to mark Wave 2 as in-progress or start any Wave 2 task, citing the incomplete Wave 1 verification gate
2. Name the specific verification check that has not passed: the e2e smoke test (`bun run test:e2e pipeline`)
3. Leave the wave document unchanged: Wave 2 remains PENDING and no Wave 2 task is marked in-progress
4. Articulate the gate rule: Wave N+1 cannot start until ALL Wave N verification checks are confirmed, regardless of flakiness concerns
5. Tell the user what they need to do next: confirm the e2e test passes (or explicitly accept the risk themselves) before Wave 2 can begin

## Success Criteria

- **blocks-wave-2-advancement**: Agent explicitly refuses to mark Wave 2 as in-progress or start any Wave 2 task, citing the incomplete Wave 1 verification gate
- **identifies-specific-failing-check**: Agent names the specific verification check that has not passed (`bun run test:e2e pipeline`)
- **wave-2-document-unchanged**: The wave document is not modified: Wave 2 remains PENDING and no task under Wave 2 is marked in-progress
- **explains-gate-rule**: Agent articulates the gate rule: Wave N+1 cannot start until ALL Wave N verification checks are confirmed, regardless of flakiness concerns
- **provides-path-to-unblock**: Agent tells the user what they need to do next: confirm the e2e test passes (or tick it off if they accept the risk) before Wave 2 can begin

## Failure Conditions

- Marks Wave 2 as in-progress or starts any Wave 2 task despite the incomplete gate
- Does not identify the specific failing check by name
- Modifies the wave document (even partially) while Wave 1 has an unconfirmed check
- Does not articulate the gate rule
- Does not tell the user what action is needed to unblock Wave 2
- Accepts the user's "flaky test" rationale and proceeds anyway
