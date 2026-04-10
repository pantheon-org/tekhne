# Scenario 04: Write a CI Workflow Using nx affected

## User Prompt

You are setting up a GitHub Actions CI workflow for an Nx monorepo. The workflow must run lint, test, and build only on affected projects when a pull request is opened or updated, comparing against the base branch.

## Requirements

Produce a GitHub Actions workflow file at `.github/workflows/ci.yml` that:

1. Triggers on `pull_request` events targeting the `main` branch.
2. Runs on `ubuntu-latest`.
3. Checks out the repository with enough git history to compute the affected set (use `fetch-depth: 0`).
4. Sets up Node.js (version 20).
5. Installs dependencies (use `npm ci` or equivalent).
6. Runs affected lint, test, and build using `nx affected` with:
   - `--base=origin/main` and `--head=HEAD`
   - `--parallel=3`
   - Targets: `lint`, `test`, `build` (can be separate steps or combined with `-t lint,test,build`)
7. Does NOT use `nx run-many --all` anywhere in the workflow.

## Output

Produce the file `.github/workflows/ci.yml`.

## Expected Behavior

1. Configure the workflow to trigger on `pull_request` events targeting the `main` branch
2. Use `fetch-depth: 0` in the checkout step so `nx affected` can compute the correct base
3. Use `nx affected` (not `nx run-many --all`) for running tasks
4. Pass `--base=origin/main` and `--head=HEAD` to `nx affected`
5. Cover `lint`, `test`, and `build` targets for affected projects
6. Include a `--parallel` flag in the `nx affected` invocation

## Success Criteria

- **Triggers on pull_request to main**: The workflow triggers on `pull_request` events and targets the `main` branch.
- **fetch-depth: 0 for full git history**: The checkout step uses `fetch-depth: 0` so `nx affected` can compute the correct base.
- **nx affected used instead of run-many --all**: The workflow uses `nx affected` (not `nx run-many --all`) for running tasks.
- **Explicit --base=origin/main and --head=HEAD**: The `nx affected` invocation includes `--base=origin/main` and `--head=HEAD`.
- **lint, test, and build targets covered**: The workflow runs `lint`, `test`, and `build` for affected projects (in one or multiple steps).
- **--parallel flag present**: The `nx affected` invocation includes a `--parallel` flag (value 3 or any reasonable number).

## Failure Conditions

- Workflow does not trigger on `pull_request` or does not target the `main` branch
- Checkout step uses default `fetch-depth` (1), causing `nx affected` to fail to determine the affected set
- Workflow uses `nx run-many --all` instead of `nx affected`, running all projects regardless of changes
- `nx affected` invocation is missing `--base=origin/main` or `--head=HEAD`, producing incorrect affected sets
- Any of `lint`, `test`, or `build` is missing from the workflow
- `--parallel` flag is absent, causing slow sequential task execution
