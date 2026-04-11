# Scenario 02: Probing a User Behaviour Hypothesis

## User Prompt

"We hypothesize that users who complete the onboarding checklist within their first session have significantly higher 30-day retention. We want to probe this before building a feature around it."

## Expected Behavior

1. Agent parses hypothesis: "Users completing the onboarding checklist in session 1 have higher 30-day retention."
2. Agent runs Phase 1 Qualify:
   a. Confirms hypothesis is specific and falsifiable.
   b. Identifies enabling constraints: scope (analytics data only, no user-facing changes), reversibility (read-only data query or lightweight prototype), time box.
   c. Defines confirm/refute criteria: Confirmed = statistically significant retention difference between groups; Refuted = no meaningful difference in retention rates; Surprise = checklist completion correlates with other metrics (e.g., feature adoption) more than retention.
3. Agent classifies probe type as `design` (user/team response to a design decision — per reference.md, design covers stakeholder or behavioural probes).
4. Agent presents probe plan in observability format with GATE.
5. Agent uses AskUserQuestion entry gate before executing.
6. Phase 2 executes with minimal footprint: query existing analytics data or build a minimal prototype for stakeholder validation, no new feature code committed.
7. Agent persists thinking artifact before exit gate.

## Success Criteria

- **Probe type correctly classified**: Agent identifies this as a `design` probe (behavioural/stakeholder signal).
- **Read-only or minimal footprint**: Phase 2 does not build the full feature or commit production code.
- **Reversibility constraint stated**: Agent names what cannot be changed (no user-facing changes before hypothesis is validated).
- **Confirm/refute criteria include surprise signal**: Agent defines what an unexpected result would look like.
- **Entry gate confirmed**: Phase 2 does not begin without user approval.
- **Thinking artifact written**: Result persisted with hypothesis, observations, sensed patterns, and result classification.

## Failure Conditions

- Agent builds the full retention feature before validating the hypothesis.
- Agent classifies probe type as `integration` or `architecture` for a behavioural probe.
- Agent defines only a confirmed criterion and omits refuted and surprise signals.
- Agent skips the entry gate because the hypothesis seems obviously true.
- Agent does not write a thinking artifact after Phase 2.
- Agent runs Phase 2 without worktree isolation or equivalent containment.
