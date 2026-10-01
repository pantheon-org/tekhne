# Scenario 2: Produce a Correctly Formatted Context File

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
   `type: plan` (singular - the folder is `plans`, but the frontmatter field is
   singular), `date: 2026-03-16`, `status: active`, and `tags`.
4. Follow the frontmatter with a `# Feature Flag Rollout` heading.
5. Contain the three rollout steps as the body content.
