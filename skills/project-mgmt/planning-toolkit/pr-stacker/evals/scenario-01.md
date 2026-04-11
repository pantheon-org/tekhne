# Scenario 01: Full Decomposition Workflow

## User Prompt

A developer has a large feature branch that mixes infrastructure changes, application logic, and housekeeping. A reviewer flagged it as too big to review. The developer wants help splitting it into smaller, reviewable PRs.

**Current branch:** `feature/PROJ-42-auth-service-migration`
**Base branch:** `main`

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

Propose a decomposition of this branch into focused pull requests. Present your proposal as a table and ask for approval before taking any action.

## Expected Behavior

1. Demonstrate awareness of which files each commit touches before assigning commits to concern groups — do not guess, reference the file list
2. Propose between 2 and 4 PR groups (not one monolith and not more than 4 over-split groups)
3. Place commits `a1b2c3` and `d4e5f6` (CDK stack, IAM role) together in an infrastructure group, not mixed into the application logic group
4. Place test commits (`p6q7r8`, `s9t0u1`, `v2w3x4`) in the same group as the source they test (`g7h8i9`, `j0k1l2`, `m3n4o5`) — not in a separate tests-only PR
5. Place the lock file commit `y5z6a7` with the group it supports, not floating in a separate group
6. Output the proposal as a Markdown table with at minimum PR number, branch name, commits, and concern columns
7. Propose descriptive branch names — not `wave-N`, `phase-N`, `part-N`, or `step-N`; each name reflects what the PR changes
8. Include `PROJ-42` as the ticket identifier prefix in all proposed branch names
9. End the response with a clear request for confirmation — do not unilaterally move to branch creation
10. Do not report creating branches, running cherry-pick, or executing any git command — the response is a proposal only
11. Indicate that PR 1 targets the base branch and subsequent PRs target the previous one (stacked structure)

## Success Criteria

- **Commit list and files are analysed before grouping**: The agent demonstrates awareness of which files each commit touches before assigning commits to concern groups
- **Commits are grouped into 2-4 concern buckets**: The proposal contains between 2 and 4 PR groups
- **Infrastructure commits are correctly separated**: Commits `a1b2c3` and `d4e5f6` (CDK stack, IAM role) appear together in an infrastructure group
- **Tests travel with the application logic they cover**: Test commits are placed in the same group as the source they test
- **Supporting commits go with the group they enable**: The lock file commit `y5z6a7` is placed with the group it supports
- **Proposal is presented as a Markdown table**: The agent outputs a markdown table with PR number, branch name, commits, and concern columns
- **Proposed branch names are descriptive, not sequential**: No branch name uses `wave-N`, `phase-N`, `part-N`, or `step-N`
- **Branch names include the ticket prefix**: All proposed branch names include `PROJ-42` as the ticket identifier prefix
- **Agent explicitly asks for user approval before proceeding**: The agent ends its response with a clear request for confirmation
- **No git commands are described as already executed**: The agent does not report creating branches, running cherry-pick, or executing any git command
- **Stacked branch structure is referenced or implied**: The proposal indicates PR 1 targets the base branch and subsequent PRs target the previous one

## Failure Conditions

- Groups commits without referencing which files they touch
- Proposes only 1 group (monolith) or more than 4 groups (over-split)
- Puts infrastructure commits in the same group as application logic
- Separates test commits into a standalone tests-only PR
- Leaves the lock file floating in its own group
- Presents the proposal as prose instead of a Markdown table
- Uses sequential naming (wave-1, phase-1, part-1) for branch names
- Omits `PROJ-42` prefix from proposed branch names
- Proceeds to create branches without asking for approval
- Reports git commands as already executed
