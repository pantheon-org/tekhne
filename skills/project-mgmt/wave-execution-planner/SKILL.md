---
name: wave-execution-planner
description: "Groups plan phases and tasks into dependency-ordered waves for parallel subagent execution via git worktrees. Builds a task dependency DAG, assigns wave numbers via topological sort, emits a living wave document that tracks status as work lands, and updates wave progress when tasks complete. Use when asked to: group tasks into waves, plan parallel execution, schedule worktrees, create a wave breakdown, wave planning, dependency grouping, update wave statuses, parallel subagents, which tasks can run in parallel."
license: MIT
compatibility: opencode
metadata:
  version: 1.0.0
  audience: agents
  workflow: planning, project-management, parallel-execution, subagents
---

# Wave Execution Planner

Groups tasks into dependency-ordered waves so multiple subagents can work concurrently in isolated git worktrees.

## Core Principles

- **Waves enforce ordering, not scheduling** — a wave boundary exists only where a real dependency forces sequential execution. Parallel is the default.
- **One branch per parallel workstream** — each phase in a parallel wave gets its own branch and worktree; agents never share a working tree.
- **The wave document is the source of truth** — status lives in the wave plan, not scattered across individual task files.
- **Gates before advancing** — ALWAYS verify the current wave fully passes before starting the next one. A broken wave in production is harder to debug than a delayed start.

## Quick Start

1. Provide requirements, a PRD, or an existing plan.
2. Invoke **Mode A** to generate the wave document at:
   ```
   .context/plans/<plan-slug>.md
   ```
3. Work waves in order; run **Mode B** after each wave to tick off statuses.

## When to use

| Signal | Mode |
|---|---|
| New requirements / PRD with no existing plan | Mode A |
| Existing flat plan, phase list, or task breakdown | Mode A (from existing plan) |
| Wave document exists; tasks have been completed | Mode B |
| "Which wave is unblocked?" / "Update status" | Mode B |

## When not to use

- The plan has only 1–2 tasks with no parallelism — a plain checklist is simpler.
- Tasks are all sequential with no independent workstreams — a single branch is enough.
- The caller has explicitly structured their own wave plan and only needs status updates (use Mode B directly).

## Recognition examples

Input that triggers Mode A:

```text
"Create a wave plan for this refactor. Tasks: extract lib (no deps),
scaffold CLI (needs lib), add commands A/B/C (needs CLI), cleanup (needs all commands)."
```

Expected output skeleton:

```text
Wave 1 (parallel): extract-lib
Wave 2 (sequential): scaffold-cli  ← depends on Wave 1
Wave 3 (parallel): command-A, command-B, command-C  ← depend on Wave 2
Wave 4 (sequential): cleanup  ← depends on Wave 3
```

## Mode A — Create Wave Plan

See [references/wave-format.md](references/wave-format.md) for output format and full examples.

### Steps

1. **Read inputs** — requirements doc, PRD, or existing plan file.
2. **Extract tasks** — list every task with an ID, description, and explicit dependencies.
   If none are stated, infer from logical ordering (a test task depends on the implementation task).
3. **Build dependency DAG** — see [references/dependency-analysis.md](references/dependency-analysis.md).
4. **Assign waves** — Wave 1 = tasks with no dependencies; Wave N = tasks whose all dependencies are in waves 1..N-1.
5. **Decide execution mode** — a wave with >1 independent task is `parallel`; a wave with 1 task (or tasks that must run in order) is `sequential`.
6. **Write output** — emit `<plan-slug>.md` to `.context/plans/` using the format in [references/wave-format.md](references/wave-format.md).
7. **Validate** — every task appears in exactly one wave; no wave contains tasks that depend on each other.

## Mode B — Update Wave Status

See [references/status-tracking.md](references/status-tracking.md) for the full update protocol.

### Steps

1. **Read wave document** — load the current `.context/plans/<slug>.md`.
2. **Run verification gate** — execute the commands listed in the completed wave's `Verification:` checklist:
   ```bash
   # example gate for a test-coverage wave
   bun run test --coverage
   bun run typecheck
   git log --oneline main..HEAD
   ```
3. **Update statuses** — tick checkboxes or update `Status` column cells.
4. **Mark wave** — append `— DONE` to the wave heading when all verifications pass.
5. **Announce next wave** — state which wave is now unblocked and whether it is parallel or sequential.
6. **Save** — write the updated document back.

### Status transitions

```
Pending  →  In Progress  →  Done
                          ↓
                       Blocked  (add a > BLOCKED: note)
```

## Anti-patterns

- **NEVER put dependent tasks in the same wave.**
  WHY: agents working in parallel worktrees assume no ordering — placing dependent tasks together causes undefined behaviour or overwrite conflicts.

- **NEVER label a wave parallel if it contains only one task.**
  WHY: parallel signals subagent tooling to spin up worktrees; a single-task wave wastes setup overhead and misleads reviewers.

- **NEVER advance to Wave N+1 before Wave N verification passes.**
  WHY: a broken merge point in production compounds into every parallel branch; early detection is always cheaper.

- **NEVER track status inside individual task files.**
  WHY: distributed status creates stale reads and coordination failures when multiple agents update concurrently.

- **NEVER invent dependencies that are not stated in the requirements.**
  WHY: fabricated ordering reduces parallelism, slows execution, and breaks the contract between the plan and the actual work.

- **ALWAYS run the verification checklist before declaring a wave done** — "it looks right" is not a gate.

## References

- [references/wave-format.md](references/wave-format.md) — Wave document format, sections, and worked examples
- [references/dependency-analysis.md](references/dependency-analysis.md) — Building the DAG and assigning wave numbers
- [references/status-tracking.md](references/status-tracking.md) — Status update protocol and verification gates
- [references/wave-document.yaml](references/wave-document.yaml) — Copy-paste template for a new wave document
