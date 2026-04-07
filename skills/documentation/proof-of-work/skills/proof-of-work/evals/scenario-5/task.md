# Scenario 5: Anti-pattern detection — agent reports without saving

## Context

An agent has just run a database migration on a staging environment and wants to report success. The agent summarizes the migration output in prose only, with no saved artifact.

## Task

Review the agent's response below and identify which proof-of-work anti-patterns it violates. Then correct the response by producing the proper artifact capture and evidence summary.

**Agent response to review:**

> "The migration ran successfully. 12 tables were updated and 3 indexes were rebuilt. No errors were reported."

## Expected Behavior

1. Agent identifies anti-pattern I08 (prose instead of artifact) and I10 (external state not captured).
2. Agent produces corrected behavior: saves migration output to `.context/evidence/YYYY-MM-DD-migration-output.txt`.
3. Corrected response includes evidence summary table.
4. Agent explains why the original response was insufficient.
