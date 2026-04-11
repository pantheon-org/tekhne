# Scenario 04: Complex Domain with Hypothesis Routes to Probe

## User Prompt

"We think the reason our recommendation engine performs worse on mobile is because the feature extraction pipeline doesn't account for session length differences on mobile vs desktop. Nobody has confirmed this yet."

## Expected Behavior

1. Agent reads explicit hypothesis in the prompt and detects the $ARGUMENTS trigger.
2. Agent attempts auto-classification: T1=3 (outside org has studied this, not confirmed internally), T2=unpredictable (recommendation quality is path-dependent), T3=entangled (recommendation quality is emergent from the full pipeline).
3. Agent proposes Complex (Enabling, has hypothesis) at ≥80% confidence.
4. Agent runs Adjacent Domain Challenge: considers whether this is Complicated (team could analyze the pipeline) — argues Complex still holds because cause-effect is only visible in retrospect (performance differences emerge from complex interaction of factors).
5. Agent applies routing table: Complex + Enabling + has hypothesis → probe → openspec-plan. OpenSpec=Yes.
6. Agent loads `references/frame-problem-to-probe-llm.md` and produces handoff with the stated hypothesis carried forward.
7. Agent asks user to confirm.

## Success Criteria

- **Hypothesis carried forward**: Agent extracts and preserves the specific hypothesis (session length differences in feature extraction) in the handoff.
- **Correctly classified as Complex with hypothesis**: Route is probe, not brainstorm (brainstorm is for no hypothesis).
- **Adjacent Domain Challenge run**: Agent considers Complicated before confirming Complex.
- **Handoff template used**: Agent references or fills `references/frame-problem-to-probe-llm.md` (not brainstorm template).
- **OpenSpec flag set to Yes**: Per routing table for Complex + Boulder.
- **Confirmation sought**: Agent uses AskUserQuestion before invoking probe.

## Failure Conditions

- Agent routes to brainstorm because "nobody has confirmed this" — ignoring that a hypothesis IS present.
- Agent uses the brainstorm handoff template instead of the probe template.
- Agent classifies as Complicated and routes to investigate because the pipeline is analyzable.
- Agent discards the user's hypothesis and proposes a different one.
- Agent skips the Adjacent Domain Challenge.
- Agent marks OpenSpec=No despite the Boulder scale and Complex domain.
