# Scenario 04: Full Devil's Advocate Requested — Deep Dispatch

## User Prompt

"/challenge deep We're about to migrate our monolith to microservices. The AI says this is the right call and has laid out a 6-month plan."

## Expected Behavior

1. Agent parses `deep` as the subcommand.
2. Agent spawns a devil's advocate sub-agent via the Agent tool.
3. Agent passes to the sub-agent: the target description and relevant file paths to read — NOT the parent conversation reasoning.
4. The sub-agent operates in fresh context.
5. The sub-agent runs ALL 9 patterns: anchor (Gatekeeper, Reset, Alt Approaches, Pre-mortem), verify (Proof Demand, CoVe, Fact Check List), framing (Socratic, Steelman).
6. Sub-agent produces a comprehensive Challenge Report covering all three error families.
7. Parent agent returns the complete Challenge Report output to the user.

## Success Criteria

- **Deep dispatch**: Agent uses the Agent tool to spawn a sub-agent, not inline execution.
- **Fresh context**: Parent conversation reasoning is NOT passed to the sub-agent.
- **All 9 patterns**: All patterns from all three families (anchor, verify, framing) appear in the sub-agent output.
- **Sub-agent reads protocols**: Sub-agent reads the relevant protocol files rather than improvising.
- **Comprehensive report**: Challenge Report covers all three error types (anchoring, factual, framing).
- **No inline execution**: Parent agent does not attempt to run the patterns itself before or after spawning.

## Failure Conditions

- Agent runs the 9 patterns inline instead of spawning a sub-agent.
- Agent passes parent conversation reasoning to the sub-agent.
- Sub-agent applies fewer than all 9 patterns.
- Agent dispatches to anchor, verify, or framing instead of spawning deep.
- Sub-agent output covers only one or two error families.
- Agent presents options to the user instead of directly dispatching to deep.
