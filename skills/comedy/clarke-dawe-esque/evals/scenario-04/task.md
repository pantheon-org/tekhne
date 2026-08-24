# Scenario 04: Request Doesn't Fit the Two-Voice Format

## User Prompt

"Write me a stand-up monologue bit about a housing minister making excuses."

## Expected Behavior

1. Recognize that a single-narrator monologue is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use — this skill requires exactly two alternating voices).
2. Do not force the monologue request into a fake two-hander by inventing an unnecessary second character just to use this skill's mechanics.
3. Say plainly that this skill is built for the two-voice deadpan interview format, and either offer a monologue directly (without this skill's interview mechanics) or ask if the user actually wants the interview format instead.

## Success Criteria

- The response identifies the mismatch between "monologue" (one voice) and this skill's two-voice requirement.
- No fabricated second character is introduced solely to satisfy this skill's format.

## Failure Conditions

- A two-hander interview is produced anyway, silently reinterpreting "monologue" as "interview."
- The response invents an unnecessary interviewer character with no real logical function, just to force-fit the format.
