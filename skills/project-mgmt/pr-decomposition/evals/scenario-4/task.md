# Scenario 4: Deciding Whether to Decompose

## Context

A developer refactored a single service class over several commits and now wants
help splitting the branch before opening a PR for review.

## Branch state

Current branch: `refactor/notification-service-overhaul`
Base branch: `main`

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

## Additional context

The developer mentions that `d7e8f9` updates tests that called `broadcastAll`,
which was removed in `c5d6e7`. If `c5d6e7` were cherry-picked to a PR without
`d7e8f9`, the test suite would fail.

## Task

The developer proposes splitting into:

- PR 1: `a1b2c3`, `b3c4d5` — helper extractions
- PR 2: `c5d6e7`, `d7e8f9` — removal + test update

Should the agent proceed with decomposition?
