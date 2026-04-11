# Scenario 03: MR Description Generation

## User Prompt

Branches have been created. Generate MR titles and descriptions for two of the three PRs before opening them.

**PR 1 — `feature/PROJ-42-add-auth-cdk-stack-and-iam`**

Commits included:
- `a1b2c3 chore: update CDK stack to add new auth Lambda`
- `d4e5f6 chore: add IAM role and policy for auth service`

Files changed: `infra/stacks/auth-stack.ts`, `infra/iam/auth-role.ts`, `infra/iam/auth-policy.ts`

**PR 2 — `feature/PROJ-42-add-jwt-session-and-token-refresh`**

Commits included:
- `g7h8i9 feat: add JWT validation middleware`
- `j0k1l2 feat: add user session handler`
- `m3n4o5 feat: add token refresh endpoint`
- `p6q7r8 test: unit tests for JWT validation middleware`
- `s9t0u1 test: unit tests for session handler`
- `v2w3x4 test: integration tests for token refresh`

Files changed: `src/middleware/jwt-validation.ts`, `src/handlers/session.ts`, `src/endpoints/token-refresh.ts`, and corresponding test files.

**Context:** This work migrates authentication from a legacy basic-auth handler to a JWT-based service. PR 1 provisions the infrastructure that PR 2's application code will run on.

Generate an MR title and description for each PR. Use the What/Why format.

## Expected Behavior

1. Provide a title for each PR in `<type>(<ticket>): <imperative summary>` conventional commit format with a valid type and `PROJ-42` present
2. PR 1 provisions infrastructure (CDK stack, IAM) — use type `chore` or `feat`, not `fix` or `test`
3. PR 2 adds application logic and tests — use type `feat`, not `chore` or `fix`
4. Each description contains a clearly labelled **What:** field for both PR 1 and PR 2
5. Each description contains a clearly labelled **Why:** field for both PR 1 and PR 2
6. The What field summarises what changed in plain English — not a file list and not a commit log message
7. The Why field explains why this change was necessary or what would happen without it — does not restate the What field
8. The PR 1 description mentions that it provisions infrastructure that the application logic in PR 2 depends on
9. Neither description contains raw file paths, commit SHAs, or a changelog-style list of changes
10. Each What field is a single sentence; each Why field is a single sentence
11. A reviewer with no prior context could read the description and understand what changed and why

## Success Criteria

- **Both PRs have a title in conventional commit format**: Each title follows `<type>(<ticket>): <imperative summary>` with a valid type and `PROJ-42` present
- **PR 1 title uses an appropriate type**: PR 1 title type should be `chore` or `feat`, not `fix` or `test`
- **PR 2 title uses an appropriate type**: PR 2 title type should be `feat`, not `chore` or `fix`
- **Both PRs have a What field**: Each description contains a clearly labelled **What:** field
- **Both PRs have a Why field**: Each description contains a clearly labelled **Why:** field
- **What field describes the diff in plain English**: The What field summarises what changed in one sentence — not a file list and not a commit log message
- **Why field explains motivation or consequence**: The Why field explains why this change was necessary or what would happen without it
- **PR 1 description references its role as prerequisite for PR 2**: PR 1 description mentions that it provisions infrastructure the application logic in PR 2 depends on
- **Neither description lists files or commit hashes**: Neither MR description contains raw file paths, commit SHAs, or a changelog-style list
- **What and Why fields are each one sentence**: Each What field is a single sentence; each Why field is a single sentence
- **Descriptions are written for a reviewer with no prior context**: A reviewer unfamiliar with the codebase could understand what changed and why

## Failure Conditions

- Uses invalid conventional commit type or omits `PROJ-42` from title
- PR 1 uses `fix` or `test` as its commit type
- PR 2 uses `chore` or `fix` as its commit type
- Omits the What or Why field from either description
- What field is a list of files or commit messages instead of a plain-English summary
- Why field restates the What field instead of explaining motivation
- PR 1 description does not establish its prerequisite relationship with PR 2
- Descriptions include file paths, commit SHAs, or changelog-style inventories
- What or Why fields span multiple sentences instead of one
