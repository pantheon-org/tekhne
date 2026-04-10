# Scenario 01: Node.js CI Pipeline

## User Prompt

A small software consultancy maintains a Node.js 20 API service on GitHub. The team has recently had a security incident on a different project where a compromised third-party GitHub Action injected malicious code into their pipeline. Leadership now mandates that all new CI pipelines meet a higher security bar before being merged to the main branch.

The team wants a standard CI workflow for the API repository that runs on every push to `main` and on every pull request. The workflow should install dependencies, run the test suite, and produce a build artifact from the `dist/` directory. The project uses npm and a `package-lock.json` file is present.

## Output Specification

Produce a complete GitHub Actions workflow YAML file saved at `.github/workflows/ci.yml`. The workflow should be production-ready and follow current security and performance best practices.

## Expected Behavior

1. Pin `actions/checkout` (and all other actions) using a full 40-character SHA rather than a tag like `@v4`
2. Add an inline comment on each SHA-pinned action identifying the version (e.g., `# v4.x.x`)
3. Include a top-level `permissions:` block that sets `contents: read` to restrict default access
4. Add a `concurrency:` block with a `group:` key and `cancel-in-progress: true` to avoid wasted runs
5. Enable npm dependency caching via `actions/setup-node` with `cache: 'npm'` or an explicit `actions/cache` step
6. Set `timeout-minutes:` on at least one job and name the output workflow file with lowercase-hyphens

## Success Criteria

- **SHA-pinned checkout**: The `actions/checkout` step uses a full 40-character SHA (not a tag like `@v4` or a branch name) as its reference
- **Version comment on SHA**: The SHA-pinned `actions/checkout` step has an inline comment identifying the version (e.g., `# v4.x.x` or `# v5.x.x`)
- **Top-level permissions block**: The workflow file contains a top-level `permissions:` key (not only job-level) that restricts default access
- **Contents read default**: The top-level or job-level `permissions:` block sets `contents: read` (not `write-all` or absent)
- **Concurrency block present**: The workflow includes a `concurrency:` block with a `group:` and `cancel-in-progress: true`
- **Dependency caching enabled**: The `actions/setup-node` step (or an explicit `actions/cache` step) enables npm caching via the `cache: 'npm'` parameter or equivalent cache key using `package-lock.json`
- **Job timeout set**: At least one job has a `timeout-minutes:` field defined
- **Workflow file lowercase-hyphen**: The output file is named with lowercase letters and hyphens only (e.g., `ci.yml`, `ci-pipeline.yml`), not underscores or uppercase
- **No branch/latest reference**: No `uses:` step references an action via `@main`, `@master`, or `@latest`

## Failure Conditions

- Any action uses a tag reference (e.g., `@v4`) instead of a full 40-character SHA
- SHA-pinned actions have no inline version comment
- No top-level `permissions:` block is present in the workflow
- `contents: read` is absent from the permissions block
- No `concurrency:` block with `cancel-in-progress: true` is present
- npm dependency caching is not configured
- No job has a `timeout-minutes:` field
- Workflow file name contains uppercase letters or underscores
- Any action is referenced via `@main`, `@master`, or `@latest`
