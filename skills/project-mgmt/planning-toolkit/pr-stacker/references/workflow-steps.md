# Workflow Steps

## Step 1 — Analyse the branch

Read all commits between the current branch and the base branch:

```sh
git log <base>..<branch> --oneline --reverse
```

Then get files changed per commit:

```sh
git log <base>..<branch> --format='%h %s' | while read hash msg; do
  echo "=== $hash: $msg ==="
  git diff-tree --no-commit-id -r "$hash" --name-only
done
```

## Step 2 — Group commits by concern

Map each commit to one of these concern types:

| Concern | Typical files |
|---|---|
| **Infrastructure** | CDK stacks, Terraform, CloudFormation, VPC config, CI config |
| **Application logic** | Source handlers, business logic, shared modules |
| **Tests** | Unit tests, integration tests, fixtures, test data |
| **Housekeeping** | Dependencies, lock files, Makefile, generated files, tooling config |

A commit that touches both application logic and tests belongs to the group that
represents the majority of the change. Supporting commits (lock files, generated code)
go with the group they enable.

Adapt concern labels to the codebase — if there is no infrastructure, collapse to two
groups (application logic + tests). If the codebase uses a different taxonomy
(frontend/backend/data), use those labels instead. The table is a communication tool;
exact label names matter less than grouping accuracy.

## Step 3 — Propose the split

Present the proposed grouping as a table for user approval **before taking any action**:

| PR | Branch name | Commits | Concern |
|---|---|---|---|
| 1 | `feature/<ticket>-<what-changes>` | `abc123`, `def456` | Infrastructure |
| 2 | `feature/<ticket>-<what-changes>` | `789abc` | Application logic |
| 3 | `feature/<ticket>-<what-changes>` | `cde012`, `fgh345` | Housekeeping |

Wait for explicit confirmation before proceeding.

## Step 4 — Name branches descriptively

Branch names must reflect the actual content — not sequence numbers, wave numbers,
or phase numbers.

**Good:** `feature/PROJ-123-remove-cdk-glue-and-eligibility-table`
**Bad:** `feature/PROJ-123-wave-1`, `feature/PROJ-123-phase-1`, `feature/PROJ-123-part-1`

The name should let a reviewer understand the scope without reading the diff.

## Step 5 — Create stacked branches

PR 1 targets the base branch. Each subsequent PR targets the previous one:

```
<base> ← PR-1 ← PR-2 ← PR-3
```

Use cherry-pick to distribute commits:

```sh
# PR 1 — from base
git checkout -b feature/<ticket>-<name-1> <base>
git cherry-pick <sha1> <sha2> <sha3>

# PR 2 — from PR 1
git checkout -b feature/<ticket>-<name-2> feature/<ticket>-<name-1>
git cherry-pick <sha4> <sha5>

# PR 3 — from PR 2
git checkout -b feature/<ticket>-<name-3> feature/<ticket>-<name-2>
git cherry-pick <sha6> <sha7> <sha8>
```

Resolve any conflicts that arise from the reordering.

## Step 6 — Generate MR titles and descriptions

For each PR, produce a title and a short description that answers **What** and **Why**:

**Title format:** `<type>(<ticket>): <imperative summary of the change>`

**Description format:**

```
**What:** <one sentence summary of what changed — the diff in plain English>

**Why:** <one sentence on why this was necessary — motivation, consequence of not doing it>
```

The description is not a file list. It is not a commit log. It explains the change
to a reviewer who has no prior context.

## Step 7 — Remind about target branches

After creating branches, remind the user to set the correct target branch when opening
each MR:

- PR 1 → `<base branch>`
- PR 2 → PR 1's branch
- PR 3 → PR 2's branch

Merge in order.
