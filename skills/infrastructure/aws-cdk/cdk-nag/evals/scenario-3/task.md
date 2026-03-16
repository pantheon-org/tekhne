# Integrate cdk-nag into a CI/CD Pipeline

## Problem Description

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
