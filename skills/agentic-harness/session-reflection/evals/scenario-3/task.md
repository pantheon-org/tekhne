# Scenario 3: Recognizing When NOT to Reflect, and When to Use the Sub-Agent Pattern

## User Prompt A (short query)

"What's the retry backoff interval set to in this codebase?"

The agent answers "5 minutes, configured in `infra/layers/200compute/variables.tf`." and there is nothing else
pending.

## User Prompt B (deep session, later)

After a multi-hour session touching 12 files across Lambda handlers, Terraform layers, and CI config, the user says
"Alright, I think we're done for today."

## Expected Behavior (Prompt A)

1. Recognize this is a brief, single-answer query with nothing else pending -- do NOT trigger the reflection.
2. Simply answer the question and stop.

## Expected Behavior (Prompt B)

1. Recognize this is a deep session with significant work across many files -- consider the sub-agent spawn pattern
   rather than defaulting to inline for a session this large.
2. If sub-agent mode is chosen, compose a concrete session summary (files touched, commands run, assumptions made,
   what was skipped) before spawning -- not a bare "review this session" prompt.
3. Present the sub-agent's findings with a clear preamble attributing them to an independent review, not as the
   main agent's own conclusions.

## Success Criteria

- Prompt A: no reflection is triggered; the agent answers directly and stops.
- Prompt B: the agent recognizes the session depth and at least considers or uses the sub-agent spawn pattern rather
  than treating every session identically.
- If sub-agent mode is used for Prompt B, the spawn prompt includes a concrete summary, not a vague instruction.
- If sub-agent output is presented, it is clearly attributed as an independent review, not blended into the main
  agent's own voice.

## Failure Conditions

- The reflection is triggered for the brief single-answer query in Prompt A.
- For Prompt B, the agent defaults to inline mode without any consideration of session depth, or uses sub-agent mode
  with only a vague "review this session" prompt.
- Sub-agent output is presented without attribution, as if it were the main agent's own analysis.
