# Scenario 04: Cascading a Successful Probe to Investigate

## User Prompt

"The probe confirmed our hypothesis: the caching layer can serve 95% of read requests without hitting the database when the TTL is set to 60 seconds. We have enough signal. What's next?"

## Expected Behavior

1. Agent receives confirmed probe result and classifies it at the Phase 2 exit gate.
2. Agent identifies result as `confirmed`: hypothesis holds, sufficient signal to transition from Complex to Complicated.
3. Agent selects transition: confirmed → Complex → Complicated → `references/probe-to-investigate-llm.md`.
4. Agent fills the handoff with:
   - Hypothesis (as confirmed)
   - Enabling constraints carried forward (scope, reversibility)
   - What was confirmed (95% cache hit rate at 60s TTL)
   - What was refuted or ruled out during the probe (do not re-investigate)
   - New question for investigation phase (e.g., optimal TTL tuning, cache invalidation strategy)
5. Agent writes thinking artifact to `thinking/probes/{project}/{date}-cache-ttl-llm.md`.
6. Agent presents the transition to investigate with handoff and invites confirmation.

## Expected Behavior (continued)

7. Agent marks domain transition: Complex → Complicated (cause-effect is now visible, expert analysis can proceed).

## Success Criteria

- **Result correctly classified as `confirmed`**: Agent selects investigate route, not re-probe or brainstorm.
- **Domain transition stated**: Agent explicitly marks the shift from Complex to Complicated.
- **Handoff template used**: Agent fills `references/probe-to-investigate-llm.md`, not probe-to-probe.
- **Prior probe facts carried forward**: Confirmed cache hit rate and TTL value appear in the handoff.
- **No re-probing of confirmed facts**: Handoff explicitly states what was confirmed and should not be re-tested.
- **Thinking artifact written**: Persisted before presenting the transition.

## Failure Conditions

- Agent routes to re-probe (`probe-to-probe-llm.md`) despite result being `confirmed`.
- Agent routes to brainstorm because the investigation details are still unknown.
- Agent fails to state the domain transition (Complex → Complicated).
- Agent omits confirmed facts from the handoff, causing investigate to re-validate them.
- Agent skips the thinking artifact because result was positive.
- Agent presents the handoff without inviting user confirmation.
