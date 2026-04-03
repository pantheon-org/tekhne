# Dependency Analysis

How to build a task dependency DAG and assign tasks to waves.

## Step 1 — Extract tasks

List every task with:
- **ID** — short identifier (`T1.1`, `P0`, `phase-3`, etc.)
- **Description** — what the task does
- **Dependencies** — IDs of tasks that must complete first (empty = none)

If dependencies are not stated in the source document, infer them:

| Signal in source | Inferred dependency |
|-----------------|---------------------|
| "after X is done" / "once X lands" | task depends on X |
| "requires X to exist" | task depends on X |
| Test task for feature F | depends on F implementation |
| Merge / cleanup task | depends on all tasks being merged |
| Validation task | depends on all implementation tasks |
| Deployment task | depends on validation task |
| Tasks touching the same file | sequential dependency on earlier task |
| Tasks touching independent files | no dependency (can be parallel) |

## Step 2 — Build the DAG

Represent the graph as an adjacency list:

```
T1.1 → []          (no dependencies)
T1.2 → []
T2.1 → [T1.1]      (depends on T1.1)
T2.2 → [T1.2]
T3.1 → [T2.1, T2.2]
```

Check for cycles — a cycle means the requirements are contradictory. Surface this to the user before proceeding.

## Step 3 — Assign waves (topological sort)

```
Wave(task) = 1                              if task has no dependencies
Wave(task) = 1 + max(Wave(dep) for dep in dependencies)
```

All tasks with the same wave number can execute concurrently.

Example:

```
T1.1 deps=[]       → Wave 1
T1.2 deps=[]       → Wave 1
T1.3 deps=[]       → Wave 1
T2.1 deps=[T1.1]   → Wave 2   (1 + max(1) = 2)
T2.2 deps=[T1.1, T1.2] → Wave 2
T3.1 deps=[T2.1]   → Wave 3
T3.2 deps=[T2.1, T2.2] → Wave 3
T4.1 deps=[T3.1, T3.2] → Wave 4
```

## Step 4 — Label waves

Give each wave a short descriptive label:

| Pattern | Example label |
|---------|--------------|
| Diagnostics / audits | "Diagnostics & Design" |
| Setup / scaffolding | "Bootstrap & Extract" |
| Core implementation | "Fix & Build" |
| Tests / coverage | "Coverage & Validation" |
| Integration / merge | "Integration & Wrap-up" |
| Pre-requisites | "Pre-work" |
| Final checks / PR | "Wrap-up" |

## Step 5 — Decide parallel vs sequential

A wave is **parallel** when:
- It contains 2 or more tasks that touch different files/modules, AND
- There is no ordering constraint among those tasks.

A wave is **sequential** when:
- It contains only 1 task, OR
- Tasks within the wave must run in a specific order (e.g. "apply migration THEN verify"), OR
- A critical shared resource prevents concurrent work (e.g. single CloudFormation stack).

A wave is **must land together** when:
- Tasks within the wave must be committed/deployed atomically (e.g. paired CDK resource removal + consumer update).

## Step 6 — Assign branches

For parallel waves, assign one branch per workstream:

```
<type>/<scope>-<slug>
```

Examples:
- `refactor/scripts-lib-http`
- `feat/phase-3-clean-lambda`
- `test/baseline`
- `ci/deployment-automation`

For sequential waves, one branch covers the whole wave:
```
<type>/<wave-label-slug>
```

## DAG diagram format

Emit in the `Dependency Graph` section of the wave document:

```text
Wave 1 (parallel)
  T1.1 <description> ─┐
  T1.2 <description> ─┤─► Wave 2
  T1.3 <description> ─┘

Wave 2 (parallel, unblocked after Wave 1)
  T2.1 <description> (← T1.1) ─┐
  T2.2 <description> (← T1.2) ─┤─► Wave 3
  T2.3 <description> (← T1.3) ─┘

Wave 3 (sequential)
  T3.1 <description> (← all Wave 2)
```

Rules:
- Use `─┐`, `─┤`, `─┘` for parallel merges.
- Use `(← X)` to annotate the specific dependency.
- Use `─►` to show the wave that unblocks.

## Edge cases

### Task with many dependents

If T1.1 is a prerequisite for 10 tasks, those 10 tasks are still all Wave 2 —
they just all depend on T1.1. Do not create an intermediate wave just to "stage" them.

### Circular dependency detected

Surface to user:
```
ERROR: Circular dependency detected: T2.1 → T3.1 → T2.1
Cannot assign wave numbers until this is resolved.
Suggested fix: [description of how to break the cycle]
```

### Tasks with unknown dependencies

When a task says "depends on the above" without specifying, treat it as depending on all tasks earlier in the same phase/section.

### Large waves (>5 tasks)

Split large parallel waves into sub-groups by semantic domain, keeping them in the same wave number. Use phase rows in the table to separate the groups visually.
