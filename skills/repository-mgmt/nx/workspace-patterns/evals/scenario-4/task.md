# Task: Write a CI workflow using nx affected

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
