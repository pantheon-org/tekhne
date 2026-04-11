# Scenario 01: Produce Deterministic Reproduction Steps

## User Prompt

A developer reports: "The user table on /admin/users sometimes shows wrong row counts after sorting. I saw it happen once this afternoon."

Using the UI Debug Workflow skill, produce a written reproduction recipe that a second developer or automated test can follow to reliably trigger the reported behavior.

Your output must be a Markdown file saved to `debug-reproduction.md` containing:

1. A numbered list of precise reproduction steps (URL to open, interactions to perform, observable outcome to watch for).
2. The environment context required: browser, viewport size, seed data state, build mode.
3. An explicit statement of what "reproduced" means (the specific observable symptom).
4. At least one console/network check step that must be performed before any code change.

The file must be self-contained so a reader with no prior context can follow the steps and reach the same outcome.

## Expected Behavior

1. Include a numbered list of at least 3 concrete steps (open URL, perform interaction, observe outcome) in `debug-reproduction.md`
2. Explicitly state the browser, viewport size, seed data state, and build mode required for the scenario
3. Include a clear statement of what symptom confirms the bug is reproduced — not vague language like "looks broken"
4. Include at least one step instructing the reader to inspect browser console errors or network failures before touching code
5. Contain only reproduction and observation steps — not propose or apply any code changes

## Success Criteria

- **Numbered reproduction steps**: `debug-reproduction.md` contains a numbered list of at least 3 concrete steps (open URL, perform interaction, observe outcome)
- **Environment context specified**: The file explicitly states browser, viewport size, seed data state, and build mode required for the scenario
- **Definition of reproduced**: The file contains a clear statement of what symptom confirms the bug is reproduced (not vague language like "looks broken")
- **Console/network check step included**: At least one step instructs the reader to inspect browser console errors or network failures before touching code
- **No code modifications proposed**: The file contains only reproduction and observation steps; it does not propose or apply any code changes

## Failure Conditions

- `debug-reproduction.md` contains fewer than 3 numbered reproduction steps
- The file does not specify browser, viewport, seed data, or build mode requirements
- The definition of "reproduced" is vague (e.g., "table looks wrong") without specifying the exact observable symptom
- No step instructs the reader to check the browser console or network panel before making changes
- The file proposes or applies code changes instead of focusing solely on reproduction
