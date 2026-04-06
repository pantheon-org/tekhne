# Scenario 0: Full Decomposition Workflow

## Context

A developer has a large feature branch that mixes infrastructure changes, application
logic, and housekeeping. A reviewer flagged it as too big to review. The developer
wants help splitting it into smaller, reviewable PRs.

## Branch state

Current branch: `feature/PROJ-42-auth-service-migration`
Base branch: `main`

```
git log main..HEAD --oneline --reverse
```

Output:

```
a1b2c3 chore: update CDK stack to add new auth Lambda
d4e5f6 chore: add IAM role and policy for auth service
g7h8i9 feat: add JWT validation middleware
j0k1l2 feat: add user session handler
m3n4o5 feat: add token refresh endpoint
p6q7r8 test: unit tests for JWT validation middleware
s9t0u1 test: unit tests for session handler
v2w3x4 test: integration tests for token refresh
y5z6a7 chore: update package-lock.json
z8a9b0 chore: remove legacy basic-auth handler
```

Files changed per commit (abbreviated):

- `a1b2c3`: `infra/stacks/auth-stack.ts`
- `d4e5f6`: `infra/iam/auth-role.ts`, `infra/iam/auth-policy.ts`
- `g7h8i9`: `src/middleware/jwt-validation.ts`
- `j0k1l2`: `src/handlers/session.ts`
- `m3n4o5`: `src/endpoints/token-refresh.ts`
- `p6q7r8`: `src/middleware/jwt-validation.test.ts`
- `s9t0u1`: `src/handlers/session.test.ts`
- `v2w3x4`: `tests/integration/token-refresh.test.ts`
- `y5z6a7`: `package-lock.json`
- `z8a9b0`: `src/middleware/basic-auth.ts` (deleted)

## Task

Propose a decomposition of this branch into focused pull requests. Present your
proposal as a table and ask for approval before taking any action.
