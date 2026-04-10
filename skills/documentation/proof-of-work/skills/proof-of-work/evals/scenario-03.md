# Scenario 03: Save Test Suite Output as Evidence

## User Prompt

A CI test run just completed locally. The agent must preserve the test output as a proof-of-work artifact before reporting pass/fail status.

Run `bun run test` and save the full output (stdout + stderr) to `.context/evidence/`. Report the result with an evidence summary block.

## Expected Behavior

1. Identify script output capture as the appropriate artifact type
2. Use `2>&1 | tee` to capture both stdout and stderr
3. Save the artifact to `.context/evidence/` with a date-prefixed filename ending in `.txt`
4. Lead the response with test pass/fail status and include an evidence summary table
5. Verify the artifact was written to disk (e.g., via `ls .context/evidence/` or a file check)

## Success Criteria

- **Captures both stdout and stderr using 2>&1 | tee**: Agent captures both stdout and stderr using `2>&1 | tee`
- **Artifact saved to .context/evidence/ with date-prefixed descriptive filename**: Artifact saved to `.context/evidence/` with date-prefixed descriptive filename
- **Response includes evidence summary table with artifact path**: Response includes evidence summary table with artifact path
- **Agent verifies artifact was written**: Agent verifies artifact was written (`ls .context/evidence/` or file check)
- **Test result reported alongside artifact reference**: Test result (pass/fail) is reported alongside the artifact reference, not instead of it

## Failure Conditions

- Runs `bun run test` without `2>&1 | tee`, losing stderr output
- Artifact is not saved to `.context/evidence/` or filename does not follow the date-prefixed convention
- Response does not include an evidence summary table
- Agent does not verify the artifact file was written to disk
- Test pass/fail result is reported without any reference to the saved artifact
