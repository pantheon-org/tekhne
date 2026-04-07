# Scenario 1: Screenshot after UI deployment

## Context

A frontend change was deployed to staging. The agent must verify the change is visible and document it as proof-of-work.

## Task

Using playwright-mcp, navigate to `https://staging.example.com/checkout` and capture a screenshot proving the new "Express Checkout" button is present. Save the artifact correctly and include an evidence summary in your response.

## Expected Behavior

1. Agent selects screenshot as the appropriate artifact type.
2. Agent uses `page.screenshot()` to capture the page.
3. Artifact is saved to `.context/evidence/` with a date-prefixed kebab-case filename.
4. Response includes an evidence summary table with artifact path and description.
5. Filename clearly describes what is shown (not `screenshot1.png` or similar).
