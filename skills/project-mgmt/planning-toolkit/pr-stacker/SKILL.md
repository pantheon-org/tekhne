---
name: pr-stacker
description: "Splits a large feature branch into smaller, focused pull requests using stacked branches and cherry-pick. Groups commits by concern (infrastructure, application logic, tests, housekeeping), proposes descriptive branch names for user approval, creates stacked branches, and generates What/Why MR titles and descriptions. Use when the user says a PR is too big, asks to split a PR, wants to decompose a branch, or needs to break work into reviewable chunks."
license: MIT
compatibility: opencode
metadata:
  version: 1.0.0
  audience: agents
  workflow: git, pull-requests, code-review, branching
---

# PR Stacker

## Mindset

A large PR fails reviewers because it mixes concerns — infrastructure sits next to
business logic sits next to test cleanup. Reviewers cannot build a mental model of
any single change when everything changes at once.

The fix is to group commits by **what kind of thing changed**, not by when they were
written or what ticket they belong to. Each group becomes its own PR targeting the
previous one (stacked branches), so changes can be reviewed and merged in order
without blocking unrelated work.

Two failure modes to avoid:

1. **The monolith** — one PR with everything, reviewers give up or rubber-stamp
2. **The over-split** — one PR per commit, stacked 10 deep, merge overhead exceeds the benefit

The sweet spot is 2–4 PRs where each one can be understood by a reviewer who only
knows that domain.

## When to use

- A PR has more than ~15 files changed across unrelated concerns
- A reviewer says "this is too big to review"
- Commits already have phase or step markers in their messages
- Changes span infrastructure, application code, and cleanup in a single branch

## When not to use

- The branch has 2–4 commits that all touch the same component — keep it together
- All files are test files for a single feature — one PR is correct
- The work is already in review and splitting restarts the cycle with more overhead than benefit
- Commits are tightly coupled (each depends on the previous for tests to pass) — cherry-pick reordering will produce broken intermediate branches

## Trigger phrases

- "This PR is too big"
- "Split this PR"
- "Break this into smaller PRs"
- "Decompose the branch"
- "Too many changes for reviewers"

## Workflow

Load [Workflow Steps](references/workflow-steps.md) for full commands. Summary:
analyse → group → propose (table, wait for approval) → name → stack (cherry-pick) →
describe (What/Why) → remind about target branches.

Consider adapting concern labels to the codebase — if there is no infrastructure layer,
you may optionally collapse to two groups (application logic + tests).

```bash
# Step 1 — read commits and files changed before grouping
git log <base>..<branch> --oneline --reverse
git diff-tree --no-commit-id -r <sha> --name-only
```

## Anti-patterns

❌ **NEVER name branches with sequence numbers (wave-1, phase-2, part-1)**

WHY: Sequence names lose meaning after any rebase or reorder. A reviewer cannot
determine scope without reading the full diff.

```bash
# BAD — reviewer cannot tell what changes without reading the diff
feature/PROJ-123-wave-1
feature/PROJ-123-phase-2

# GOOD — scope is clear from the name alone
feature/PROJ-123-add-cdk-stack-and-iam
feature/PROJ-123-add-jwt-handler-and-tests
```

**Consequence:** Reviewers rubber-stamp because they cannot build a mental model.

---

❌ **NEVER create branches or cherry-pick before the user approves the grouping**

WHY: Grouping requires domain knowledge the agent does not have. A misassigned commit
means deleting and recreating branches.

**Consequence:** Wasted cycles cleaning up wrong branches; conflicts from reordering.

---

❌ **NEVER split commits that share a single failing test (broken intermediate state)**

WHY: If PR 1 removes a handler and PR 2 adds its replacement, PR 1 alone fails CI.

**Consequence:** The merge queue blocks. The entire decomposition must restart.

---

❌ **NEVER write MR descriptions as file lists or commit logs**

WHY: A reviewer can already read the diff. The What/Why format gives the context that
a reviewer without prior knowledge actually needs.

```bash
# BAD — no context, reviewer learns nothing
# - Updated auth-stack.ts
# - Added auth-role.ts

# GOOD — answers what changed and why it was necessary
# **What:** Provisions the CDK stack and IAM role the new auth service requires.
# **Why:** Without dedicated infrastructure, the service cannot deploy to any environment.
```

**Consequence:** Reviewers miss architectural intent and are more likely to approve
problematic changes.

---

❌ **NEVER leave cherry-pick conflicts unresolved before reporting completion**

WHY: A branch with conflicts is not buildable. The user discovers this only when CI runs.

ALWAYS resolve every conflict and verify the branch compiles before reporting success.

**Consequence:** The entire decomposition must restart. Partial branches pollute the remote.

## Example proposal

Given commits touching infra (CDK, IAM), application logic (handlers + tests), and
housekeeping (lock file, Makefile), the grouping table looks like:

| PR | Branch | Concern |
|---|---|---|
| 1 | `feature/PROJ-123-add-cdk-stack-and-iam` | Infrastructure |
| 2 | `feature/PROJ-123-add-handlers-and-tests` | Application logic + tests |
| 3 | `feature/PROJ-123-housekeeping` | Housekeeping |

See [Worked Example](references/worked-example.md) for the complete walkthrough.

## References

- [Workflow Steps](references/workflow-steps.md) — Full step-by-step git commands; load when executing a decomposition
- [Anti-Patterns](references/anti-patterns.md) — NEVER/WHY/Consequence patterns with BAD/GOOD examples; load when explaining a failure mode
- [Worked Example](references/worked-example.md) — Complete 9-commit walkthrough; load when verifying output format
