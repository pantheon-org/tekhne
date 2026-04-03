# Worked Example

## Input: 9 commits on `feature/PROJ-123-data-pipeline-refactor`

```
a1b2c3 chore: update CDK stack for new Glue job
d4e5f6 chore: remove eligibility table construct
g7h8i9 feat: add eligibility data handler
j0k1l2 feat: add pipeline orchestrator module
m3n4o5 test: add unit tests for eligibility handler
p6q7r8 test: add integration tests for orchestrator
s9t0u1 chore: update lock file
v2w3x4 chore: remove unused Makefile targets
y5z6a7 docs: update README with new pipeline diagram
```

## Step 2 grouping output

| PR | Branch name | Commits | Concern |
|---|---|---|---|
| 1 | `feature/PROJ-123-remove-cdk-glue-eligibility-table` | `a1b2c3`, `d4e5f6`, `s9t0u1` | Infrastructure |
| 2 | `feature/PROJ-123-add-eligibility-and-orchestrator` | `g7h8i9`, `j0k1l2`, `m3n4o5`, `p6q7r8` | Application logic + tests |
| 3 | `feature/PROJ-123-housekeeping-and-docs` | `v2w3x4`, `y5z6a7` | Housekeeping |

Notes on grouping decisions:
- `s9t0u1` (lock file) goes with the infra group — it was caused by the CDK dependency change
- Tests travel with the source they cover (`m3n4o5` with `g7h8i9`, `p6q7r8` with `j0k1l2`)
- README update goes with housekeeping since it documents the removed Glue job

## Step 6 MR description for PR 2

**Title:** `feat(PROJ-123): add eligibility data handler and pipeline orchestrator`

**Description:**

```
**What:** Adds the eligibility handler and orchestrator modules that replace
the removed Glue job, including unit and integration test coverage.

**Why:** The Glue job was removed in PR 1; this introduces the replacement
application-layer logic that processes eligibility data via the new pipeline.
```

## Step 5 cherry-pick commands

```sh
# PR 1 — from main
git checkout -b feature/PROJ-123-remove-cdk-glue-eligibility-table main
git cherry-pick a1b2c3 d4e5f6 s9t0u1

# PR 2 — from PR 1
git checkout -b feature/PROJ-123-add-eligibility-and-orchestrator \
  feature/PROJ-123-remove-cdk-glue-eligibility-table
git cherry-pick g7h8i9 j0k1l2 m3n4o5 p6q7r8

# PR 3 — from PR 2
git checkout -b feature/PROJ-123-housekeeping-and-docs \
  feature/PROJ-123-add-eligibility-and-orchestrator
git cherry-pick v2w3x4 y5z6a7
```
