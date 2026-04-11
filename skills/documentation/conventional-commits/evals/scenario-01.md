# Scenario 01: Commit Message Review and Correction

## User Prompt

A junior developer on your team has been submitting pull requests with poorly formatted git commit messages. The tech lead has asked you to review a batch of proposed commit messages and produce corrected versions for each one, along with a brief explanation of what was wrong.

The messages come from a mix of work: a new export feature added to the API, a fix for a race condition in the session manager, some test coverage added for the payment module, and a CI pipeline update for deployment automation.

Create a file called `commit-corrections.md` that contains:
- Each original message (quoted)
- The corrected commit message
- A one-sentence explanation of what was wrong

The following proposed messages need correction. Extract them before starting.

```text
=============== FILE: inputs/proposed-messages.txt ===============
1. Added new export endpoint for user data to the REST API
2. chore: fixed null pointer crash when session token is missing
3. Updated tests for payment processing to cover edge cases
4. CI pipeline changes for blue-green deployment
5. Refactored the authentication module by extracting token validation logic into a separate helper function and updating all callers
```

## Expected Behavior

1. Correct message #1 to use `feat` type for the new export endpoint
2. Correct message #2 to use `fix` type instead of `chore` for the null pointer crash
3. Correct message #3 to use `test` type for the test coverage update
4. Correct message #4 to use `ci` type for the CI pipeline changes
5. Correct message #5 to use `refactor` type for the authentication module restructuring
6. Rewrite all corrected headers using imperative mood (e.g., `add`, `fix`, `extract`) not past tense
7. Ensure no corrected header ends with a trailing period
8. Keep all corrected headers at 72 characters or fewer
9. Lowercase all description portions after the colon and space
10. Identify in the explanation for message #2 that `chore` was misused for a bug fix

## Success Criteria

- **feat for new feature**: The corrected message for the new export endpoint uses `feat` type (not `chore`, `build`, or another type)
- **fix for bug correction**: The corrected message for the null pointer crash uses `fix` type (not `chore`)
- **test for test addition**: The corrected message for the test coverage update uses `test` type
- **ci for pipeline update**: The corrected message for the CI pipeline changes uses `ci` type
- **refactor for restructuring**: The corrected message for the authentication module restructuring uses `refactor` type
- **Imperative mood**: All corrected headers use imperative mood (e.g., `add`, `fix`, `extract`) rather than past tense (`added`, `fixed`, `updated`)
- **No trailing period**: None of the corrected headers end with a period
- **Header <= 72 chars**: All corrected headers (type + scope + description) are 72 characters or fewer
- **Lowercase description**: All corrected description portions are lowercase (the text after the colon and space)
- **No chore misuse flagged**: The explanation for message #2 identifies that `chore` was used incorrectly (it was a bug fix, not maintenance)

## Failure Conditions

- Uses a type other than `feat` for the new export endpoint
- Keeps `chore` type on the null pointer crash fix instead of correcting it to `fix`
- Uses a type other than `test` for the test coverage addition
- Uses a type other than `ci` for the CI pipeline update
- Uses a type other than `refactor` for the authentication module restructuring
- Uses past tense verbs (`added`, `fixed`, `updated`) in any corrected header
- Any corrected header ends with a trailing period
- Any corrected header exceeds 72 characters
- Any description portion starts with an uppercase letter
- Fails to note in the explanation for message #2 that `chore` was the wrong type
