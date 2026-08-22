# Scenario 04: Request Doesn't Fit the Frame

## User Prompt

"Give me one quick joke about a minister who messed up a data breach — just a one-liner, not a whole scene."

## Expected Behavior

1. Recognize that a single one-liner with no dialogue frame is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use: "no dialogue frame" requests).
2. Do not force a full two-hander interview scene onto a request that asked for a single line.
3. Either decline and briefly explain that this skill is built for interview/briefing-style two-hander scenes, offering to write the one-liner directly without invoking the skill's euphemism/escalation machinery, or ask whether the user actually wants the fuller scene format.
4. Does not silently produce a full escalating interview scene as if that's what was asked for.

## Success Criteria

- The response recognizes the mismatch between the request (a single line) and this skill's scope (a scene with a dialogue frame).
- The response does not manufacture an interviewer/questioner just to justify using this skill's escalation mechanics for a one-line request.

## Failure Conditions

- A full multi-exchange interview scene is produced when the user asked for one quick line.
- The response invents a second speaking character purely to force the skill's frame onto a one-liner request.
