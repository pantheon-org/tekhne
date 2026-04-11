# Scenario 04: Cherry-Pick Conflict Handling and Post-Creation Checklist

## User Prompt

The user approved the grouping proposal and the agent is now creating the stacked branches. During cherry-pick, a conflict arises. Handle it correctly and produce the correct post-creation reminder.

**Branch grouping (already approved):**

| PR | Branch name | Commits |
|---|---|---|
| 1 | `feature/PROJ-42-add-auth-cdk-stack-and-iam` | `a1b2c3`, `d4e5f6` |
| 2 | `feature/PROJ-42-add-jwt-session-and-token-refresh` | `g7h8i9`, `j0k1l2`, `m3n4o5`, `p6q7r8`, `s9t0u1`, `v2w3x4` |
| 3 | `feature/PROJ-42-housekeeping` | `y5z6a7`, `z8a9b0` |

**Conflict scenario:** While creating PR 2's branch and cherry-picking commit `g7h8i9`, a conflict occurs in `src/middleware/jwt-validation.ts` because the file was modified in PR 1 as well.

1. Describe how to handle this cherry-pick conflict.
2. After all branches are created and conflicts resolved, produce the reminder message the user needs before opening MRs.

## Expected Behavior

1. Do not report the branch as ready, the PR as openable, or the decomposition as complete while the cherry-pick conflict is still active
2. Provide concrete guidance: edit the conflicting file, stage it with `git add`, then run `git cherry-pick --continue` (or an equivalent correct resolution workflow)
3. After resolving the conflict, mention verifying the build (e.g. run tests, check compilation) before proceeding to the next cherry-pick
4. After all branches are created, output a clear reminder listing which target branch each PR should open against: PR 1 → main (or base), PR 2 → PR 1's branch, PR 3 → PR 2's branch
5. The reminder covers all three PRs with their correct targets explicitly named
6. Treat the cherry-pick conflict as a normal part of the process — do not recommend reverting to a single PR or abandoning the stack
7. Explain that the conflict happened because the same file was modified in an earlier commit now in PR 1

## Success Criteria

- **Agent does not report completion while conflict is unresolved**: The agent does not say the branch is ready, the PR can be opened, or the decomposition is complete while the cherry-pick conflict is still active
- **Agent instructs how to resolve the conflict**: The agent provides concrete guidance: edit the conflicting file, stage it with `git add`, then run `git cherry-pick --continue` (or equivalent)
- **Agent verifies the branch is in a buildable state after resolution**: After resolving the conflict, the agent mentions verifying the build before proceeding to the next cherry-pick
- **Agent produces the target-branch reminder after all branches are created**: The agent outputs a clear reminder listing which target branch each PR should open against
- **Reminder lists all three PRs with their correct targets**: The reminder covers all three PRs, not just the first or last; each PR's correct target branch is explicitly named
- **Agent does not suggest abandoning the decomposition due to the conflict**: The agent treats the cherry-pick conflict as normal and expected, not a reason to revert to a single PR
- **Agent explains why the conflict occurred**: The agent explains that the conflict happened because the same file was modified in an earlier commit now in PR 1

## Failure Conditions

- Reports the branch as ready or the PR as openable while the conflict is still active
- Does not provide concrete conflict resolution steps (`git add`, `git cherry-pick --continue`)
- Does not mention verifying the build after resolving the conflict
- Does not produce a target-branch reminder after all branches are created
- The reminder omits one or more PRs or has incorrect target branches
- Recommends abandoning the decomposition or reverting to a single PR due to the conflict
- Does not explain why the conflict occurred
