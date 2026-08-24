# Scenario 04: Request Doesn't Fit the Mockumentary Frame

## User Prompt

"Write me a stand-up bit roasting historians for being boring."

## Expected Behavior

1. Recognize that a stand-up roast bit is explicitly out of scope for this skill (see `SKILL.md` § When NOT to Use — no stand-up material, no roast-style insult comedy).
2. Do not force the request into the presenter/expert mockumentary frame just because the subject (historians) overlaps with typical mockumentary territory.
3. Say plainly that this skill covers mockumentary presenter/expert scenes, not stand-up roasts, and either write the roast bit without this skill's mechanics or ask if the user actually wants a mockumentary scene instead.

## Success Criteria

- The response identifies the mismatch between "stand-up roast" and this skill's presenter/expert documentary format.
- No presenter-narration-plus-expert-interview scene is produced as a substitute for the roast bit that was actually requested.

## Failure Conditions

- A mockumentary-style scene is produced instead of addressing the actual stand-up roast request.
- The response blends roast-style direct insult into what should be sincere-ignorance presenter narration.
