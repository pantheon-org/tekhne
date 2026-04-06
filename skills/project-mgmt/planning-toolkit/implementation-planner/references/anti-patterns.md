# Implementation Planner — Anti-patterns

## Mode 1 anti-patterns

### Vague verification gates

```markdown
<!-- BAD: unverifiable -->
### Gate
The API works correctly and all tests pass.

<!-- GOOD: runnable, exit-code-based -->
### Gate
```sh
npm test -- --reporter=tap | tap-parser --ok
curl -sf http://localhost:3000/health | jq -e '.status == "ok"'
```
```

### Over-bundled tasks

```markdown
<!-- BAD: one task, five unrelated files, no isolation -->
# P01T03 — Set up project
Implement src/server.ts, src/routes/users.ts, src/db/migrations/001.sql,
package.json, and tsconfig.json.

<!-- GOOD: one task, one file, independently verifiable -->
# P01T03 — Initialise tsconfig.json
File: tsconfig.json
Verification: npx tsc --noEmit && echo "ok"
```

ALWAYS: one task = one independently verifiable unit of work.

### Root README with implementation detail

```markdown
<!-- BAD: implementation steps in the navigation index -->
# Plan: URL Shortener
## Phase 1
Install dependencies with `npm install`. Then create src/index.ts with the
following content: ...

<!-- GOOD: navigation index only -->
# Plan: URL Shortener
| Phase | Goal | Status |
|---|---|---|
| [01 — Bootstrap](phases/phase-01-workspace-bootstrap/README.md) | Runnable skeleton | pending |
```

### Zero-padded numbering violations

```
# BAD: breaks alphabetical sort, inconsistent
task-P1T1-setup.md
task-P1T10-final.md   ← sorts before P1T2

# GOOD: consistent alphabetical sort
task-P01T01-setup.md
task-P01T10-final.md
```

### Skipping validate-plan.sh

NEVER report completion without running `sh scripts/validate-plan.sh <slug>` first.
ALWAYS fix every schema violation and re-run until exit 0 before reporting to the user.

WHY: Schema violations caught here prevent downstream agents from parsing task files correctly. A plan that looks complete but fails validation is unusable.

### Ignoring the >8-phase guardrail

WHY: Plans with 9+ phases become unmanageable. Splitting or consolidating up-front is far cheaper than restructuring after files exist. Silently capping scope hides requirements from the user — a correctness bug, not a style preference.

When the PRD yields **9 or more** natural phases, the ONLY correct action is to
stop and ask. There is no other valid path.

NEVER silently cap at 8 phases — omitting scope without telling the user is a bug.
NEVER silently create 9+ phases — proceeding without the user's choice is a bug.
ALWAYS count phases as the very first action in Step 2, before designing anything.
ALWAYS stop and message the user the moment the count reaches 9.
ALWAYS wait for the user's answer before running any scripts or creating any files.

```
# BAD: silently limits plan to 8 phases without telling the user
# (agent designs 8 phases, leaves out the 9th domain entirely)
sh scripts/new-phase.sh my-plan 08 ...   ← should have asked first

# BAD: creates all 12 phases without asking
sh scripts/new-phase.sh my-plan 12 ...   ← should have stopped at count=9

# GOOD: counts first, stops immediately when count ≥ 9, asks before any files
"I've identified 9 phases. Before I create any files, which do you prefer?
 A. Split into plan-core (phases 1–5) and plan-surface (phases 6–9)
 B. Consolidate to 7 phases by merging ops and DX into one
 C. Proceed with all 9 phases in a single plan"
```

### Manually creating directories without scripts

NEVER use `mkdir -p` to build the plan tree. ALWAYS use `new-plan.sh`, `new-phase.sh`,
and `new-task.sh` — they stamp the correct file stubs and naming conventions that
`validate-plan.sh` expects.

WHY: Hand-rolled directories miss required sections, use wrong naming conventions, and fail schema validation. The scripts are the single source of truth for the file contract.

### Modifying existing files when appending a new phase

WHY: Pre-existing files may be in active use by other agents or humans. Editing them causes conflicts, corrupts in-progress work, and breaks the additive-only contract that makes plans safe to extend incrementally.

NEVER edit, rename, or delete existing phase directories, task files, or the root
README's existing phase entries when adding a new phase to a plan. ALWAYS treat
all pre-existing files as read-only. The only permitted write to the root README
is appending the new phase entry at the end of the phases list.

```sh
# BAD: editing an existing task file while adding phase-03
edit phases/phase-02-data-model/tasks/task-P02T01-schema.md   ← MUST NOT touch

# GOOD: only new files are written
sh scripts/new-phase.sh my-plan 03 api-layer
sh scripts/new-task.sh my-plan 03 01 user-endpoints
# existing phase-01, phase-02 files are untouched
```

---

## Mode 2 anti-patterns

### Numeric-only directory names

```
# BAD: opaque — must open file to understand contents
phases/1/
phases/2/
phases/2/1/

# GOOD: self-documenting
phases/phase-1-codebase-analysis/
phases/phase-2-service-extraction-prep/
```

### Over-nesting small groups

```
# BAD: 2 items don't need their own group directory
phase-3-user-service/
  activities/
    group-a-database/
      activity-3.1-create-schema.md    # only 2 items — flatten!
      activity-3.2-run-migrations.md
    group-b-api/
      activity-3.3-implement-crud.md   # only 1 item — definitely flatten!

# GOOD: flat under activities/ when <3 children
phase-3-user-service/
  activities/
    activity-3.1-create-schema.md
    activity-3.2-run-migrations.md
    activity-3.3-implement-crud.md
```

### Deleting source before validation

NEVER delete the source document before `validate-structure.sh` exits 0.
ALWAYS treat the source as the ground truth until the new hierarchy is confirmed valid.

```sh
# BAD: data loss if hierarchy is invalid
rm -rf docs/old-plan.md
sh scripts/validate-structure.sh docs/refactoring/phases  # too late

# GOOD: validate first, delete after
sh scripts/validate-structure.sh docs/refactoring/phases && rm docs/old-plan.md
```

### Mixing Mode 1 and Mode 2 without detection

NEVER apply Mode 2 restructuring to a new PRD, or Mode 1 scaffolding to an existing
flat document. ALWAYS check the signal table in "When to use each mode" before
deciding which mode applies.

---

## Cross-cutting NEVER rules

### NEVER generate a plan without first reading the full requirements document

- **WHY**: Partial context produces plans with missing dependencies, incorrect phase ordering, and tasks that contradict the actual requirements — errors that are expensive to fix after scaffolding files exist.
- **BAD**: Skim the first section of the requirements and begin generating phase files before understanding the full scope and constraints.
- **GOOD**: Read the complete requirements document before designing any phases; ask clarifying questions for ambiguous acceptance criteria before running any scripts.

### NEVER create tasks too large to verify independently

- **WHY**: Tasks that span multiple days or produce multiple unrelated artifacts cannot be meaningfully reviewed, tested in isolation, or handed off to another agent; they become the primary source of plan ambiguity.
- **BAD**: "Phase 2 Task 3: Implement the entire authentication system" — no single verifiable output, no clear scope boundary, impossible to confirm done.
- **GOOD**: "Phase 2 Task 3: Implement JWT token issuance endpoint with unit tests" — one endpoint, one test suite, one PR, one runnable verification command.

### NEVER mix technical implementation tasks with cross-cutting concerns in the same phase

- **WHY**: Accessibility, performance, and security requirements affect multiple features; mixing them into feature phases makes them easy to skip, defers them to "later" that never comes, and obscures coverage gaps in review.
- **BAD**: Scatter "add input validation" and "check for XSS" across individual feature tasks where they are easily overlooked during implementation.
- **GOOD**: Dedicate a dedicated hardening phase or explicit cross-cutting tasks for security, performance validation, and accessibility review with their own gates.

### NEVER estimate effort without identifying risks and dependencies

- **WHY**: Estimates without risk analysis are systematically optimistic; downstream dependencies and integration risks are the most common source of schedule overruns and are only visible when explicitly listed.
- **BAD**: Generate a plan with effort estimates per phase and no section identifying external blockers, integration complexity, or uncertainty factors.
- **GOOD**: Include a dependencies and risks section in each phase README that identifies external blockers, integration complexity, and uncertainty factors alongside the effort estimate.
