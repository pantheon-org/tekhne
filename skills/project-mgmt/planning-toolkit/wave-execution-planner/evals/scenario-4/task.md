# Task: Start Wave 2 while Wave 1 verification is incomplete

The wave document for a data pipeline refactor lives at `.context/plans/pipeline-refactor.md`.
Wave 1 has two of three verification checks confirmed, but the third is still open.
The user wants to start Wave 2 immediately.

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

## What the user wants

The e2e test is flaky and unlikely to fail for real reasons. T2.1 is already
coded up. Can you mark Wave 2 as in-progress and start T2.1?
