# Scenario 2: Produce a Correctly Formatted Context File

## Context

A developer asks: "Can you document my plan for rolling out feature flags to production?
The plan has three steps: 1) Add the feature flag SDK, 2) Gate the new checkout flow
behind a flag, 3) Enable the flag for 10% of users via canary rollout."

Today's date is 2026-03-16.

## Your Task

Create a properly formatted context file at the path:
`.context/plans/<three-word-id>-feature-flag-rollout.md`

Use any valid three-word ID of your choice for the filename prefix.

## Requirements

The file must:

1. Start with valid YAML frontmatter containing `date: 2026-03-16` and a formatted
   `title: Feature Flag Rollout`.
2. Contain the three rollout steps as the body content.
3. Use the specific slug `feature-flag-rollout` (not a generic slug).
4. Be placed in the `.context/plans/` subdirectory (not justifications or scratches).

## Output Spec

File: `.context/plans/<three-word-id>-feature-flag-rollout.md`
