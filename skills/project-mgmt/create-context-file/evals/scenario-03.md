# Scenario 03: Identify Anti-Patterns in Context File Usage

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

## Expected Behavior

1. Flag the failure to check the existing `.context/` structure first
2. Flag `notes` as a generic slug that risks collisions and violates the naming
   rule
3. Identify that the proposal mixes three typologies in one file: `findings`
   (latency investigation), `guides` (retry how-to), and `plans` (migration
   steps)
4. Flag skipping frontmatter and dropping the date prefix as violations of the
   `YYYY-MM-DD-slug.md` naming and frontmatter conventions
5. End with a corrected plan: three separate date-prefixed files under the right
   typologies, with specific slugs, frontmatter, and a pre-creation check

## Success Criteria

- **Skipping pre-creation check identified**: `antipattern-review.md` flags the
  failure to check the existing `.context/` structure first
- **Generic slug identified**: `antipattern-review.md` flags `notes` as a
  generic slug that violates the naming rule
- **Mixed typologies identified**: `antipattern-review.md` identifies that the
  proposal mixes `findings`, `guides`, and `plans` in one file
- **Missing frontmatter and date identified**: `antipattern-review.md` flags
  skipping frontmatter and dropping the date prefix as convention violations
- **Corrected plan provided**: `antipattern-review.md` ends with three separate
  date-prefixed files under the correct typologies, with specific slugs,
  frontmatter, and a pre-creation check

## Failure Conditions

- Fails to flag the missing pre-creation check
- Accepts the generic `notes` slug without flagging it
- Does not identify that the file mixes three typologies
- Does not flag the missing frontmatter or the dropped date prefix
- Provides no corrected plan, or one that still has the same violations
