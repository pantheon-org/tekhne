# Scenario 03: Identify Anti-Patterns in a Priority Table

## User Prompt

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

Review this table and the PM's comments against the MoSCoW skill's anti-patterns. Produce a written review saved to `table-review.md`.

Your output must:

1. Identify every anti-pattern present (at minimum: preference labeled as Must, missing Won't items, no constraint check, no rationale).
2. For each anti-pattern, quote the specific evidence from the table or PM comments.
3. For each anti-pattern, provide the corrected approach.
4. Produce a corrected version of the table that fixes all identified issues.

## Expected Behavior

1. Flag "Dark mode (executive request)" as a stakeholder preference incorrectly categorized as Must
2. Identify the absence of any Won't items and note it enables silent scope creep
3. Flag the PM's admission that constraints (release date, capacity) were not confirmed before categorizing
4. Flag the lack of rationale columns as a violation (unjustified categories get re-litigated)
5. Produce a corrected table with: dark mode moved from Must, at least one explicit Won't with a revisit date, and a rationale column populated

## Success Criteria

- **Preference-as-Must anti-pattern identified**: `table-review.md` flags "Dark mode (executive request)" as a stakeholder preference incorrectly categorized as Must
- **Missing Won't anti-pattern identified**: `table-review.md` identifies the absence of any Won't items and notes it enables silent scope creep
- **Missing constraint check anti-pattern identified**: `table-review.md` flags the PM's admission that constraints (release date, capacity) were not confirmed before categorizing
- **Missing rationale anti-pattern identified**: `table-review.md` flags the lack of rationale columns as a violation (unjustified categories are re-litigated)
- **Corrected table produced**: `table-review.md` includes a corrected table with: dark mode moved from Must, at least one explicit Won't with revisit date, and a rationale column populated

## Failure Conditions

- Does not flag dark mode as a preference-based Must violation
- Does not identify the missing Won't items as an anti-pattern
- Does not flag the missing constraint check
- Does not flag the missing rationale columns
- Does not produce a corrected table addressing the identified violations
