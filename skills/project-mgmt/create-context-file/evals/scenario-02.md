# Scenario 02: Produce a Correctly Formatted Context File

## User Prompt

A developer asks: "Can you document my plan for rolling out feature flags to production? The plan has three steps: 1) Add the feature flag SDK, 2) Gate the new checkout flow behind a flag, 3) Enable the flag for 10% of users via canary rollout."

Today's date is 2026-03-16.

Create a properly formatted context file at the path:
`.context/plans/<three-word-id>-feature-flag-rollout.md`

Use any valid three-word ID of your choice for the filename prefix.

The file must:

1. Start with valid YAML frontmatter containing `date: 2026-03-16` and a formatted `title: Feature Flag Rollout`.
2. Contain the three rollout steps as the body content.
3. Use the specific slug `feature-flag-rollout` (not a generic slug).
4. Be placed in the `.context/plans/` subdirectory (not justifications or scratches).

## Expected Behavior

1. Place the file under `.context/plans/` (not `.context/justifications/` or `.context/scratches/`)
2. Begin the file with valid YAML frontmatter containing `date: 2026-03-16` in ISO 8601 format
3. Include `title: Feature Flag Rollout` (or equivalent formatted title) in the frontmatter
4. Include the slug `feature-flag-rollout` in the filename (not a generic term like `notes` or `plan`)
5. Include all three rollout steps in the file body: SDK addition, feature flag gate, and canary rollout

## Success Criteria

- **Correct subdirectory**: The file is placed under `.context/plans/` (not `justifications/` or `scratches/`)
- **Valid frontmatter with ISO 8601 date**: The file begins with YAML frontmatter containing `date: 2026-03-16` in ISO 8601 format
- **Formatted title in frontmatter**: The frontmatter contains `title: Feature Flag Rollout` (or equivalent formatted title)
- **Specific slug in filename**: The filename contains the slug `feature-flag-rollout` (not a generic term like `notes` or `plan`)
- **Content includes all three rollout steps**: The file body contains all three steps: SDK addition, feature flag gate, and canary rollout

## Failure Conditions

- Places the file in `.context/justifications/` or `.context/scratches/` instead of `.context/plans/`
- Omits YAML frontmatter or uses a date format other than ISO 8601 (YYYY-MM-DD)
- Omits the title from frontmatter
- Uses a generic slug (e.g. `notes`, `plan`, `todo`) instead of `feature-flag-rollout`
- Omits one or more of the three rollout steps from the file body
