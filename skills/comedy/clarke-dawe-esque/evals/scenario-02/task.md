# Scenario 02: Draft with angry interviewer and a resolved ending

## User Prompt

"Here's a draft mock-interview sketch I wrote about a bank closing its branches and calling it a 'community banking transformation.' Can you punch it up? Right now the interviewer says 'This is outrageous, you can't seriously call this a transformation!' and it ends with the spokesperson admitting 'You're right, we'll reverse the closures.' Make it funnier."

## Expected Behavior

1. The agent identifies that the interviewer's line breaks the deadpan mechanism (visible outrage) and flags or rewrites it to stay flat and reasonable-sounding instead.
2. The agent identifies that the ending resolves the contradiction (the spokesperson agreeing to reverse the closures) and flags or rewrites it so the absurdity stays unresolved and is instead delivered as a flat, understated closing line.
3. The rewritten sketch keeps the exchange escalating one step at a time rather than jumping straight from calm to a full reversal.
4. The rewritten sketch exposes the contradiction (branch closures dressed up as "transformation") through the interviewer's follow-up questions rather than through either character stating outright that it doesn't make sense.
5. The agent explains briefly why these two changes (flat interviewer, unresolved ending) make the sketch work better, tying it to the mechanics of the form.

## Success Criteria

- The rewritten interviewer line contains no exclamation, accusation, or overt emotional outburst.
- The rewritten ending does not have the spokesperson concede, apologize, or announce a reversal — the absurd position is maintained calmly to the end.
- The agent's explanation references the deadpan/flat-voices mechanism or the "unresolved contradiction as punchline" principle in its own words.

## Failure Conditions

- The agent leaves the outraged interviewer line unchanged in its rewrite.
- The agent leaves the resolved/reversed ending unchanged in its rewrite.
- The agent "punches up" the sketch by adding more emotion or a bigger resolution rather than flattening and leaving things unresolved.
