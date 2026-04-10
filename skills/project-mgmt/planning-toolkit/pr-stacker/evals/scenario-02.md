# Scenario 02: Branch Naming Quality

## User Prompt

A developer has agreed to split their branch into three groups. They suggest using "wave" naming for the branches. Evaluate the proposed names and either accept or propose better alternatives with an explanation.

**Agreed grouping:**

| PR | Commits | Concern |
|---|---|---|
| 1 | `a1b2c3`, `d4e5f6` | Infrastructure (CDK stack + IAM role) |
| 2 | `g7h8i9`, `j0k1l2`, `m3n4o5`, `p6q7r8`, `s9t0u1`, `v2w3x4` | Application logic + tests |
| 3 | `y5z6a7`, `z8a9b0` | Housekeeping |

**Developer's suggested branch names:**

- PR 1: `feature/PROJ-42-wave-1`
- PR 2: `feature/PROJ-42-wave-2`
- PR 3: `feature/PROJ-42-wave-3`

## Expected Behavior

1. Explicitly state that names like `wave-1`, `wave-2`, `wave-3` are not acceptable — do not silently accept them or use them in any proposed output
2. Give a reason for rejecting sequence names (e.g. they don't convey scope, lose meaning after a rebase, or force reviewers to read every diff)
3. Propose replacement branch names that reflect what each PR changes (e.g. `add-auth-cdk-stack-and-iam`, `add-jwt-session-and-token-refresh`)
4. Provide a descriptive branch name for all three PRs (not just the first or second)
5. Include the ticket identifier `PROJ-42` as a prefix in all proposed branch names
6. Use lowercase letters and hyphens only — no spaces, underscores, camelCase, or uppercase letters
7. Make names sufficiently specific — not vague (e.g. `feature/PROJ-42-infra` is too vague; names should describe what specifically changes)
8. Do not execute git commands in this step — branch creation requires separate user confirmation
9. All three branch names follow the same naming convention (same prefix format, same case, same separator style)

## Success Criteria

- **Agent rejects the wave/sequence naming**: The agent explicitly states that names like `wave-1`, `wave-2`, `wave-3` are not acceptable
- **Agent explains why sequence names are problematic**: The agent gives a reason for rejecting sequence names
- **Proposed branch names describe actual content**: Each replacement branch name reflects what the PR changes; names are not generic or placeholder
- **All three PRs get replacement branch names**: The agent provides a descriptive branch name for all three PRs
- **Branch names retain the ticket prefix**: All proposed branch names include the ticket identifier `PROJ-42` as a prefix
- **Branch names are kebab-case with no spaces or underscores**: All proposed branch names use lowercase letters and hyphens only
- **Branch names are sufficiently specific**: Names describe what specifically changes, not just the concern category
- **Agent does not create branches as part of this response**: The agent proposes names but does not execute git commands
- **Proposed names are consistent in style**: All three branch names follow the same naming convention

## Failure Conditions

- Accepts the `wave-1`, `wave-2`, `wave-3` naming without objection
- Provides no reason for rejecting sequential names
- Proposes replacement names that are still too generic (e.g. `infra`, `app`, `cleanup`)
- Only proposes replacement names for one or two PRs
- Omits `PROJ-42` from one or more replacement branch names
- Uses spaces, underscores, or uppercase in replacement branch names
- Reports creating branches or executing git commands in this step
- Proposed names have inconsistent formatting across the three PRs
