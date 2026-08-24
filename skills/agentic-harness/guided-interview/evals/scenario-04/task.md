# Scenario 04: Repeated "Other" and a Contradicting Answer (Edge Case)

## User Prompt

"Interview me on which alerting channel we should default to for on-call."

Turn 1 -- agent offers {Slack, PagerDuty, email, SMS}; user picks "Other" and writes in "a Teams channel we already use."
Turn 2 -- agent offers a new curated set based on "Teams"; user picks "Other" again with another free-text answer.
Turn 3 -- later, the user gives an answer that directly contradicts what they said in Turn 1 (e.g. "actually, forget Teams, let's not use any chat tool for this").

## Expected Behavior

1. After the user picks "Other" twice in a row, treat this as a real signal that the curated option sets are not matching their mental model -- the next question should shift to an open question instead of a third curated set built on a guess.
2. When the contradiction surfaces in Turn 3, surface it plainly to the user and ask which position holds, rather than silently picking one or ignoring the conflict.
3. Do not let the contradiction pass into the recap unexamined -- the recap must reflect whichever answer the user confirms after the contradiction is raised, not an averaged or invented middle ground.
4. Continue the adaptive, one-question-at-a-time protocol throughout despite the edge case.

## Failure Conditions

- A third curated option set offered immediately after two consecutive "Other" picks, without pivoting to an open question.
- The Turn 1/Turn 3 conflict silently resolved by the agent picking one answer without surfacing it.
- The recap presenting both contradictory answers as if they were both still valid, or inventing a compromise the user never stated.
- Reverting to bundled or non-adaptive questioning once the edge case appears.
