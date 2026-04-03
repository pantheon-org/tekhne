# Scenario 2: MR Description Generation

## Context

Branches have been created. The developer now needs MR titles and descriptions
for two of the three PRs before opening them.

## Branch details

### PR 1 — `feature/PROJ-42-add-auth-cdk-stack-and-iam`

Commits included:
- `a1b2c3 chore: update CDK stack to add new auth Lambda`
- `d4e5f6 chore: add IAM role and policy for auth service`

Files changed: `infra/stacks/auth-stack.ts`, `infra/iam/auth-role.ts`,
`infra/iam/auth-policy.ts`

### PR 2 — `feature/PROJ-42-add-jwt-session-and-token-refresh`

Commits included:
- `g7h8i9 feat: add JWT validation middleware`
- `j0k1l2 feat: add user session handler`
- `m3n4o5 feat: add token refresh endpoint`
- `p6q7r8 test: unit tests for JWT validation middleware`
- `s9t0u1 test: unit tests for session handler`
- `v2w3x4 test: integration tests for token refresh`

Files changed: `src/middleware/jwt-validation.ts`, `src/handlers/session.ts`,
`src/endpoints/token-refresh.ts`, and corresponding test files.

## Context

This work migrates authentication from a legacy basic-auth handler to a JWT-based
service. PR 1 provisions the infrastructure that PR 2's application code will run on.

## Task

Generate an MR title and description for each PR. Use the What/Why format.
