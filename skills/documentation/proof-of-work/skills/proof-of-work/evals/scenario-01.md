# Scenario 01: Screenshot after UI Deployment

## User Prompt

A frontend change was deployed to staging. The agent must verify the change is visible and document it as proof-of-work.

Using playwright-mcp, navigate to `https://staging.example.com/checkout` and capture a screenshot proving the new "Express Checkout" button is present. Save the artifact correctly and include an evidence summary in your response.

## Expected Behavior

1. Select screenshot as the appropriate artifact type for UI verification
2. Use `page.screenshot()` via playwright-mcp to capture the page
3. Save the artifact to `.context/evidence/` with a date-prefixed kebab-case filename
4. Include an evidence summary table in the response with the artifact path and description
5. Use a descriptive filename that clearly identifies what is shown (not `screenshot1.png` or similar)

## Success Criteria

- **Identifies screenshot as the correct artifact type**: Agent identifies screenshot as the correct artifact type for UI verification
- **Uses playwright-mcp page.screenshot() to save artifact**: Agent uses playwright-mcp `page.screenshot()` with a `.context/evidence/` path
- **Filename follows date-prefixed kebab-case convention**: Filename follows `YYYY-MM-DD-<descriptive-slug>.png` convention
- **Response includes evidence summary table**: Response includes evidence summary table with path and description
- **Agent verifies artifact was written**: Agent verifies artifact was written (e.g., `ls .context/evidence/`)

## Failure Conditions

- Uses a different tool or method (e.g., WebFetch screenshot) instead of playwright-mcp `page.screenshot()`
- Artifact is not saved to `.context/evidence/` or is saved to an incorrect path
- Filename is non-descriptive (e.g., `screenshot1.png`) or does not follow `YYYY-MM-DD-<slug>.png`
- Response does not include an evidence summary table
- Agent does not verify the artifact was written to disk
