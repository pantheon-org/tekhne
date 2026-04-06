# Scenario 3: Cherry-Pick Conflict Handling and Post-Creation Checklist

## Context

The user approved the grouping proposal from Scenario 0 and the agent is now creating
the stacked branches. During cherry-pick, a conflict arises. The agent must handle
it correctly and then produce the correct post-creation reminder.

## Branch grouping (already approved)

| PR | Branch name | Commits |
|---|---|---|
| 1 | `feature/PROJ-42-add-auth-cdk-stack-and-iam` | `a1b2c3`, `d4e5f6` |
| 2 | `feature/PROJ-42-add-jwt-session-and-token-refresh` | `g7h8i9`, `j0k1l2`, `m3n4o5`, `p6q7r8`, `s9t0u1`, `v2w3x4` |
| 3 | `feature/PROJ-42-housekeeping` | `y5z6a7`, `z8a9b0` |

## Conflict scenario

While creating PR 2's branch and cherry-picking commit `g7h8i9`, a conflict occurs
in `src/middleware/jwt-validation.ts` because the file was modified in PR 1 as well.

## Task

1. Describe how to handle this cherry-pick conflict.
2. After all branches are created and conflicts resolved, produce the reminder message
   the user needs before opening MRs.
