# Scenario 04: Integrate cdk-nag into a CI/CD Pipeline

## User Prompt

A team has cdk-nag working locally but their GitHub Actions CI pipeline does not run it. They currently only run tests in CI, and plan to add cdk-nag checks as a pre-deployment gate just before deploying to production.

Their current `.github/workflows/ci.yml`:

```yaml
name: CI
on: [push, pull_request]
jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm ci
      - run: npm test
```

They also have a separate `deploy.yml` workflow that runs only on main branch pushes and calls `npx cdk deploy`. They planned to add cdk-nag checks as a step in `deploy.yml`.

Explain why running cdk-nag only at deployment time is wrong, and produce an updated `ci.yml` that runs cdk-nag checks on every push and pull request.

## Expected Behavior

1. Explain that running cdk-nag only at deploy time means security defects are found too late, causing expensive rework
2. Add `npx cdk synth` as a step that runs on every push and pull request in `ci.yml`
3. Place the `cdk synth` step after `npm ci` (dependencies installed) and after `npm test`
4. Ensure the workflow triggers on `pull_request` events, not only on push to main
5. Structure the workflow so both tests and synth must pass (sequential steps or separate blocking steps)

## Success Criteria

- **Late-stage-only enforcement explained as wrong**: The response explains that running cdk-nag only at deploy time means security defects are found too late, causing expensive rework
- **npx cdk synth added to ci.yml**: The updated ci.yml includes npx cdk synth as a step that runs on every push and pull request
- **Synth step runs after npm ci and npm test**: The cdk synth step appears after dependencies are installed and after tests run
- **CI triggers on pull_request (not just main push)**: The workflow runs on pull_request events, not only on push to main
- **npm test && npx cdk synth pattern or equivalent**: The workflow combines test and synth so both must pass (sequential or separate steps that both block merge)

## Failure Conditions

- Does not explain why running cdk-nag only at deploy time is problematic
- `npx cdk synth` is added to `deploy.yml` instead of `ci.yml`
- `cdk synth` step runs before `npm ci` or before `npm test`
- Workflow does not trigger on `pull_request` events
- `npm test` and `npx cdk synth` are structured so only one must pass (e.g., using `||`)
