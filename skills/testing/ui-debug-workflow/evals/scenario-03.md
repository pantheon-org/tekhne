# Scenario 03: Identify Anti-Patterns in a Debug Process

## User Prompt

A junior engineer submitted the following pull request description:

> "Fixed the broken header layout. I noticed it looked off on my laptop so I tweaked the CSS until it looked right. Tested on my machine — looks good now! Merging."

Review the submitted PR description against the UI Debug Workflow skill's anti-patterns and produce a written review saved to `pr-review.md`.

Your output must:

1. Identify every anti-pattern from the skill that the process violates (at minimum: missing reproduction steps, skipped console/network review, missing evidence artifacts, different conditions between runs if applicable).
2. For each anti-pattern identified, quote or paraphrase the specific part of the PR description that violates it.
3. For each anti-pattern identified, provide a corrected action the engineer should have taken instead.
4. End with a clear APPROVE or REQUEST CHANGES verdict.

## Expected Behavior

1. Explicitly call out the lack of deterministic reproduction steps (no numbered steps, URL, or interaction sequence) in `pr-review.md`
2. Explicitly call out the absence of screenshots, DOM snapshots, or comparison artifacts
3. Note that the engineer edited CSS without first inspecting console errors or network failures
4. Include a specific alternative action the engineer should have taken for each anti-pattern identified
5. End with an explicit APPROVE or REQUEST CHANGES verdict (must be REQUEST CHANGES for this scenario)

## Success Criteria

- **Missing reproduction steps anti-pattern identified**: `pr-review.md` explicitly calls out the lack of deterministic reproduction steps (no numbered steps, URL, or interaction sequence)
- **Missing evidence artifacts anti-pattern identified**: `pr-review.md` explicitly calls out the absence of screenshots, DOM snapshots, or comparison artifacts
- **Skipped console/network review anti-pattern identified**: `pr-review.md` notes that the engineer edited CSS without first inspecting console errors or network failures
- **Corrected actions provided**: For each anti-pattern identified, `pr-review.md` includes a specific alternative action the engineer should have taken
- **Clear verdict stated**: `pr-review.md` ends with an explicit APPROVE or REQUEST CHANGES verdict (must be REQUEST CHANGES for this scenario)

## Failure Conditions

- `pr-review.md` does not identify the missing reproduction steps anti-pattern
- `pr-review.md` does not call out the absence of evidence artifacts (screenshots, snapshots)
- `pr-review.md` does not note that CSS was edited without first checking console/network errors
- No corrected alternative action is provided for one or more identified anti-patterns
- `pr-review.md` does not end with an explicit APPROVE or REQUEST CHANGES verdict, or concludes with APPROVE instead of REQUEST CHANGES
