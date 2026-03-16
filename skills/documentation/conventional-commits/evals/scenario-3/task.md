# Write Commit Messages for a Bug Fix Sprint

## Problem Description

Your team just closed a two-day bug sprint. Three issues were resolved and are ready to be committed to the main branch. The project uses automated changelog generation and GitHub's issue-closing keywords.

Here are the three resolved issues and the changes made:

**Issue #301 — Double submission on checkout form**
A race condition allowed users to double-click the "Place Order" button during network latency, resulting in duplicate charges. The fix debounces the submit handler with a 300ms window and disables the button immediately on first click.

**Issue #178 — Stale product prices shown after update**
Product prices cached in the session store were not invalidated when an admin updated prices in the dashboard. The fix clears the relevant cache keys whenever the product catalogue endpoint detects a write operation.

**Issue #412 — Wrong HTTP status on validation failure**
The registration endpoint was returning HTTP 500 for invalid email addresses rather than HTTP 422. The endpoint now validates input before attempting database insertion and returns the correct status with a descriptive error body.

Write a separate commit message for each issue. All three should reference their issue numbers so that the linked issues close automatically when the commits land.

## Output Specification

Create a file called `commit-messages.txt` containing all three commit messages, clearly separated (e.g. with a line "---" between them).
