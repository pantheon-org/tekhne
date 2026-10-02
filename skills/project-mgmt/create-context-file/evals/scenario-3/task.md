# Scenario 3: Identify Anti-Patterns in Context File Usage

## User Prompt

A developer proposes the following context file creation plan:

> "I'll create `.context/plans/notes.md` without checking what's already in
> `.context/`. The file will document what I found investigating the login
> latency spike, plus a reusable how-to for our retry wrapper, plus the
> step-by-step migration plan. I'll skip the frontmatter and drop the date from
> the filename since it's just temporary."

Review this proposal against the create-context-file skill's anti-patterns and
produce a written review saved to `antipattern-review.md`.

Your output must:

1. Identify every anti-pattern from the skill that this proposal violates.
2. For each anti-pattern, quote the specific part of the proposal that violates
   it.
3. For each violation, state the corrected approach.
4. Provide a corrected creation plan at the end that fixes all issues (correct
   typologies, specific slugs, date-prefixed filenames, frontmatter, and a
   pre-creation check).
