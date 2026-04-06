# Anti-Patterns

## NEVER name branches with sequence numbers

**WHY:** Sequence names (wave-1, phase-2, part-1) become meaningless after any rebase
or reorder. A reviewer cannot determine scope without reading the full diff.

```sh
# BAD
feature/PROJ-123-wave-1
feature/PROJ-123-phase-2

# GOOD
feature/PROJ-123-remove-cdk-glue-table
feature/PROJ-123-add-eligibility-handler
```

**Consequence:** Future `git blame` and bisect queries return useless results.
Reviewers rubber-stamp because they cannot build a mental model from the branch name.

---

## NEVER create branches or cherry-pick before the user approves the grouping

**WHY:** Grouping decisions require domain knowledge the agent does not have.
A misassigned commit means deleting and recreating branches.

**Consequence:** Wasted cycles cleaning up wrong branches; cherry-pick conflicts
introduced by reordering commits incorrectly.

---

## NEVER split commits that share a single failing test

**WHY:** If PR 1 removes a handler and PR 2 adds its replacement, the stack has a
broken intermediate state. CI fails on PR 1 and blocks the merge queue.

```sh
# BAD — handler removed in PR 1, replacement added in PR 2
# PR 1 alone: tests fail on missing handler

# GOOD — removal and replacement travel together in one PR
```

**Consequence:** The stacked PR approach collapses — each PR must be independently
green before the next can merge.

---

## NEVER leave cherry-pick conflicts unresolved

**WHY:** A branch with conflicts is not buildable. The user discovers this only
when CI runs.

**Consequence:** The entire decomposition must restart. Partial branches pollute
the remote.

---

## NEVER write MR descriptions as file lists or commit logs

**WHY:** A reviewer can read the diff. A file list adds zero information. The
What/Why format gives context a reviewer without prior knowledge needs.

```
# BAD
- Updated auth-stack.ts
- Added auth-role.ts
- Modified session.ts

# GOOD
**What:** Provisions the CDK stack and IAM role that the new auth service requires.
**Why:** Without dedicated infrastructure, the service cannot deploy to any environment.
```

**Consequence:** Reviewers have no context for why the change exists and are more
likely to miss architectural issues.
