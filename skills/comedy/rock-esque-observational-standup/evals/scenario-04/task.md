# Scenario 04: Request Doesn't Fit the Solo-Argument Form

## User Prompt

"Write me a sketch with three characters arguing at a family dinner about who has to host Thanksgiving next year."

## Expected Behavior

1. Recognize that a multi-character scripted sketch is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use — this skill produces a single-voice escalating argument, not a multi-character scene).
2. Do not force the family-dinner premise into a single-narrator "there's a difference between X and Y" bit just because the subject could plausibly fit either form.
3. Say plainly that this skill is for solo observational stand-up, and either write the multi-character sketch directly without this skill's mechanics or ask if the user wants a solo bit on the same subject instead.

## Success Criteria

- The response identifies the mismatch between "sketch with three characters" and this skill's single-narrator argument form.
- No attempt is made to collapse three arguing characters into one stand-up voice just to use this skill.

## Failure Conditions

- A single-narrator observational bit is produced in place of the actual multi-character sketch that was requested, without flagging the substitution.
