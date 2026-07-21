# Scenario 01: Select the Correct Typology

## User Prompt

You receive the following requests from different developers on the same team.
For each, classify which context-file **typology** to use and explain why.

**Request A:** "I dug into why our login latency spiked last week and want to
write up what I found so the team can reference it."

**Request B:** "I'm about to migrate the database from Postgres 14 to 15. I want
to lay out the steps I'll follow before I start writing any scripts."

**Request C:** "I keep having to explain how our retry/backoff wrapper works.
I want a reusable how-to the team can point new joiners at."

**Request D:** "There are three edge cases we agreed to defer on the checkout
refactor. I want to capture them so they aren't forgotten."

Produce a Markdown file saved to `typology-selection.md` that, for each request
(A, B, C, D):

1. States the typology you would use (`findings`, `plans`, `guides`, or
   `follow-ups`).
2. Explains, from the skill, why that typology fits (pick by *what the artifact
   is*).
3. Provides the slug you would use (specific, task-tied, not generic).

## Expected Behavior

1. Classify Request A (investigation write-up) as `findings`
2. Classify Request B (steps before execution) as `plans`
3. Classify Request C (reusable how-to) as `guides`
4. Classify Request D (deferred work capture) as `follow-ups`
5. Justify each by what the artifact is, per the skill's typology guide
6. Provide a specific, task-tied slug for each (e.g. `login-latency-spike`,
   `postgres-14-to-15-migration`, `retry-backoff-wrapper`,
   `checkout-refactor-deferred-edge-cases`) — not generic terms like `notes`

## Success Criteria

- **Request A classified as findings**: `typology-selection.md` classifies
  Request A as `findings`
- **Request B classified as plans**: `typology-selection.md` classifies Request
  B as `plans`
- **Request C classified as guides**: `typology-selection.md` classifies Request
  C as `guides`
- **Request D classified as follow-ups**: `typology-selection.md` classifies
  Request D as `follow-ups`
- **Reasoning references the typology guide**: each classification explains the
  choice by what the artifact is, per the skill
- **Specific slugs provided**: each entry includes a task-specific slug, not a
  generic term like `notes` or `todo`

## Failure Conditions

- Misclassifies any of A/B/C/D into the wrong typology
- Justifies a choice by lifecycle alone instead of what the artifact is
- Provides generic slugs (e.g. `notes`, `todo`, `research`)
- Invents a brand-new typology when an existing curated one fits
