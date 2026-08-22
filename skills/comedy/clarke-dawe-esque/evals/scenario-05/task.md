# Scenario 05: Draft Where the Interviewer Breaks Deadpan

## User Prompt

"Here's my draft: Interviewer: Wait, that's insane! You can't seriously believe that! Spokesperson: I do believe it. Can you fix the pacing?"

## Expected Behavior

1. Recognize that the interviewer's line violates the "never let the interviewer express outrage or disbelief" anti-pattern — "Wait, that's insane!" is visible emotion, not a flat follow-up question.
2. Rewrite the interviewer's line as a flat, neutral follow-up question that exposes the same absurdity through content rather than tone (per the Anti-Patterns example pattern in `SKILL.md`).
3. Keep the spokesperson's confident, literal-minded answer but adjust the surrounding exchange if needed so the escalation still reads as one-step-at-a-time.
4. Briefly explain that the fix was about restoring deadpan neutrality in the interviewer's line, not just "pacing."

## Success Criteria

- The revised interviewer line contains no exclamation, no explicit judgment word ("insane"), and no visible emotion.
- The revised line is still a genuine follow-up question that advances the escalation.
- The response names the specific issue (interviewer breaking deadpan) rather than only addressing generic "pacing."

## Failure Conditions

- The revision keeps the outraged tone or just softens the wording while preserving visible emotion.
- The fix addresses pacing/length only and misses that the core problem is broken deadpan.
