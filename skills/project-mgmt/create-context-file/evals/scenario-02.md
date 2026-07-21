# Scenario 02: Produce a Correctly Formatted Context File

## User Prompt

A developer asks: "Can you document my plan for rolling out feature flags to
production? The plan has three steps: 1) Add the feature flag SDK, 2) Gate the
new checkout flow behind a flag, 3) Enable the flag for 10% of users via canary
rollout."

Today's date is 2026-03-16.

Create a properly formatted context file at the path:
`.context/plans/2026-03-16-feature-flag-rollout.md`

The file must:

1. Live under `.context/plans/` (the `plans` typology), not another typology.
2. Use the date-prefixed filename `2026-03-16-feature-flag-rollout.md` with the
   specific slug `feature-flag-rollout` (not a generic slug).
3. Start with valid YAML frontmatter containing `title: "Feature Flag Rollout"`,
   `type: plans`, `date: 2026-03-16`, `status: active`, and `tags`.
4. Follow the frontmatter with a `# Feature Flag Rollout` heading.
5. Contain the three rollout steps as the body content.

## Expected Behavior

1. Place the file under `.context/plans/` (not another typology folder)
2. Name the file `2026-03-16-feature-flag-rollout.md` (date prefix + specific
   slug, not a generic term like `notes` or `plan`)
3. Begin with valid YAML frontmatter including `date: 2026-03-16` (ISO 8601),
   `type: plans`, and a formatted `title`
4. Include a `# Feature Flag Rollout` top-level heading after the frontmatter
5. Include all three rollout steps in the body: SDK addition, feature flag gate,
   and canary rollout

## Success Criteria

- **Correct typology folder**: the file is under `.context/plans/`
- **Date-prefixed filename**: the filename is `2026-03-16-feature-flag-rollout.md`
  (ISO date prefix + specific slug)
- **Valid frontmatter with ISO date and type**: frontmatter contains
  `date: 2026-03-16`, `type: plans`, and a formatted `title`
- **Title heading present**: a `# Feature Flag Rollout` heading follows the
  frontmatter
- **Content includes all three steps**: the body contains SDK addition, feature
  flag gate, and canary rollout

## Failure Conditions

- Places the file under a typology other than `plans`
- Omits the date prefix, or uses a date format other than ISO 8601 (YYYY-MM-DD)
- Uses a three-word-ID prefix or a generic slug (e.g. `notes`, `plan`, `todo`)
  instead of `feature-flag-rollout`
- Omits `type` or `title` from the frontmatter
- Omits one or more of the three rollout steps from the body
