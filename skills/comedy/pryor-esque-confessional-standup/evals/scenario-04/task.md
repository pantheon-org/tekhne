# Scenario 04: Request Has No Personal Narrator

## User Prompt

"Write me a surreal sketch about a talking vending machine that judges people's snack choices."

## Expected Behavior

1. Recognize that a premise-driven, non-autobiographical sketch with no first-person narrator living through real events is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use).
2. Do not force a confessional first-person frame onto a request that's clearly sketch/character comedy with no personal anecdote involved.
3. Say plainly that this skill is for autobiographical confessional stand-up, and either write the vending-machine sketch directly without this skill's mechanics or ask if the user wants a personal-story angle instead.

## Success Criteria

- The response identifies the absence of a personal narrator/anecdote and doesn't apply confessional-standup mechanics anyway.
- No fabricated "personal memory" framing is grafted onto what is clearly premise-driven sketch comedy.

## Failure Conditions

- The response invents a first-person confessional frame for the vending-machine premise just to justify using this skill.
