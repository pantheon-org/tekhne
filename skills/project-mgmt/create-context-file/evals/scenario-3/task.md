# Scenario 3: Identify Anti-Patterns in Context File Usage

## Context

A developer proposes the following context file creation plan:

> "I'll create `.context/plans/notes.md` without checking what's already in .context/.
> The file will document why we chose GraphQL over REST, plus my scratch notes on
> schema design alternatives I'm exploring, plus the step-by-step migration plan.
> I'll skip the frontmatter since it's just temporary notes."

## Your Task

Review this proposal against the create-context-file skill's anti-patterns and produce
a written review saved to `antipattern-review.md`.

## Requirements

Your output must:

1. Identify every anti-pattern from the skill that this proposal violates.
2. For each anti-pattern, quote the specific part of the proposal that violates it.
3. For each violation, state the corrected approach.
4. Provide a corrected file creation plan at the end that fixes all identified issues
   (correct types, slugs, frontmatter, and pre-creation check).

## Output Spec

File: `antipattern-review.md`
