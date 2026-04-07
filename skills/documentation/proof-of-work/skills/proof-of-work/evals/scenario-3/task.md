# Scenario 3: Save test suite output as evidence

## Context

A CI test run just completed locally. The agent must preserve the test output as a proof-of-work artifact before reporting pass/fail status.

## Task

Run `bun run test` and save the full output (stdout + stderr) to `.context/evidence/`. Report the result with an evidence summary block.

## Expected Behavior

1. Agent identifies script output capture as the appropriate artifact type.
2. Agent uses `2>&1 | tee` to capture both stdout and stderr.
3. Artifact is saved to `.context/evidence/` with a date-prefixed filename ending in `.txt`.
4. Response leads with test pass/fail status and includes an evidence summary table.
5. Agent verifies the artifact was written to disk.
