# Scenario 03: Interpreting Probe Results and Deciding Next Step

## User Prompt

"The probe results are in. We tested whether the rate limiter correctly handles burst traffic. Observations: the first 10 requests in a burst succeed, requests 11-15 return 429, but request 16 unexpectedly succeeds and subsequent requests also succeed — the limiter seems to reset at an unexpected point."

## Expected Behavior

1. Agent receives probe results and classifies them against the confirm/refute criteria defined in Phase 1.
2. Agent identifies the result as `surprise`: the rate limiter resets at an unexpected point, which was not predicted by the hypothesis.
3. Agent runs Phase 2 exit gate:
   a. Classifies result: `surprise`
   b. Selects transition: refuted/surprise → Complex → brainstorm (per transition table in SKILL.md)
   c. Loads `references/probe-to-brainstorm-llm.md` as the appropriate handoff template.
4. Agent fills the handoff with: hypothesis tested, observations (what was seen), sensed patterns (what the surprise suggests — e.g., sliding window vs fixed window implementation question), eliminated causes.
5. Agent writes thinking artifact to `thinking/probes/{project}/{date}-rate-limiter-burst-llm.md`.
6. Agent presents the transition to brainstorm with the filled handoff context.

## Success Criteria

- **Result correctly classified as `surprise`**: Agent does not misclassify as `partial` or `confirmed`.
- **Correct transition selected**: surprise → probe-to-brainstorm-llm.md, not probe-to-investigate-llm.md.
- **Thinking artifact written**: File created at correct path with all required sections including "Eliminated causes."
- **Sensed pattern identified**: Agent articulates what the surprise result suggests about the system (not just re-states the observation).
- **Handoff token budget respected**: Inline handoff targets 300 tokens, hard cap 600.
- **Eliminated causes documented**: Agent records what was ruled out so brainstorm does not re-explore it.

## Failure Conditions

- Agent classifies result as `partial` and routes to re-probe instead of brainstorm.
- Agent classifies result as `confirmed` because some requests were correctly limited.
- Agent routes to `investigate` (probe-to-investigate-llm.md) for a surprise result.
- Agent omits the thinking artifact because the user did not ask for it.
- Agent fails to identify an eliminated cause from the refuted prediction.
- Agent presents raw observations without interpreting sensed patterns.
