---
name: implementation-planner
description: >
  Converts a PRD or requirements document into a structured, phased implementation
  plan with individual phase files and granular per-task files written to
  .context/plans/. Creates a root plan index summarising all phases, a numbered
  phase file per phase, and a numbered task file per task inside each phase
  directory.
license: MIT
compatibility: opencode
metadata:
  version: 1.0.0
  audience: agents
  workflow: planning, project-management, task-breakdown
---

# Implementation Planner

## What I do

Given a PRD or requirements document, I produce a complete, navigable implementation
plan written to `.context/plans/`:

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

Each file is self-contained: an agent can work on a single task file without reading
the rest of the plan.

## Trigger phrases

Use this skill when the user says any of the following (or close variants):

- "create an implementation plan"
- "plan this out in phases"
- "break this PRD into tasks"
- "generate a phased plan"
- "write a detailed implementation plan"
- "create tasks from this spec"
- "decompose this into phases and tasks"

## Inputs

| Input | Description |
|---|---|
| PRD / spec | A document, inline description, or file path describing what to build |
| Phase count | Optional — infer from scope if not provided |
| Output path | Optional — defaults to `.context/plans/` |

## Workflow

### Step 1 — Read and analyse the requirements

Read the PRD in full. Identify:

- The primary goal and non-negotiable constraints
- Natural delivery milestones (bootstrapping, core features, integration, hardening, etc.)
- Dependencies between areas of work

### Step 2 — Design phases

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

Use the identifier format `P{phase_number}T{task_number}`, both zero-padded. Example: `P02T03` = phase 2, task 3.

### Step 4 — Scaffold and write output files

Use the scripts in `scripts/` to create the directory structure deterministically,
then populate the generated README / task files using the templates in `templates/`.

```sh
# 1. Create the plan root
sh scripts/new-plan.sh <plan-slug>

# 2. Create each phase directory
sh scripts/new-phase.sh <plan-slug> <phase-number> <phase-slug>

# 3. Create each task file inside its phase
sh scripts/new-task.sh <plan-slug> <phase-number> <task-number> <task-slug>
```

Slugs are lowercase kebab-case summaries of the title. Examples:
- Plan "E-commerce Checkout Redesign" → `plan-ecommerce-checkout-redesign`
- Phase "Workspace Bootstrap" → `phase-01-workspace-bootstrap`
- Task P01T02 "Root package.json" → `task-P01T02-root-package-json.md`

After scaffolding, fill in the generated stub files following the structure defined
in `templates/plan.yaml`, `templates/phase.yaml`, and `templates/task.yaml`.

### Step 5 — Validate all output files

After writing all files, run the validation script to verify every file conforms
to its JSON Schema before reporting to the user.

```sh
sh scripts/validate-plan.sh <plan-slug>
```

The script checks each file against its schema in `schemas/`:

| File | Schema |
|---|---|
| `plan-<slug>/README.md` | `schemas/plan.schema.json` |
| `phases/phase-NN-<slug>/README.md` | `schemas/phase.schema.json` |
| `phases/phase-NN-<slug>/tasks/task-*.md` | `schemas/task.schema.json` |

If any file fails, fix it and re-run until all checks pass (exit 0) before
proceeding.

### Step 6 — Report to the user

After writing all files, output a summary:

```
Created implementation plan at .context/plans/plan-<slug>/

  README.md
  phases/phase-01-workspace-bootstrap/README.md  (N tasks)
  phases/phase-01-workspace-bootstrap/tasks/task-P01T01-*.md
  ...
```

## File formats

See `templates/` for the exact Markdown structure of each file type.

| Template | Schema | Purpose |
|---|---|---|
| `plan.yaml` | `schemas/plan.schema.json` | Root index structure |
| `phase.yaml` | `schemas/phase.schema.json` | Phase overview structure |
| `task.yaml` | `schemas/task.schema.json` | Individual task structure |

## Anti-patterns

**Do not** create tasks that span multiple unrelated files without a declared
reason — this makes tasks hard to verify independently.

**Do not** write vague gates like "works correctly". Gates must be runnable
commands or observable binary outcomes (exit 0 / non-zero, file exists, URL returns
200, etc.).

**Do not** embed implementation detail in `plan.md` — keep it as a navigation
index only. Detail belongs in phase and task files.

**Do not** number phases or tasks starting from 0. Use 1-based numbering (`01`,
`02`, ...) for consistent alphabetical sorting.

**Do not** skip the verification section in task files. Every task must be
provably complete.

## Notes

- Existing `.context/plans/` content is never deleted; new files are additive.
- If a plan already exists, append new phases rather than overwriting.
- Ask the user before creating more than 8 phases — very large plans often benefit
  from being split into separate planning documents.
