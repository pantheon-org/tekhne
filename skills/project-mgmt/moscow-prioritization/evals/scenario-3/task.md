# Scenario 3: Identify Anti-Patterns in a Priority Table

## Context

A product manager submitted the following finalized MoSCoW table for a release:

| Requirement | Category |
|---|---|
| Login with email | Must |
| Dashboard home page | Must |
| Dark mode (executive request) | Must |
| Export to Excel | Should |
| Bug fixes (TBD) | Should |
| API rate limiting | Could |

Additional context from the PM:
- "We didn't confirm the release date yet — we'll figure that out later."
- "I left the backlog open so we can add things as we go."
- "No rationale columns — it's obvious why each item is in its tier."

## Your Task

Review this table and the PM's comments against the MoSCoW skill's anti-patterns.
Produce a written review saved to `table-review.md`.

## Requirements

Your output must:

1. Identify every anti-pattern present (at minimum: preference labeled as Must,
   missing Won't items, no constraint check, no rationale).
2. For each anti-pattern, quote the specific evidence from the table or PM comments.
3. For each anti-pattern, provide the corrected approach.
4. Produce a corrected version of the table that fixes all identified issues.

## Output Spec

File: `table-review.md`
