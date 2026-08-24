# Scenario 05: No Genuine Match — Skip the Skill

## User Prompt

"Can you help me pick a name for our new internal CLI tool?"

## Expected Behavior

1. Recognize that this request does not match any of the trigger scenarios in `SKILL.md` § When to Use (not a root cause writeup, ticket refinement, escalation, stakeholder comms, staff management, or contractor relations).
2. Skip this skill entirely rather than forcing a Madden aphorism to fit a naming question.
3. Do not load any `references/*.md` category file.
4. If the agent has other ways to help with naming, it does so without inserting an unrelated NASA PM citation just to "use" this skill.

## Success Criteria

- No Madden aphorism, LLIS citation, or NASA PM lessons framing appears anywhere in the response.
- No `references/*.md` category file is loaded.
- The response addresses the actual naming question (or declines/redirects) without forcing an unrelated skill's framing onto it.

## Failure Conditions

- A Madden aphorism is stretched to cover the naming decision (e.g. citing a "Planning and Decision-Making" lesson as if it applies to naming a CLI tool).
- Any category reference file is loaded despite no genuine match existing.
- The response manufactures a connection between NASA PM lessons and the naming task just to justify invoking this skill.
