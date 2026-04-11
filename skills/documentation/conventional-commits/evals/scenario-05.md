# Scenario 05: Create a Commit Log for a Release Preparation

## User Prompt

A startup's backend team is preparing a v2.3.0 release. Over the past two weeks, five changes were merged to the main branch, but the engineer who merged them used informal descriptions in the merge commit messages. The release manager needs proper commit messages for each change to feed into their semantic-release pipeline, which generates the CHANGELOG automatically.

The five changes and their informal descriptions are:

1. **"auth stuff"** — Added support for multi-factor authentication via TOTP. Users can now enroll in MFA from their account settings page. This is a new, optional feature; no existing flows are disrupted.

2. **"misc updates"** — Bumped eslint from 8.x to 9.x, updated the prettier config to use the new flat config format, and removed the deprecated `@babel/eslint-parser` plugin. No application logic changed.

3. **"fixed that login thing"** — The session cookie was not being marked `HttpOnly` and `Secure` when the app was deployed behind a reverse proxy. This caused sessions to be accessible via JavaScript in some environments.

4. **"docs"** — Updated the `README.md` quickstart section to reflect the new Docker Compose setup introduced in v2.2.0. No code changes.

5. **"db migration helper"** — Added a `migrate:rollback` script to `package.json` that wraps the existing Knex rollback command with environment validation, preventing accidental rollbacks in production.

Write a proper commit message for each of the five changes.

Create a file called `commit-messages.txt` with all five commit messages, separated by `---` lines.

## Expected Behavior

1. Use `feat` type for change #1 (multi-factor authentication — new optional feature)
2. Use `build` or `style` type (NOT `chore`) for change #2 (eslint/prettier tooling upgrade)
3. Use `fix` type for change #3 (session cookie security bug)
4. Use `docs` type for change #4 (README update)
5. Use a semantically precise type for change #5 (migrate:rollback script) — `build`, `feat`, or `chore` only if no better type applies
6. Use imperative mood in all five headers (`add`, `upgrade`, `fix`, `update`)
7. Keep all five headers at 72 characters or fewer
8. Ensure no header ends with a trailing period
9. Start all five description portions with a lowercase letter
10. Keep specific implementation details (package version numbers, file names) out of the headers

## Success Criteria

- **feat for MFA**: Change #1 (multi-factor authentication) uses `feat` type
- **build for tooling updates**: Change #2 (eslint/prettier upgrade) uses `build` or `style` type, NOT `chore`
- **fix for session bug**: Change #3 (session cookie security) uses `fix` type
- **docs for README**: Change #4 (README update) uses `docs` type
- **Appropriate type for script**: Change #5 (migrate:rollback script) uses a semantically precise type such as `build`, `feat`, or `chore` — if `chore`, there is no better-fitting type that applies
- **Imperative mood all five**: All five headers use imperative mood (`add`, `upgrade`, `fix`, `update`) not past tense
- **Header <= 72 chars all five**: All five headers are 72 characters or fewer
- **No trailing periods**: None of the five headers end with a period
- **Lowercase descriptions**: All five description portions begin with a lowercase letter
- **No implementation details in headers**: None of the five headers include specific implementation details (e.g., specific package version numbers, specific file names)

## Failure Conditions

- Uses a type other than `feat` for the MFA addition
- Uses `chore` for the eslint/prettier tooling upgrade when `build` or `style` is more appropriate
- Uses a type other than `fix` for the session cookie security bug
- Uses a type other than `docs` for the README update
- Uses a vague or catch-all type without justification for the rollback script
- Any header uses past tense (e.g., `added`, `upgraded`, `fixed`)
- Any header exceeds 72 characters
- Any header ends with a trailing period
- Any description portion starts with an uppercase letter
- Any header mentions specific file names or package version numbers
