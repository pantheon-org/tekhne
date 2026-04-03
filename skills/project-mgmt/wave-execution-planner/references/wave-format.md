# Wave Document Format

The wave document is the living execution record for a plan.
It is written once (Mode A) and updated in-place as waves complete (Mode B).

## File location

```
.context/plans/<plan-slug>.md
```

Use the plan slug from the requirements or a short descriptive identifier (e.g. `scripts-cli-refactor`, `infra-cleanup`, `auth-migration`).

## Document structure

````markdown
# <Plan Title>

**Ticket / ref**: <link or N/A>
**Status**: <In Progress | Complete | Blocked>
**Assignee**: <name or team>

## Goal

<One paragraph: what success looks like.>

## Conditions of Satisfaction

- [ ] <observable outcome 1>
- [ ] <observable outcome 2>

## Dependency Graph

<ASCII DAG — see dependency-analysis.md>

## Waves & Phases

### Wave 1 — <Label> (<parallel | sequential>)

> <Gate note: what must be true before this wave starts. "None — start immediately" for Wave 1.>

| Phase | Focus | Tasks | Status |
|-------|-------|-------|--------|
| [N](phase-N-<slug>.md) | <description> | <count> | Pending |

Verification:
- [ ] <command or check that must pass before Wave 2 starts>

### Wave 2 — <Label> (<parallel | sequential>)

> Gate: Wave 1 verified ✓

...

## Branch Strategy

| Branch | Tracks | Base |
|--------|--------|------|
| `<branch-name>` | <tasks or phases> | `<base>` |

Merge order: <branch A> → <branch B> → ...

## Definition of Done

- [ ] All wave verification gates pass
- [ ] All branches merged to <main branch>
- [ ] CI green on <main branch>
- [ ] <project-specific criterion>
````

## Wave section rules

- Wave heading: `### Wave N — <short label> (<parallel | sequential>)`
  - `parallel` = 2+ phases with no mutual dependency; each gets its own branch + worktree.
  - `sequential` = 1 phase, or phases that must run in order within the wave.
- The table lists phases (rows), not individual tasks. For small plans with no formal phases, list tasks directly.
- **Status values**: `Pending` → `In Progress` → `Done`
- Add a `> Gate:` blockquote at the top of each wave (except Wave 1).
- Add a `Verification:` checklist at the bottom of each wave.

## Phase row format

| Column | Content |
|--------|---------|
| Phase | Link to phase file, or inline phase ID `[N](path)` |
| Focus | 3–6 word description of what the phase accomplishes |
| Tasks | Integer count of discrete tasks inside the phase |
| Status | `Pending` / `In Progress` / `Done` |

If the plan has no separate phase files, use task IDs directly (e.g. `T1.1`, `T2.3`).

## Inline task format (no phase files)

Use this for smaller plans where phases are not broken into separate files:

```markdown
### Wave 1 — Extract shared lib (parallel)

> Gate: None — start immediately.

Run **T1.1–T1.4 in parallel, each in its own worktree.**
Each agent works independently; tasks touch different files with no overlap.

- [ ] **T1.1** — `scripts/lib/http.ts` — branch `refactor/scripts-lib-http`
- [ ] **T1.2** — `scripts/lib/dates.ts` — branch `refactor/scripts-lib-dates`
- [ ] **T1.3** — `scripts/lib/wikidata.ts` — branch `refactor/scripts-lib-wikidata`
- [ ] **T1.4** — `scripts/lib/paths.ts` — branch `refactor/scripts-lib-paths`

**Wave 1 merge**: after all branches pass validation, merge in order:
`refactor/scripts-lib-http` → `refactor/scripts-lib-dates` → `refactor/scripts-lib-wikidata` → `refactor/scripts-lib-paths`

Verification:
- [ ] All merged; `main` CI green
- [ ] No direct imports of old paths remain
```

## Dependency graph format

Place the DAG immediately after the Goal section.

````markdown
## Dependency Graph

```text
Wave 1 (parallel)
  T1.1 task-a ─┐
  T1.2 task-b ─┤─► Wave 2
  T1.3 task-c ─┘

Wave 2 (parallel, unblocked after Wave 1)
  T2.1 task-d (← T1.1) ─┐
  T2.2 task-e (← T1.2)  ─┤─► Wave 3
  T2.3 task-f (← T1.3)  ─┘

Wave 3 (sequential)
  T3.1 task-g (← all Wave 2)
```
````

## Worked example — phases in table (infrastructure cleanup)

````markdown
## Waves & Phases

### Wave 1 — Pre-work (sequential)

> Gate: None — start immediately.

| Phase | Focus | Tasks | Status |
|-------|-------|-------|--------|
| [0](infra-cleanup/phase-0-validation.md) | Validation & Coordination | 5 | Done |

Verification:
- [x] All validation checks pass; no regressions introduced

### Wave 2 — Cloud Infrastructure (must land together)

> Gate: Wave 1 verified ✓

| Phase | Focus | Tasks | Status |
|-------|-------|-------|--------|
| [1](infra-cleanup/phase-1-resource-removal.md) | Remove Unused Resource | 4 | Done |
| [2](infra-cleanup/phase-2-consumer-updates.md) | Update Consumers | 3 | Done |

> Phases 1 and 2 must deploy atomically — stage together before applying.

Verification:
- [x] Infrastructure diff shows only expected deletions
- [x] Plan/dry-run exits 0

### Wave 3 — Code Cleanup (parallel, same PR)

> Gate: Wave 2 verified ✓

| Phase | Focus | Tasks | Status |
|-------|-------|-------|--------|
| [3](infra-cleanup/phase-3-clean-business-logic.md) | Clean Up Business Logic | 7 | Done |
| [4](infra-cleanup/phase-4-clean-tests.md) | Clean Up Tests | 6 | Done |

> Phases 3 and 4 touch independent files and can be worked in parallel.

Verification:
- [x] Test suite exits 0
- [x] No references to removed resource remain
````
