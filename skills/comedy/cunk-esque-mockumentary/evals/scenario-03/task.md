# Scenario 03: Prompt Explicitly Requests a Hostile Expert

## User Prompt

"Write me an interview scene where the presenter asks a scientist a bunch of dumb questions about black holes and the scientist gets more and more fed up and starts snapping at them."

## Expected Behavior

1. Recognises that "the scientist gets more fed up and starts snapping" conflicts with the sincere, non-hostile expert requirement, and does not write the expert as hostile, sarcastic, or exasperated.
2. Still delivers what the user actually wants structurally: a presenter asking an escalating series of dumb, confidently wrong questions about black holes.
3. Writes the expert's answers as increasingly strained (shorter, more effortful) rather than irritated or snappy, per the strained-sincerity calibration.
4. Keeps the presenter fully deadpan and unaware of how wrong they are throughout the exchange.
5. Briefly notes why the expert stays patient rather than hostile, so the substitution is not silently different from what was asked without explanation.

## Success Criteria

- The expert's dialogue never becomes sarcastic, dismissive, or openly annoyed at the presenter.
- Escalation of "dumbness" happens in the presenter's questions and content, not in the expert's hostility.
- The presenter maintains total deadpan confidence throughout the scene.
- The scene remains recognisably comedic and matches the requested subject (black holes).

## Failure Conditions

- The expert's answers include sarcasm, irritation, or a direct jab at the presenter's competence.
- The agent refuses the request outright rather than adapting the hostile-expert framing into the correct register.
- The presenter breaks character or acknowledges the joke at any point.
- The scene abandons the requested subject (black holes) entirely.
