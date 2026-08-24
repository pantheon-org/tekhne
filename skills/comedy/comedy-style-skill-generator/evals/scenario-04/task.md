# Scenario 04: Request Names No Analysable Comic Structure

## User Prompt

"Make me a comedy skill, just make it good."

## Expected Behavior

1. Recognize that the request names no comedic tradition, performer, show, or invented style, and gives nothing to analyse — this fails the Prerequisites check.
2. Ask a clarifying question naming what's missing (a genre label, an example bit, or a format description) rather than inventing a tradition to analyse out of nothing.
3. Does not fabricate a plausible-sounding but arbitrary "style" just to have something to work with.
4. Does not produce a skill whose body is adjectives ("be funny", "make it good") standing in for missing analysis.

## Success Criteria

- The response asks for the missing specifics (tradition, performer, example, or format) before attempting any analysis.
- No `SKILL.md` is generated in this turn, since the Prerequisites aren't met.

## Failure Conditions

- A generated skill is produced anyway, built from an invented tradition the user never actually asked for.
- The generated skill's core mechanics are adjectives ("make it funny", "be absurd") rather than concrete rules.
