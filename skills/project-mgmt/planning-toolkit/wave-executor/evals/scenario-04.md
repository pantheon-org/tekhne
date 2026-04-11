# Scenario 04: Execute Wave 1 Where One Phase Is Blocked

## User Prompt

Execute Wave 1 of the plan at `.context/plans/research-gap.md`. Wave 0 has been verified. However, not all Wave 0 branches have been merged yet.

## Merged Wave 0 branches (confirmed on main)

- `feat/triage-memory` ✓
- `feat/triage-rag` ✓
- `feat/triage-conv` ✓
- `feat/academic-sweep` ✓

## NOT yet merged

- `feat/rubric` — still in review

## Wave document excerpt

```markdown
### Wave 1 — Consolidate + Synthesise (parallel)

> Gate: Wave 0 verified ✓

| Phase | Focus | Tasks | Status | Writes | Blocked on | Model |
|-------|-------|-------|--------|--------|------------|-------|
| `feat/synthesis` | Write Key Themes section from 14 analysis files | 1 | Pending | `ANALYSIS.md` | `feat/rubric` merged | smart |
| `feat/consolidate` | Flip PUNCHLIST, add REVIEWED rows, sync index | 3 | Pending | `REVIEWED.md`, `PUNCHLIST.md`, `REFERENCE_INDEX.md` | all Wave 0 triage branches merged | fast |
| `feat/benchmark-decision` | Resolve HELMET and LongBench disposition | 1 | Pending | `REVIEWED.md` | — | standard |
```

## Expected Behavior

1. Do NOT make an Agent call for `feat/synthesis` — it is correctly identified as blocked on `feat/rubric` not being merged
2. Make an Agent call for `feat/consolidate` with model: 'haiku'
3. Make an Agent call for `feat/benchmark-decision` with model: 'sonnet'
4. Make the two unblocked Agent calls in a single parallel response
5. Clearly report that `feat/synthesis` is blocked on `feat/rubric` not being merged, and state what is needed to unblock it

## Success Criteria

- **feat-synthesis-not-spawned**: No Agent call is made for `feat/synthesis` — it is correctly identified as blocked
- **feat-consolidate-spawned**: An Agent call is made for `feat/consolidate` with model: 'haiku'
- **feat-benchmark-decision-spawned**: An Agent call is made for `feat/benchmark-decision` with model: 'sonnet'
- **unblocked-agents-spawned-in-parallel**: The two unblocked Agent calls appear in a single parallel response
- **blocked-phase-reported-to-user**: The response clearly reports that `feat/synthesis` is blocked on `feat/rubric` not being merged, and states what is needed to unblock it

## Failure Conditions

- Spawns an Agent call for `feat/synthesis` despite it being blocked
- Uses the wrong model for `feat/consolidate` (should be `haiku`, not `sonnet` or `opus`)
- Uses the wrong model for `feat/benchmark-decision` (should be `sonnet`, not `haiku` or `opus`)
- Spawns the two unblocked agents in separate sequential messages instead of in parallel
- Does not report that `feat/synthesis` is blocked or explain what is needed to unblock it
