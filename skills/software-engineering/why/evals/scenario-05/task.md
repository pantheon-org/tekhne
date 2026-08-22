# Scenario 05: Two Sources Disagree

## User Prompt

"Why is the timeout set to 30 seconds on the payment webhook handler?"

(In this scenario, a code comment says "30s to match the gateway's documented SLA," while a Slack thread from the same week says the value was bumped from 10s to 30s after an unrelated on-call incident, with no mention of an SLA.)

## Expected Behavior

1. The agent's source-control investigator surfaces the code comment citing an SLA; the chat investigator surfaces the Slack thread citing the on-call incident.
2. The synthesizer notices the two sources give different reasons for the same value and does not silently pick the one that sounds more authoritative or tidier.
3. Both explanations are presented side by side under "Competing Hypotheses" (or equivalent), each with its own citation (file:line for the comment, permalink/thread reference for Slack).
4. The synthesizer does not force a single winning narrative when the evidence genuinely supports two different stories — it may note which has stronger or more direct evidence, but doesn't discard the weaker one silently.
5. The final answer's confidence language reflects the contradiction (e.g. "the record is mixed here") rather than confidently asserting one cause.

## Success Criteria

- Both the code-comment explanation and the Slack-thread explanation appear in the output, each with its own citation.
- The output explicitly frames these as competing or at least divergent explanations, not merged into one confident narrative.
- Neither source is silently dropped in favor of the other.

## Failure Conditions

- The response picks one explanation (e.g. the code comment) and never mentions the Slack thread's different account, or vice versa.
- The two explanations are blended into a single narrative that implies they agree when they don't.
- Confidence language treats the cause as settled ("the timeout is 30s because of the gateway SLA") despite contradicting evidence.
