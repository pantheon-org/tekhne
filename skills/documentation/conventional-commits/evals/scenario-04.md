# Scenario 04: Write Commit Messages for a Bug Fix Sprint

## User Prompt

Your team just closed a two-day bug sprint. Three issues were resolved and are ready to be committed to the main branch. The project uses automated changelog generation and GitHub's issue-closing keywords.

Here are the three resolved issues and the changes made:

**Issue #301 — Double submission on checkout form**
A race condition allowed users to double-click the "Place Order" button during network latency, resulting in duplicate charges. The fix debounces the submit handler with a 300ms window and disables the button immediately on first click.

**Issue #178 — Stale product prices shown after update**
Product prices cached in the session store were not invalidated when an admin updated prices in the dashboard. The fix clears the relevant cache keys whenever the product catalogue endpoint detects a write operation.

**Issue #412 — Wrong HTTP status on validation failure**
The registration endpoint was returning HTTP 500 for invalid email addresses rather than HTTP 422. The endpoint now validates input before attempting database insertion and returns the correct status with a descriptive error body.

Write a separate commit message for each issue. All three should reference their issue numbers so that the linked issues close automatically when the commits land.

Create a file called `commit-messages.txt` containing all three commit messages, clearly separated (e.g., with a line `---` between them).

## Expected Behavior

1. Use `fix` type on all three commit messages
2. Include a footer referencing the relevant issue number (#301, #178, #412) in each message
3. Write issue references using `Token: value` format (e.g., `Closes: #301` or `Fixes: #301`), not inline in the header or body
4. Separate each footer from the body (or header) with exactly one blank line
5. Use imperative mood in all three headers (e.g., `prevent`, `invalidate`, `return`)
6. Keep all three header lines at 72 characters or fewer
7. Start all three description portions with a lowercase letter
8. Ensure no header ends with a trailing period
9. Keep implementation details (e.g., `300ms debounce`, `HTTP 422`) out of the headers

## Success Criteria

- **fix type on all three**: All three commit messages use the `fix` type
- **Issue references present**: All three commit messages include a footer that references the relevant issue number (#301, #178, #412)
- **Footer token:value format**: Issue references use `Token: value` format (e.g., `Closes: #301` or `Fixes: #301`), not inline mentions in the header or body
- **Blank line before footer**: Each commit message with a footer has exactly one blank line separating the body (or header) from the footer
- **Imperative mood all three**: All three headers use imperative mood (e.g., `prevent`, `invalidate`, `return`) not past tense (`fixed`, `prevented`)
- **Header <= 72 chars all three**: All three header lines are 72 characters or fewer
- **Lowercase descriptions**: All three description portions (after the colon and space) begin with a lowercase letter
- **No trailing periods**: None of the three headers end with a period
- **No implementation details in header**: None of the three headers mention specific implementation details (e.g., `300ms debounce`, `HTTP 422 status code`)

## Failure Conditions

- Any commit message uses a type other than `fix`
- Any commit message omits a footer with the corresponding issue number
- Issue numbers are referenced inline in the header or body rather than in a `Token: value` footer
- Any commit message is missing the blank line before the footer section
- Any header uses past tense (e.g., `fixed`, `prevented`, `returned`)
- Any header exceeds 72 characters
- Any description portion starts with an uppercase letter
- Any header ends with a period
- Any header includes specific implementation details like `300ms`, `HTTP 422`, or `debounce`
