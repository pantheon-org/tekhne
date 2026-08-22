# Scenario 05: Revising a Draft That Breaks Character

## User Prompt

"Here's a draft: OFFICIAL: Look, between you and me, I know how bad this sounds, but the department needed the numbers to look better. Can you punch this up?"

## Expected Behavior

1. Recognize that the draft line violates the "never let the character become self-aware" anti-pattern — the official is explicitly admitting the deception rather than sincerely justifying it.
2. Rewrite the line (and any surrounding dialogue affected) so the official stays convinced their own reasoning is sound, replacing the confession with euphemistic self-justification.
3. Explain briefly why the original line was flattened into a confession rather than kept as satire, tying the fix back to the sincerity mechanism.
4. Preserve the underlying comic premise (numbers being made to look better) while changing only how the character talks about it.

## Success Criteria

- The revised line no longer contains the character acknowledging wrongdoing ("I know how bad this sounds") or breaking the fourth wall ("between you and me").
- The revised line uses euphemistic, confident language for the same underlying fact.
- The response names the specific anti-pattern being fixed (sincerity/self-awareness).

## Failure Conditions

- The "punched up" version keeps or intensifies the confessional, self-aware framing instead of removing it.
- The fix only makes the line more crude or insulting rather than more euphemistic and sincere.
- The response doesn't identify why the original draft didn't work.
