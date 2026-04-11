# Scenario 03: Identify Anti-Patterns in Context File Usage

## User Prompt

A developer proposes the following context file creation plan:

> "I'll create `.context/plans/notes.md` without checking what's already in `.context/`. The file will document why we chose GraphQL over REST, plus my scratch notes on schema design alternatives I'm exploring, plus the step-by-step migration plan. I'll skip the frontmatter since it's just temporary notes."

Review this proposal against the create-context-file skill's anti-patterns and produce a written review saved to `antipattern-review.md`.

Your output must:

1. Identify every anti-pattern from the skill that this proposal violates.
2. For each anti-pattern, quote the specific part of the proposal that violates it.
3. For each violation, state the corrected approach.
4. Provide a corrected file creation plan at the end that fixes all identified issues (correct types, slugs, frontmatter, and pre-creation check).

## Expected Behavior

1. Flag the failure to check existing `.context/` structure before creating the file
2. Flag `notes` as a generic slug that risks collisions and violates the naming rule
3. Identify that the proposal mixes justification (GraphQL rationale), scratch (schema exploration), and plan (migration steps) in one file
4. Flag the proposal to skip frontmatter as a violation of the ISO 8601 date requirement
5. End with a corrected creation plan: three separate files with specific slugs, correct types, frontmatter, and a pre-creation `ls` check

## Success Criteria

- **Skipping pre-creation check identified**: `antipattern-review.md` flags the failure to check existing `.context/` structure before creating the file
- **Generic slug identified**: `antipattern-review.md` flags `notes` as a generic slug that risks collisions and violates the naming rule
- **Mixed content types identified**: `antipattern-review.md` identifies that the proposal mixes justification (GraphQL rationale), scratch (schema exploration), and plan (migration steps) in one file
- **Missing frontmatter identified**: `antipattern-review.md` flags the proposal to skip frontmatter as a violation of the ISO 8601 date requirement
- **Corrected plan provided**: `antipattern-review.md` ends with a corrected creation plan: three separate files with specific slugs, correct types, frontmatter, and a pre-creation `ls` check

## Failure Conditions

- Fails to flag the missing pre-creation check
- Accepts the generic `notes` slug without flagging it as a violation
- Does not identify that the proposed file mixes three different content types
- Does not flag the missing frontmatter as a violation
- Provides no corrected plan or provides a plan that still has the same violations
