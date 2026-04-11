# Scenario 02: Add a New Phase to an Existing Implementation Plan

## User Prompt

An implementation plan already exists at `.context/plans/plan-blog-platform/`. The plan currently has two phases:

- `phases/phase-01-workspace-bootstrap/` (3 tasks — all complete)
- `phases/phase-02-data-model/` (4 tasks — all complete)

You need to extend this plan with a new phase covering the **REST API layer**. The API layer must implement:

1. `GET /posts` — list all posts (paginated, 10 per page)
2. `POST /posts` — create a new post (title, body, author required)
3. `GET /posts/:id` — get a single post by ID
4. `PUT /posts/:id` — update title/body of a post
5. `DELETE /posts/:id` — delete a post (soft delete — set `deleted_at`)
6. `GET /health` — health check returning `{"status":"ok"}`

Use Express.js. All handlers must be in separate files under `src/api/`.

Add this as phase 3 to the existing plan.

**Additive-only constraint:** The following files already exist and are read-only — they MUST NOT be modified, renamed, or deleted under any circumstances:

- `phases/phase-01-workspace-bootstrap/` and all files within it
- `phases/phase-02-data-model/` and all files within it

The root `README.md` may have a new phase-03 entry appended to its phase listing. No other edits to the root README are permitted.

Only new files may be created: the phase-03 directory, its README, and its task files.

## Expected Behavior

1. Create a new directory `phases/phase-03-<slug>/` inside the existing plan (not a new plan root)
2. Leave `phase-01` and `phase-02` directories and their contents completely unchanged
3. Include a concrete pass/fail shell-command gate in the phase-03 `README.md` (not vague language)
4. Create a number of task files appropriate for the 6 endpoints — ideally one task per handler file or closely related pair
5. Use `P03` prefix for all task identifiers (e.g. `task-P03T01-*.md`) with zero-padded 1-based numbering
6. Scope each task file to a single route handler file or closely related endpoints
7. Include a runnable verification command in every task file (e.g. `curl -s localhost:3000/posts | jq length`)
8. No task verification uses vague phrases like "the endpoint works" or "API responds correctly"
9. Explicitly declare dependencies on phase-02 (data model) and any required artefacts in the phase-03 `README.md`
10. Use `new-phase.sh` and `new-task.sh` scripts; run `validate-plan.sh` after adding the phase
11. Update the plan root `README.md` to include phase-03 in its phase listing (additive only — prior phases still listed)

## Success Criteria

- **phase-03-directory-created**: A new directory `phases/phase-03-<slug>/` was created inside the existing plan
- **existing-phases-untouched**: `phase-01` and `phase-02` directories and their contents were not modified, moved, or deleted
- **phase-readme-has-gate**: The phase-03 `README.md` includes a gate section with a concrete pass/fail shell command
- **task-count-matches-scope**: The number of task files in `phase-03/tasks/` is appropriate for the 6 endpoints described
- **task-ids-continue-sequence**: Task identifiers use `P03` prefix (e.g. `task-P03T01-*.md`) with zero-padded 1-based numbering
- **task-scoped-to-single-handler**: Each task file covers a single route handler file or closely related pair
- **task-verification-is-runnable**: Every task file includes a verification section with a specific shell command that exits 0 on success
- **no-vague-verification**: No task verification uses vague phrases without specifying the exact command and expected output
- **dependencies-declared**: The phase-03 `README.md` explicitly declares dependencies on phase-02 and any specific artefacts needed
- **scaffold-scripts-used**: Agent used `new-phase.sh` and `new-task.sh` scripts to create the phase and task files
- **validate-plan-run**: Agent ran `validate-plan.sh` after adding phase-03 and fixed any issues
- **root-readme-updated**: The plan root `README.md` was updated to include phase-03 in its phase listing
- **slug-format-correct**: All new directory names and file slugs use lowercase kebab-case
- **completion-summary-reported**: Agent reported a completion summary listing the new phase README and all new task files

## Failure Conditions

- Modifies, renames, or deletes existing `phase-01` or `phase-02` files
- Creates a new plan root instead of adding a phase to the existing plan
- Phase-03 `README.md` has a vague gate without a concrete shell command
- Bundles all 6 endpoints into a single task instead of scoping tasks per handler
- Task identifiers do not use the `P03` prefix or correct zero-padded numbering
- Task verification sections use vague language without concrete commands
- Phase-03 `README.md` does not declare dependency on phase-02
- Does not use scaffold scripts or `validate-plan.sh`
