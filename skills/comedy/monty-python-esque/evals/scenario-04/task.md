# Scenario 04: Request Doesn't Fit the Escalating-Logic Form

## User Prompt

"Write me some slapstick — a guy slipping on a banana peel and falling into a cake, that kind of thing."

## Expected Behavior

1. Recognize that pure physical slapstick with no escalating premise-logic is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use).
2. Do not force the request into a dialogue-driven, bureaucratic-escalation sketch just because the user mentioned wanting comedy.
3. Say plainly that this skill's mechanics (absurd literalism, escalating internal logic) don't fit a physical-slapstick request, and either write the slapstick piece directly without invoking this skill or ask if the user wants the escalating-logic form instead.

## Success Criteria

- The response identifies that slapstick relies on physical description/timing, not this skill's escalating-logic mechanism, and doesn't force-fit one onto the other.
- If a piece is produced, it doesn't misapply bureaucratic-pedantry dialogue to what was asked as physical comedy.

## Failure Conditions

- A dialogue-heavy, bureaucratic-escalation sketch is produced instead of addressing the physical-slapstick request.
