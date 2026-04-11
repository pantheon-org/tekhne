# Scenario 05: Anti-pattern Detection — Agent Reports Without Saving

## User Prompt

An agent has just run a database migration on a staging environment and wants to report success. The agent summarizes the migration output in prose only, with no saved artifact.

Review the agent's response below and identify which proof-of-work anti-patterns it violates. Then correct the response by producing the proper artifact capture and evidence summary.

**Agent response to review:**

> "The migration ran successfully. 12 tables were updated and 3 indexes were rebuilt. No errors were reported."

## Expected Behavior

1. Identify anti-pattern I08 (prose instead of artifact) and I10 (external state not captured) as the violations in the original response
2. Provide corrected behavior: a capture command that saves migration output to `.context/evidence/YYYY-MM-DD-migration-output.txt`
3. Include an evidence summary table in the corrected response
4. Explain why the original response was insufficient as proof-of-work

## Success Criteria

- **Correctly identifies prose-without-artifact anti-patterns**: Agent correctly identifies that prose description without artifact violates anti-patterns I08 and I10
- **Provides corrected capture command to .context/evidence/**: Agent provides corrected capture command that saves migration output to `.context/evidence/`
- **Corrected filename follows date-prefixed descriptive convention**: Corrected filename follows `YYYY-MM-DD-<descriptive-slug>.txt` convention
- **Corrected response includes evidence summary table**: Corrected response includes evidence summary table
- **Agent explains why original prose response is insufficient**: Agent explains why the original prose response is insufficient as proof-of-work

## Failure Conditions

- Fails to identify anti-patterns I08 or I10 as the violations
- Does not provide a corrected capture command that saves output to `.context/evidence/`
- Corrected filename does not follow the `YYYY-MM-DD-<slug>.txt` convention
- Corrected response does not include an evidence summary table
- Does not explain why the original prose-only response is insufficient as proof-of-work
