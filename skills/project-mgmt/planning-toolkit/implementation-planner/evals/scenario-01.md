# Scenario 01: Create an Implementation Plan for a URL Shortener Service

## User Prompt

Create a complete implementation plan for the following project.

## Requirements

Build a URL Shortener web service with the following capabilities:

- Accept a long URL via a REST API and return a short code
- Redirect short codes to their original URLs via HTTP 301/302
- Track click counts per short code
- Persist data in SQLite
- Expose a health check endpoint
- Provide a minimal HTML form for manual testing

## Constraints

- Runtime: Node.js with TypeScript
- No external authentication required
- Short codes must be URL-safe (alphanumeric only)
- The service must start with `npm start`
- All tests must run with `npm test`

Create a complete implementation plan under `.context/plans/` for this project. The plan should cover all work from bootstrapping the project through to a production-ready release.

## Expected Behavior

1. Create a plan root directory at `.context/plans/plan-<slug>/` with a `README.md` navigation index
2. The root `README.md` lists phases and links to them, containing no implementation details or per-task instructions
3. Create a `phases/` subdirectory with at least one numbered phase directory (e.g. `phases/phase-01-<slug>/`)
4. Every phase `README.md` contains a clear goal statement, a concrete pass/fail shell-command gate, and explicit dependencies on prior phases
5. Each phase directory contains a `tasks/` subdirectory with at least one task file
6. All task filenames follow `task-P{NN}T{NN}-<slug>.md` with zero-padded, 1-based numbering
7. Every task file includes a verification section with a provable, runnable check (shell command that exits 0/non-zero, file exists check, HTTP status check, etc.)
8. Use `new-plan.sh`, `new-phase.sh`, and `new-task.sh` scripts to scaffold the directory structure; run `validate-plan.sh` after writing all files
9. Output a structured completion summary listing the root README, each phase README with task count, and all task file paths
10. Keep the plan between 2 and 8 phases appropriate for the project scope

## Success Criteria

- **plan-root-created**: A plan root directory exists at `.context/plans/plan-<slug>/` with a `README.md` file
- **root-readme-is-index-only**: The root `README.md` acts as a navigation index — it lists phases and links to them but does not contain implementation details, code, or per-task instructions
- **phases-directory-structure**: A `phases/` subdirectory exists containing at least one numbered phase directory (e.g. `phases/phase-01-<slug>/`)
- **phase-readme-has-goal-gate-deps**: Every phase `README.md` contains a clear goal statement, a pass/fail gate that can be verified by running a shell command, and explicit dependencies on prior phases or artefacts
- **gate-is-concrete-not-vague**: No phase gate uses vague language like "works correctly" or "tests pass" without specifying the exact command to run
- **tasks-directory-exists-per-phase**: Each phase directory contains a `tasks/` subdirectory with at least one task file
- **task-identifier-format-correct**: All task filenames follow the pattern `task-P{NN}T{NN}-<slug>.md` with zero-padded, 1-based numbering
- **task-files-have-verification-section**: Every task file includes a verification section with a provable, runnable check
- **task-verification-not-vague**: No task verification section uses vague language — each verification describes a specific command or observable output
- **tasks-scoped-to-single-file**: Task files are scoped to a single file or a tightly coupled set of files
- **slug-format-correct**: All directory names and file slugs use lowercase kebab-case
- **scripts-used-for-scaffolding**: The agent invoked `new-plan.sh`, `new-phase.sh`, and `new-task.sh` scripts to scaffold the directory structure
- **validate-plan-script-run**: The agent ran `validate-plan.sh <plan-slug>` after writing all files and addressed any reported failures
- **completion-summary-reported**: Agent output a structured completion summary listing the root README, each phase README with its task count, and all individual task file paths
- **additive-no-deletion**: The agent did not delete or overwrite any pre-existing files in `.context/plans/`
- **reasonable-phase-count**: The plan contains between 2 and 8 phases appropriate for the project scope

## Failure Conditions

- Creates plan files without a root `README.md` navigation index
- Root `README.md` contains implementation details instead of acting as an index
- Does not create a `phases/` subdirectory structure
- Phase `README.md` files lack goal statements, shell-command gates, or dependency declarations
- Phase gates use vague language without specifying the exact command
- Task files are missing, or do not have their own `tasks/` subdirectory per phase
- Task filenames do not follow the `task-P{NN}T{NN}-<slug>.md` pattern
- Task verification sections use vague language without specifying a concrete runnable check
- Creates the directory structure manually instead of using scaffold scripts
- Does not run `validate-plan.sh` after creating the plan
- Creates more than 8 phases without asking the user first
