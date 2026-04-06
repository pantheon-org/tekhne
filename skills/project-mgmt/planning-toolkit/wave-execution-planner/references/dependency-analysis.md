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
| Infrastructure task removes a resource (config value, secret, permission grant, env var, queue) that the live application still reads at runtime | both the infrastructure removal and the consumer code removal **must land together** — flag for Step 5 |
| New code requires a resource (env var, schema column, API endpoint) that does not yet exist in the deployed environment | both the resource-creation task and the consumer code task **must land together**, or the resource-creation wave must precede the consumer wave — flag for Step 5 |
| A task changes an interface (API contract, event schema, DB column type) in a way that is incompatible with existing consumers | use expand-contract sequencing: Wave N adds the new alongside the old, Wave N+1 migrates consumers, Wave N+2 removes the old — flag for Step 5 |

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
- Tasks within the wave must be committed/deployed atomically because any intermediate deployed state would break a live service.

**How to detect it:** For every wave boundary, ask: *"If this wave deploys and the next has not landed yet, does any existing functionality break?"* If yes, tasks on both sides must be in the same **must land together** wave. Three patterns trigger this:

| Pattern | Broken intermediate state | Fix |
|---------|--------------------------|-----|
| **Stranded consumer** | A resource (config, env var, permission, queue) is removed in Wave N; code that still reads it is only removed in Wave N+1 | Merge both tasks into one must-land-together wave |
| **Premature consumer** | Code that requires a new resource (env var, schema column, API endpoint) ships in Wave N; the resource is created in Wave N+1 | Create the resource first (earlier wave), or merge into must-land-together |
| **Breaking contract** | An API, event schema, or DB column changes incompatibly in Wave N; consumers that rely on the old contract are only updated in Wave N+1 | Use expand-contract sequencing: add new alongside old → migrate consumers → remove old |

Common triggers:
- Removing a configuration value, secret, or permission grant that the live service reads at startup or runtime
- Removing a queue, topic, or storage resource that the live service publishes or reads from
- Deploying a service that reads a new required env var before that variable is provisioned
- Adding a NOT NULL database column before the code that writes to it is deployed
- Changing an API contract (removing a field or endpoint) before all consumers are updated

> Note: the code-level dependency graph will NOT catch these. Runtime dependencies have no source imports, so a naive topological sort assigns the tasks to different waves. You must reason about the **deployed runtime state**, not the source tree.

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

### Unsafe intermediate deploy states

The naive topological sort assigns waves based on code-level imports. It cannot detect runtime dependencies. Three patterns produce an unsafe intermediate state; all require manual correction.

#### Stranded consumer

When a wave removes an infrastructure resource AND a later wave removes the consumer code that still references it, deploying the first wave alone breaks the live service.

**Fix:** merge both tasks into the same **must-land-together** wave. They can be worked in parallel worktrees but must deploy together.

Example (decommissioning a third-party SDK):
```
WRONG — separate waves:
  Wave 2: remove config parameter, permission grant, env var   (infrastructure)
  Wave 3: remove SDK client & dead code paths                  (application code)
  → Deploying Wave 2 alone breaks the live service

CORRECT — same wave:
  Wave 2 (must land together):
    Phase 1: remove config parameter, permission grant, env var   (infrastructure)
    Phase 2: remove SDK client & dead code paths                  (application code)
  > Phases 1 and 2 must deploy atomically.
```

#### Premature consumer

When a wave deploys code that depends on a resource that does not yet exist in the environment (new required env var, NOT NULL schema column, new API endpoint), the service fails on startup or at the call site.

**Fix:** either create the resource in an earlier wave (safe default), or merge resource-creation and consumer into a single **must-land-together** wave.

Example (adding a required external API key):
```
WRONG — consumer first:
  Wave 1: deploy service code that reads NEW_API_KEY env var
  Wave 2: provision NEW_API_KEY in the environment
  → Wave 1 fails at startup: env var missing

CORRECT — resource first:
  Wave 1: provision NEW_API_KEY in the environment
  Wave 2: deploy service code that reads NEW_API_KEY
```

#### Breaking contract change

When a wave changes an interface (REST endpoint shape, event schema, DB column type) in a way that is incompatible with current consumers, deploying the change alone breaks every caller that has not yet been updated.

**Fix:** use expand-contract sequencing across three waves. Never remove the old version until all consumers have migrated.

Example (renaming an API field):
```
WRONG — big-bang:
  Wave 2: rename field user_id → userId in both API and all consumers
  → Any consumer not yet deployed breaks

CORRECT — expand-contract:
  Wave 2: API emits both user_id and userId (backwards-compatible)
  Wave 3: migrate all consumers to read userId
  Wave 4: remove user_id from the API response
```
