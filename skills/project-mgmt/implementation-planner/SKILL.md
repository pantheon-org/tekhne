---
name: implementation-planner
description: >
  Converts a PRD or requirements document into a structured, phased implementation
  plan with individual phase files and granular per-task files written to
  .context/plans/. Also restructures existing monolithic planning documents into
  digestible, hierarchical directory structures. Creates a root plan index
  summarising all phases, a numbered phase file per phase, and a numbered task
  file per task inside each phase directory.
license: MIT
compatibility: opencode
metadata:
  version: 2.0.0
  audience: agents
  workflow: planning, project-management, task-breakdown, restructuring
---

# Implementation Planner

## What I do

**Mode 1 — Create new plan:** Given a PRD or requirements document, produce a
complete, navigable implementation plan written to `.context/plans/`:

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

**Mode 2 — Restructure existing plan:** Given a monolithic planning document or
flat set of phase files, produce a hierarchical directory structure under
`docs/refactoring/phases/` (or a path you specify):

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

## Trigger phrases

Use this skill when the user says any of the following (or close variants):

**Creating a new plan:**
- "create an implementation plan"
- "plan this out in phases"
- "break this PRD into tasks"
- "generate a phased plan"
- "write a detailed implementation plan"
- "create tasks from this spec"
- "decompose this into phases and tasks"

**Restructuring an existing plan:**
- "split this plan"
- "organize phases"
- "break down implementation docs"
- "create task hierarchy"
- "refactor this planning doc"
- "split this into separate files"

## Mode 1 — Create New Plan

### Inputs

| Input | Description |
|---|---|
| PRD / spec | A document, inline description, or file path describing what to build |
| Phase count | Optional — infer from scope if not provided |
| Output path | Optional — defaults to `.context/plans/` |

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

Use the identifier format `P{phase_number}T{task_number}`, both zero-padded.
Example: `P02T03` = phase 2, task 3.

### Step 4 — Scaffold and write output files

Use the scripts in `scripts/` to create the directory structure deterministically,
then populate the generated README / task files using the templates in `references/templates/`.

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

After scaffolding, fill in the generated stub files following the structure
defined in `references/templates/plan.yaml`, `references/templates/phase.yaml`, and `references/templates/task.yaml`.

### Step 5 — Validate all output files

After writing all files, run the validation script to verify every file conforms
to its JSON Schema before reporting to the user.

```sh
sh scripts/validate-plan.sh <plan-slug>
```

The script checks each file against its schema in `references/schemas/`:

| File | Schema |
|---|---|
| `plan-<slug>/README.md` | `references/schemas/plan.schema.json` |
| `phases/phase-NN-<slug>/README.md` | `references/schemas/phase.schema.json` |
| `phases/phase-NN-<slug>/tasks/task-*.md` | `references/schemas/task.schema.json` |

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

### File formats (Mode 1)

| Template | Schema | Purpose |
|---|---|---|
| `references/templates/plan.yaml` | `references/schemas/plan.schema.json` | Root index structure |
| `references/templates/phase.yaml` | `references/schemas/phase.schema.json` | Phase overview structure |
| `references/templates/task.yaml` | `references/schemas/task.schema.json` | Individual task structure |

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

**Handling legacy conventions:**
1. Extract the semantic meaning, not the literal text
2. Normalise to outcome-based names
3. Document the mapping for traceability
4. Keep original numbering for cross-referencing

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

Remove flat source files **only after** validation passes:

```sh
sh scripts/validate-structure.sh docs/refactoring/phases
# Exit 0 = valid, 1 = invalid
```

### Automation (Mode 2)

**Generate from JSON plan:**

```sh
sh scripts/generate-structure.sh --plan plan.json
sh scripts/generate-structure.sh --example   # show expected JSON format
```

JSON format:

```json
{
  "outputPath": "docs/refactoring/phases",
  "phases": [{
    "number": 1,
    "name": "bootstrap",
    "description": "...",
    "type": "activities",
    "items": [{
      "number": 1,
      "name": "setup-toolchain",
      "description": "...",
      "subItems": [{
        "number": 1,
        "name": "install-deps",
        "description": "...",
        "checklist": ["item 1"],
        "acceptanceCriteria": ["criterion 1"],
        "status": "pending",
        "dependencies": []
      }]
    }]
  }]
}
```

**Validate existing structure:**

```sh
sh scripts/validate-structure.sh <phases-dir>
```

Checks: READMEs present, descriptive names, required sections, valid links,
proper hierarchy depth, no numeric-only directory names.

### Validation checklist

Before marking complete:

- [ ] Every phase has its own directory
- [ ] Every directory has a `README.md`
- [ ] All step/activity directories have descriptive names
- [ ] Related items are grouped (`1.1`, `1.2` together; not `1.x` with `2.x`)
- [ ] All README links resolve correctly
- [ ] Old flat files removed
- [ ] Each leaf file has: title, description, checklist, acceptance criteria, status

### File formats (Mode 2)

| Template | Schema | Purpose |
|---|---|---|
| `references/templates/phase-readme.yaml` | `references/schemas/readme-file.schema.json` | Phase directory README |
| `references/templates/group-readme.yaml` | `references/schemas/readme-file.schema.json` | Group/intermediate README |
| `references/templates/intermediate-readme.yaml` | `references/schemas/readme-file.schema.json` | Activities/steps dir README |
| `references/templates/step-file.yaml` | `references/schemas/step-file.schema.json` | Leaf step/activity file |

See [references/example-transformation.md](references/example-transformation.md) for a before/after structure comparison.

### Error recovery

**Phase with 50+ items:**
1. Split by milestone — create `phase-1a-`, `phase-1b-` prefixes
2. Extract verticals — create feature-based phases if multiple features are mixed
3. Promote to phase — if a group of 10+ items is cohesive, give it its own phase
> Anti-pattern: just adding more nesting. Deep hierarchies with 50 siblings are still unusable.

**Conflicting naming conventions (mixed `step-X.Y` and `activity-X.Y`):**
1. Choose dominant pattern — whichever has more items, normalise to that
2. Phase-aware convention — use `steps/` for implementation phases, `activities/` for analysis phases
3. Document the hybrid — add a README note explaining the dual convention

**Partial split (abandoned mid-work):**
```sh
ls docs/refactoring/phases/*.md                          # flat files still in root
find docs/refactoring/phases -type d \
  -exec test ! -f {}/README.md \; -print                # dirs missing READMEs
git diff HEAD~5 --name-only | grep phases/              # recovery via git
```
Prioritise: complete partial splits before starting new ones.

**Items referencing non-existent parents:**
1. Renumber children to match actual parent
2. Create missing parent if children are cohesive
3. Flatten if only 1–2 children (they don't need a parent directory)

---

## Anti-patterns

### Mode 1

**Do not** create tasks that span multiple unrelated files without a declared
reason — this makes tasks hard to verify independently.

**Do not** write vague gates like "works correctly". Gates must be runnable
commands or observable binary outcomes (exit 0 / non-zero, file exists, URL
returns 200, etc.).

**Do not** embed implementation detail in `plan.md` — keep it as a navigation
index only. Detail belongs in phase and task files.

**Do not** number phases or tasks starting from 0. Use 1-based numbering (`01`,
`02`, ...) for consistent alphabetical sorting.

**Do not** skip the verification section in task files. Every task must be
provably complete.

### Mode 2

**Never use numeric-only directory names** (`step-1/`, `activity-2/`) — contributors
cannot navigate without opening every file.

**Never mix different parent groups** — `step-2.1` must not live inside `step-1-extract/`.

**Never create structures deeper than 4 levels** — flatten intermediate levels instead.

**Never skip README files** — every directory must explain its purpose.

**Never use generic directory names** (`step-1-stuff/`, `activity-1-things/`) — zero
information forces users to open files just to understand the context.

---

## Notes

- Existing `.context/plans/` content is never deleted; new files are additive.
- If a plan already exists, append new phases rather than overwriting.
- Ask the user before creating more than 8 phases — very large plans often benefit
  from being split into separate planning documents.
- When restructuring, create the new hierarchy first, validate, then remove the old
  flat files — never delete source before the new structure is confirmed valid.
