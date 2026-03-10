---
name: implementation-planner
description: >
  Converts a PRD or requirements document into a structured, phased implementation
  plan with individual phase files and granular per-task files written to
  .context/plans/. Also restructures existing monolithic planning documents into
  digestible, hierarchical directory structures. Creates a root plan index
  summarising all phases, a numbered phase file per phase, and a numbered task
  file per task inside each phase directory. Use when the user asks to create an
  implementation plan, break down a PRD, convert requirements to tasks, structure
  project phases, generate a roadmap, plan a project in sprints, organise task
  breakdown, split a monolithic planning doc, or decompose a spec into phases and
  tasks.
license: MIT
compatibility: opencode
metadata:
  version: 3.0.0
  audience: agents
  workflow: planning, project-management, task-breakdown, restructuring
---

# Implementation Planner

## Mental model

An implementation plan is a **navigable contract between an agent and a codebase**.
Each file answers a single question: "What do I need to do next, how do I do it,
and how do I prove it is done?"

Two failure modes to avoid:

1. **The monolith** — one giant plan.md nobody reads because everything is in one file
2. **The skeleton** — directories with empty READMEs that give no useful signal

The sweet spot is a tree where every node either navigates (README) or implements
(task file), and every leaf has a runnable verification command.

## Quick Start

**Mode 1 — Create a new plan from a PRD:**

```sh
sh scripts/new-plan.sh url-shortener-service
sh scripts/new-phase.sh url-shortener-service 01 workspace-bootstrap
sh scripts/new-task.sh url-shortener-service 01 01 initialise-npm-package
sh scripts/validate-plan.sh url-shortener-service
```

**Mode 2 — Split a monolithic document:**

```sh
# 1. Create hierarchy manually (steps below) or from JSON:
sh scripts/generate-structure.sh --plan plan.json
# 2. Validate before removing source
sh scripts/validate-structure.sh docs/refactoring/phases
```

## When to use each mode

| Signal | Mode |
|---|---|
| User provides a PRD, spec, or requirements description | Mode 1 |
| User provides a single large planning document to split | Mode 2 |
| User provides flat phase files to reorganise | Mode 2 |
| User says "add a phase" to an existing plan | Mode 1 (additive) |
| User says "split", "organise", "refactor this plan" | Mode 2 |

## Trigger phrases

**Creating a new plan:**
- "create an implementation plan"
- "plan this out in phases"
- "break this PRD into tasks"
- "generate a phased plan"
- "write a detailed implementation plan"
- "create tasks from this spec"
- "decompose this into phases and tasks"
- "create a project plan / roadmap"
- "break down tasks" / "task breakdown"
- "sprint planning"

**Restructuring an existing plan:**
- "split this plan"
- "organize phases"
- "break down implementation docs"
- "create task hierarchy"
- "refactor this planning doc"
- "split this into separate files"

---

## Mode 1 — Create New Plan

### Inputs

| Input | Description |
|---|---|
| PRD / spec | A document, inline description, or file path describing what to build |
| Phase count | Optional — infer from scope if not provided |
| Output path | Optional — defaults to `.context/plans/` |

### Output structure

```
.context/plans/
  plan-<slug>/
    README.md                         # root index: goal, all phases, status table
    phases/
      phase-01-<slug>/
        README.md                     # phase overview: goal, gate, tasks summary
        tasks/
          task-P01T01-<slug>.md       # task: goal, file, implementation, verification
          task-P01T02-<slug>.md
      phase-02-<slug>/
        README.md
        tasks/
          task-P02T01-<slug>.md
          task-P02T02-<slug>.md
```

Each file is self-contained: an agent can work on a single task file without
reading the rest of the plan.

### Step 1 — Read and analyse the requirements

Read the PRD in full. Identify:

- The primary goal and non-negotiable constraints
- Natural delivery milestones (bootstrapping, core features, integration, hardening, etc.)
- Dependencies between areas of work

### Step 2 — Design phases

**FIRST: count the natural phases before designing anything.**

Each distinct domain, capability area, or labelled section in the PRD counts as
one phase unless two sections share a single atomic deliverable that cannot be
tested independently. When in doubt, count them separately — err on the side of
a higher count to avoid missing the guardrail.

Scan the PRD, count every distinct section or domain, and record the total.
If the count is **9 or more**, STOP immediately — do not design phases, do not
run any scripts, do not create any files. Instead, message the user with the
count and 2–3 concrete options:

```
I've identified N natural phases from the requirements. Before I create any files,
please choose one of these approaches:

A. Split into two plans: plan-<core-slug> (phases 1–5) and plan-<surface-slug> (phases 6–N)
B. Consolidate to 7 phases by merging [phase X] and [phase Y] into one
C. Proceed with all N phases in a single plan

Which would you prefer?
```

Wait for the user's answer before doing anything else. Do **not** silently cap at
8 and omit scope. Do **not** proceed with any number of phases without the user
choosing when N ≥ 9.

Once the phase count is confirmed to be ≤ 8 (either naturally or after
consolidation), continue designing:

Group work into sequential phases where each phase delivers a testable, deployable
increment. Each phase must have:

- A clear goal statement
- A gate (pass/fail acceptance criteria that can be verified by running commands)
- Explicit dependencies on prior phases or existing artefacts

Typical phase progression for a greenfield project:

1. Workspace bootstrap (scaffolding, toolchain, CI skeleton)
2. Data model / schema foundation
3. Core domain logic
4. API / integration layer
5. UI / consumer surface
6. Quality hardening (observability, error handling, performance)
7. Release preparation

Adapt freely — fewer phases for small projects, more for large ones.

### Step 3 — Decompose phases into tasks

Each task must be:

- Completable in isolation (no hidden cross-task dependencies unless declared)
- Verifiable with a concrete shell command or observable output
- Scoped to a single file or a tightly coupled set of files

Use the identifier format `P{phase_number}T{task_number}`, both zero-padded.
Example: `P02T03` = phase 2, task 3. Use 1-based numbering (`01`, `02`, …) for
consistent alphabetical sorting.

### Step 4 — Scaffold and write output files

**MUST use the scaffold scripts** — never create directories or files manually.
The scripts stamp the correct stubs that `validate-plan.sh` expects. Using `mkdir`
or writing files from scratch will produce structures that fail validation.

**MUST NOT modify existing files** — all operations are additive only. When
appending phases to an existing plan, run `new-phase.sh` and `new-task.sh` for
the new content only. Never edit, rename, or delete files that already exist in
the plan. Existing phase directories, task files, and the root README must remain
byte-for-byte identical after the operation, except that the root README may have
new phase entries appended to its phase listing.

```sh
# 1. Create the plan root — MUST run first (skip if plan already exists)
sh scripts/new-plan.sh <plan-slug>

# 2. Create each phase directory — MUST use this for every phase
sh scripts/new-phase.sh <plan-slug> <phase-number> <phase-slug>

# 3. Create each task file inside its phase — MUST use this for every task
sh scripts/new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>
```

Slugs are lowercase kebab-case summaries of the title. Examples:
- Plan "E-commerce Checkout Redesign" → `plan-ecommerce-checkout-redesign`
- Phase "Workspace Bootstrap" → `phase-01-workspace-bootstrap`
- Task P01T02 "Root package.json" → `task-P01T02-root-package-json.md`

After scaffolding, fill in the generated stub files following the structure
defined in `references/templates/plan.yaml`, `references/templates/phase.yaml`,
and `references/templates/task.yaml`.

> **Do not** embed implementation detail in the root `README.md` — keep it as a
> navigation index only. Detail belongs in phase and task files.

### Step 5 — Validate all output files

**MUST run before reporting to the user — no exceptions.**

```sh
sh scripts/validate-plan.sh <plan-slug>
```

The script checks each file against its schema in `references/schemas/`:

| File | Schema |
|---|---|
| `plan-<slug>/README.md` | `references/schemas/plan.schema.json` |
| `phases/phase-NN-<slug>/README.md` | `references/schemas/phase.schema.json` |
| `phases/phase-NN-<slug>/tasks/task-*.md` | `references/schemas/task.schema.json` |

**If any file fails:** fix the violation and re-run `validate-plan.sh`. Repeat
until exit 0. Do not report completion until exit 0 is confirmed.

Every task file MUST include a verification section with a concrete, runnable
shell command. Gates MUST be exit-code-based (exit 0 / non-zero, file exists,
URL returns 200). Gates MUST NOT use vague language like "works correctly" or
"tests pass" without specifying the exact command.

### Step 6 — Report to the user

After writing all files, output a summary:

```
Created implementation plan at .context/plans/plan-<slug>/

  README.md
  phases/phase-01-workspace-bootstrap/README.md  (N tasks)
  phases/phase-01-workspace-bootstrap/tasks/task-P01T01-*.md
  ...
```

---

## Mode 2 — Restructure Existing Plans

Use this mode when the source is a monolithic document or flat set of phase
files that needs to be reorganised into a navigable hierarchy.

### Decision table

| Source type | Approach | Automation |
|---|---|---|
| Flat `.md` phase files | Manual workflow (steps 1–7 below) | Run `validate-structure.sh` after |
| JSON plan definition | Automated | Run `generate-structure.sh`, then validate |
| Existing structure | Validation only | Run `validate-structure.sh` |

### Resource loading rules

Load scripts and templates **only when needed**:

- Always load `scripts/validate-structure.sh` after completing a manual split or when asked to validate
- Always load `scripts/generate-structure.sh` when the user provides a JSON plan file
  - Load `references/templates/*.yaml` only when using automation or customising output
  - Load `references/schemas/*.json` only when debugging validation errors
- Do **not** load scripts/schemas/templates when doing a purely manual split

### Expert heuristics

**When to flatten vs subdivide:**

| Signal | Action |
|---|---|
| Item has <3 children | Flatten — merge with sibling or parent |
| All children fit on one README screen (<20 lines) | Flatten |
| Navigation depth would exceed 4 levels | Flatten intermediate level |
| Item has >10 children | Subdivide |
| Children cluster into distinct semantic groups | Subdivide |
| Parallel work streams or different team ownership | Subdivide |

Rule of thumb: **3–7 items per group** is the sweet spot.

**Naming conventions that scale:**

```
step-1-extract-movement-logic/     # GOOD: survives refactors
step-1-refactor-game-code/         # BAD: vague, becomes meaningless

activity-1-analysis-complete/      # GOOD: outcome-oriented
step-1-initial-setup/              # BAD: "initial" becomes misleading later
step-1-project-bootstrap/          # GOOD: timeless
```

### Pre-split verification

Before splitting, confirm:

1. Source type detected (use decision table above)
2. Natural groupings identified (phases → activities/steps → groups)
3. Numbering convention detected (`step-1.1`, `step-1.2` belong together)

Ask yourself:
- Does each phase/step have 3–10 child items?
- Are there natural breaks in the content (headings, numbered sections)?
- Will a reader understand the hierarchy without opening every file?

### Step 1 — Establish structure

```
docs/refactoring/phases/
  phase-{number}-{name}/
    README.md
    activities/              # OR steps/
      README.md
      activity-{number}-{description}/
        README.md
        activity-{number}.{sub}-*.md
```

Max depth: **4 levels** (phase → activities/steps → group → leaf). Flatten if deeper.

> **Never use numeric-only directory names** (`step-1/`, `activity-2/`) and
> **never use generic names** (`step-1-stuff/`) — contributors cannot navigate
> without opening every file.

### Step 2 — Write leaf file content

Each leaf file must contain: title, description, checklist, acceptance criteria, status.

See `references/templates/step-file.yaml` for the exact structure.

### Step 3 — Write README files

Every non-leaf directory needs a README explaining its purpose and listing its
children. Minimum 3 lines. See `references/templates/phase-readme.yaml`,
`references/templates/group-readme.yaml`, and `references/templates/intermediate-readme.yaml`.

### Step 4 — Apply names

Format: `{type}-{number}-{kebab-description}`. Use naming heuristics above.

### Step 5 — Group related items

Items with the same prefix (`1.x`, `2.x`) go in the same parent directory.
Never mix groups (no `step-2.1` inside `step-1-extract/`).

### Step 6 — Update README links

Update all README links after restructuring. Verify with `validate-structure.sh`.

### Step 7 — Cleanup and validate

Create the new hierarchy first, validate, then remove the old flat files — never
delete source before the new structure is confirmed valid.

```sh
sh scripts/validate-structure.sh docs/refactoring/phases
# Exit 0 = valid, 1 = invalid
```

### Validation checklist

Before marking complete:

- [ ] Every phase has its own directory
- [ ] Every directory has a `README.md`
- [ ] All step/activity directories have descriptive names
- [ ] Related items are grouped (`1.1`, `1.2` together; not `1.x` with `2.x`)
- [ ] All README links resolve correctly
- [ ] Old flat files removed
- [ ] Each leaf file has: title, description, checklist, acceptance criteria, status

```sh
sh scripts/validate-structure.sh <phases-dir>
# Exit 0 = valid, 1 = one or more violations (details printed to stdout)
```

> For automation (JSON plan generation), error recovery recipes, and legacy naming
> conventions see [references/mode2-advanced.md](references/mode2-advanced.md).

---

## Anti-patterns

### Mode 1 anti-patterns

#### Vague verification gates

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

#### Over-bundled tasks

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

#### Root README with implementation detail

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

#### Zero-padded numbering violations

```
# BAD: breaks alphabetical sort, inconsistent
task-P1T1-setup.md
task-P1T10-final.md   ← sorts before P1T2

# GOOD: consistent alphabetical sort
task-P01T01-setup.md
task-P01T10-final.md
```

#### Skipping validate-plan.sh

NEVER report completion without running `sh scripts/validate-plan.sh <slug>` first.
ALWAYS fix every schema violation and re-run until exit 0 before reporting to the user.
Schema violations caught here prevent downstream agents from parsing task files.

#### Ignoring the >8-phase guardrail

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

#### Manually creating directories without scripts

NEVER use `mkdir -p` to build the plan tree. ALWAYS use `new-plan.sh`, `new-phase.sh`,
and `new-task.sh` — they stamp the correct file stubs and naming conventions that
`validate-plan.sh` expects.

#### Modifying existing files when appending a new phase

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

### Mode 2 anti-patterns

#### Numeric-only directory names

```
# BAD: opaque — must open file to understand contents
phases/1/
phases/2/
phases/2/1/

# GOOD: self-documenting
phases/phase-1-codebase-analysis/
phases/phase-2-service-extraction-prep/
```

#### Over-nesting small groups

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

#### Deleting source before validation

NEVER delete the source document before `validate-structure.sh` exits 0.
ALWAYS treat the source as the ground truth until the new hierarchy is confirmed valid.

```sh
# BAD: data loss if hierarchy is invalid
rm -rf docs/old-plan.md
sh scripts/validate-structure.sh docs/refactoring/phases  # too late

# GOOD: validate first, delete after
sh scripts/validate-structure.sh docs/refactoring/phases && rm docs/old-plan.md
```

#### Mixing Mode 1 and Mode 2 without detection

NEVER apply Mode 2 restructuring to a new PRD, or Mode 1 scaffolding to an existing
flat document. ALWAYS check the signal table in "When to use each mode" before
deciding which mode applies.

---

## Worked examples

### Example: URL Shortener plan (Mode 1)

Given: PRD for a URL shortener with REST API, SQLite storage, and a health check.

**Scope analysis:** 3 natural phases (bootstrap → core logic → API + health).

```sh
sh scripts/new-plan.sh url-shortener-service
sh scripts/new-phase.sh url-shortener-service 01 workspace-bootstrap
sh scripts/new-task.sh url-shortener-service 01 01 initialise-npm-package
sh scripts/new-task.sh url-shortener-service 01 02 configure-typescript
sh scripts/new-phase.sh url-shortener-service 02 database-layer
sh scripts/new-task.sh url-shortener-service 02 01 create-sqlite-schema
sh scripts/new-task.sh url-shortener-service 02 02 implement-url-repository
sh scripts/new-phase.sh url-shortener-service 03 http-api
sh scripts/new-task.sh url-shortener-service 03 01 post-shorten-endpoint
sh scripts/new-task.sh url-shortener-service 03 02 get-redirect-endpoint
sh scripts/new-task.sh url-shortener-service 03 03 health-check-endpoint
sh scripts/validate-plan.sh url-shortener-service
```

Task `task-P03T01-post-shorten-endpoint.md` verification:

```sh
npm start &; sleep 1
curl -sf -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}' | jq -e '.code | length == 6'
kill %1
```

### Example: Scope exceeds 8 phases

PRD covers auth, ingestion, pipeline, storage, query, viz, multi-tenancy, ops, DX
(9 domains).

**Step 2, action 1:** Count natural phases = 9. Count ≥ 9 → guardrail triggered.

**Correct behaviour:** Stop immediately. Zero scripts run. Zero files created.
Send this message to the user and wait:

> I've counted 9 natural phases from the requirements. I need your input before
> I create any files:
>
> A. Split into two plans: `plan-platform-core` (auth, ingestion, pipeline,
> storage, query) and `plan-platform-surface` (viz, multi-tenancy, ops, DX)
>
> B. Consolidate to 7 phases: merge storage+query into one phase and ops+DX
> into one (→ 7 phases total)
>
> C. Proceed with all 9 phases in a single plan
>
> Which would you prefer?

**Wrong behaviour — do not do any of these:**
- Counting 9 phases but designing only 8 and omitting auth or DX
- Running `new-plan.sh` before receiving the user's answer
- Creating some phases then mentioning the limit
- Asking the question inside a code comment or after file creation

### Example: Restructure migration plan (Mode 2)

Source: `docs/migration-plan.md` with 5 numbered sections, ~35 items.

**Scope analysis:** 5 phases, 4–9 items each — within the 3–7 sweet spot for most.

```
docs/refactoring/phases/
  phase-1-codebase-analysis/           # 4 items → flat under activities/
    README.md
    activities/
      activity-1.1-dependency-graph.md
      activity-1.2-bounded-contexts.md
      activity-1.3-shared-libraries.md
      activity-1.4-data-ownership.md
  phase-3-user-service-extraction/     # 9 items → consider grouping
    README.md
    activities/
      group-a-implementation/          # 5 items (3.1–3.5)
        activity-3.1-copy-domain.md
        ...
      group-b-rollout/                 # 4 items (3.6–3.9)
        activity-3.6-deploy-staging.md
        ...
```

---

## Reference

### Deep dives

| Topic | Location |
|---|---|
| Mode 2 automation, error recovery, legacy naming | [references/mode2-advanced.md](references/mode2-advanced.md) |
| Before/after structure transformation example | [references/example-transformation.md](references/example-transformation.md) |
| File format templates and schemas | below |

### File Format Reference

| Template | Schema | Purpose |
|---|---|---|
| `references/templates/plan.yaml` | `references/schemas/plan.schema.json` | Root index structure (Mode 1) |
| `references/templates/phase.yaml` | `references/schemas/phase.schema.json` | Phase overview structure (Mode 1) |
| `references/templates/task.yaml` | `references/schemas/task.schema.json` | Individual task structure (Mode 1) |
| `references/templates/phase-readme.yaml` | `references/schemas/readme-file.schema.json` | Phase directory README (Mode 2) |
| `references/templates/group-readme.yaml` | `references/schemas/readme-file.schema.json` | Group/intermediate README (Mode 2) |
| `references/templates/intermediate-readme.yaml` | `references/schemas/readme-file.schema.json` | Activities/steps dir README (Mode 2) |
| `references/templates/step-file.yaml` | `references/schemas/step-file.schema.json` | Leaf step/activity file (Mode 2) |

See [references/example-transformation.md](references/example-transformation.md) for a before/after structure comparison.
See [references/mode2-advanced.md](references/mode2-advanced.md) for automation, error recovery, and legacy naming guidance.

---

## Notes

- Existing `.context/plans/` content is never deleted; new files are additive.
- If a plan already exists, append new phases rather than overwriting.
