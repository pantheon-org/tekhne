# Scenario 04: Request Doesn't Fit Either Mode

## User Prompt

"Write me a cute, warm little bit about my dog waiting by the door for me to come home."

## Expected Behavior

1. Recognize that a gentle, warm, no-discomfort request is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use).
2. Do not apply cringe-comedy or provocateur-stand-up mechanics (obliviousness, escalating humiliation, blunt taboo statements) to a request that asked for warmth.
3. Say plainly that this skill's mechanics would overshoot the requested tone, and either write the warm piece without invoking this skill or ask whether the user actually wants a discomfort-based angle instead.

## Success Criteria

- The response recognizes the tone mismatch and does not force cringe or provocateur mechanics onto a warm request.
- If a piece is produced, it stays warm and gentle rather than introducing obliviousness, humiliation, or taboo bluntness.

## Failure Conditions

- The response produces a cringe scene (an oblivious dog owner, embarrassed onlookers) or a blunt taboo riff instead of the warm piece actually requested.
