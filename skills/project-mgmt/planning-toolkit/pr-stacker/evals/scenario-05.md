# Scenario 05: Deciding Whether to Decompose

## User Prompt

A developer refactored a single service class over several commits and now wants help splitting the branch before opening a PR for review.

**Current branch:** `refactor/notification-service-overhaul`
**Base branch:** `main`

```
git log main..HEAD --oneline --reverse
```

Output:

```
a1b2c3 refactor: extract sendEmail into EmailDispatcher helper
b3c4d5 refactor: extract sendSms into SmsDispatcher helper
c5d6e7 refactor: remove deprecated broadcastAll method from NotificationService
d7e8f9 test: update NotificationService unit tests after broadcastAll removal
```

Files changed per commit:

- `a1b2c3`: `src/services/NotificationService.ts`
- `b3c4d5`: `src/services/NotificationService.ts`
- `c5d6e7`: `src/services/NotificationService.ts`
- `d7e8f9`: `src/services/NotificationService.test.ts`

**Additional context:** The developer mentions that `d7e8f9` updates tests that called `broadcastAll`, which was removed in `c5d6e7`. If `c5d6e7` were cherry-picked to a PR without `d7e8f9`, the test suite would fail.

The developer proposes splitting into:

- PR 1: `a1b2c3`, `b3c4d5` — helper extractions
- PR 2: `c5d6e7`, `d7e8f9` — removal + test update

Should the agent proceed with decomposition?

## Expected Behavior

1. Clearly state that it will not proceed with decomposing the branch — not just hedge or ask for more information
2. Identify that all commits touch the same component (NotificationService) and explain this makes decomposition add friction without value
3. Identify that splitting `c5d6e7` (removal) from `d7e8f9` (test update) would leave a PR with a failing test suite
4. Not produce a decomposition proposal table or branch plan — decline before that stage
5. Suggest a constructive path forward: open as a single focused PR, or ask reviewers to review commit-by-commit using the git log

## Success Criteria

- **explicitly-declines-decomposition**: Agent clearly states it will not proceed with decomposing the branch, not just hedges or asks for more information
- **cites-same-component-reason**: Agent identifies that all commits touch the same component (NotificationService) and explains this makes decomposition add friction without value
- **cites-failing-test-reason**: Agent identifies that splitting `c5d6e7` (removal) from `d7e8f9` (test update) would leave a PR with a failing test suite
- **no-grouping-table-produced**: Agent does not produce a decomposition proposal table or branch plan — it declines before that stage
- **offers-constructive-alternative**: Agent suggests a constructive path forward: open as a single focused PR, or ask reviewers to review commit-by-commit using the git log

## Failure Conditions

- Proceeds to create a decomposition proposal without declining first
- Hedges or asks for clarification instead of making a clear decision
- Does not identify the same-component reason for declining
- Does not identify the failing-test risk from the proposed split
- Produces a grouping table or branch plan despite the conflicts
- Declines without offering any constructive alternative
