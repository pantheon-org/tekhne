# Scenario 05: Decompose a Multi-Need Request by Typology

## User Prompt

A developer says: "I need to figure out the best way to handle versioning for
our public API. I'm going to investigate URL versioning vs header versioning vs
no versioning and write up what I find. Then I'll need to track the steps to
actually implement whatever I choose. And I want a short how-to the team can
follow to add a new API version later."

This single statement describes three separate needs at once.

Using the create-context-file skill's typology guide (pick by *what the artifact
is*), produce a file saved to `typology-breakdown.md` that:

1. Identifies all three distinct needs embedded in the developer's statement.
2. Maps each need to the correct typology.
3. Proposes a specific date-prefixed filename for each (assume today is
   2026-03-16).
4. States the lifecycle expectation for each file (kept, retired when done, etc).

## Expected Behavior

1. Identify three separate needs: investigation write-up, implementation steps,
   and a reusable how-to
2. Map the investigation write-up to a `findings` file
3. Map the implementation tracking to a `plans` file
4. Map the reusable how-to to a `guides` file
5. Provide a specific, date-prefixed filename for each (e.g.
   `.context/findings/2026-03-16-api-versioning-options.md`,
   `.context/plans/2026-03-16-api-versioning-implementation.md`,
   `.context/guides/2026-03-16-adding-a-new-api-version.md`) — not generic slugs
6. Include an explicit lifecycle statement for each file

## Success Criteria

- **Three distinct needs identified**: `typology-breakdown.md` identifies
  investigation, implementation steps, and a reusable how-to
- **Investigation mapped to findings**: the investigation write-up maps to
  `findings`
- **Implementation mapped to plans**: the implementation tracking maps to `plans`
- **How-to mapped to guides**: the reusable how-to maps to `guides`
- **Specific date-prefixed filenames provided**: each file has a
  `YYYY-MM-DD-slug.md` name with a specific slug (not `notes`/`todo`) under the
  correct typology folder
- **Lifecycle expectations provided**: each file has an explicit lifecycle
  statement

## Failure Conditions

- Identifies fewer than three distinct needs, collapsing them into one file
- Maps the investigation to a typology other than `findings`
- Maps the implementation steps to a typology other than `plans`
- Maps the how-to to a typology other than `guides`
- Provides generic slugs, or filenames without the ISO date prefix
- Omits lifecycle expectations for one or more files
