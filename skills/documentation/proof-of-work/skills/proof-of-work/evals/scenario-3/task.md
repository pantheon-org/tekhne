# Scenario 3: Save Test Suite Output as Evidence

## User Prompt

A CI test run just completed locally. The agent must preserve the test output as a proof-of-work artifact before reporting pass/fail status.

Run `bun run test` and save the full output (stdout + stderr) to `.context/evidence/`. Report the result with an evidence summary block.
