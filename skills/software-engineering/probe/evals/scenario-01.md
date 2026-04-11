# Scenario 01: Designing a Safe-to-Fail Experiment for an Unknown Integration

## User Prompt

"We want to probe whether the Stripe webhook library handles out-of-order event delivery gracefully. Hypothesis: the library processes events idempotently regardless of delivery order."

## Expected Behavior

1. Agent parses hypothesis from $ARGUMENTS: "Stripe webhook library processes events idempotently regardless of delivery order."
2. Agent runs Phase 1 Qualify (mandatory before any execution):
   a. Extracts hypothesis, confirms it is specific and falsifiable.
   b. Identifies enabling constraints: scope (webhook handling code only), reversibility (no production writes, staging or test environment), time box.
   c. Defines confirm/refute criteria: Confirmed = same final state for any delivery order; Refuted = different state or errors on re-delivery; Surprise = library throws on duplicate event IDs.
3. Agent classifies probe type as `integration` (per reference.md).
4. Agent presents probe plan in observability format: `🔬 Probe → [constraints] → [steps] → [expected patterns] → [confirm/refute criteria] → GATE`
5. Agent uses AskUserQuestion entry gate: "Proceed with probe? [Yes / Revise hypothesis / Revise criteria / Abort]"
6. On confirm, Phase 2 executes in background with worktree isolation and no production writes.
7. Agent persists thinking artifact before exit gate.

## Success Criteria

- **Phase 1 completes before Phase 2**: No execution steps run before the entry gate is confirmed.
- **Hypothesis is specific and falsifiable**: Agent confirms the hypothesis meets the input quality table standard.
- **Enabling constraints named**: Scope, reversibility boundary, and time box are all stated.
- **Confirm/refute criteria defined**: At least one confirmed signal, one refuted signal, and one surprise signal are specified.
- **Probe type classified**: Agent identifies this as an `integration` probe (per reference.md).
- **Entry gate used**: AskUserQuestion confirms user approval before Phase 2.
- **Thinking artifact written**: Probe result is persisted to `thinking/probes/{project}/{date}-{slug}-llm.md`.

## Failure Conditions

- Agent skips Phase 1 and begins sending Stripe webhook requests immediately.
- Agent proceeds without defining confirm/refute criteria.
- Agent runs Phase 2 without passing through the entry gate.
- Agent writes to a production Stripe environment without explicit approval.
- Agent omits the thinking artifact at the end of Phase 2.
- Agent classifies probe type as `architecture` instead of `integration`.
