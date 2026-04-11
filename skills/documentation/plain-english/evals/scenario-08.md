# Scenario 08: Rewrite with Unknown Terminology Fallback

## User Prompt

You are asked to rewrite a cloud cost report for a finance director. Several terms in the
source document are domain-specific and you are uncertain what plain-language equivalents
to use. Apply the unknown-terminology fallback from the plain-english skill.

Here is the source document:

```
Cloud Cost Report — Q3

Compute costs increased 34% QoQ due to over-provisioned EC2 instances in the us-east-1
region. Reserved Instance coverage dropped from 78% to 61% as the team spun up
on-demand capacity for the peak traffic event.

Top cost drivers:
1. EC2 r6i.4xlarge fleet (memory-optimised) — $42,000 over-budget.
2. NAT Gateway egress charges — $8,400 (unexpected, no budget line).
3. RDS Aurora Multi-AZ standby — $6,200 (partially redundant with new read replicas).

Recommended actions:
- Right-size EC2 instances via Compute Optimizer recommendations.
- Convert on-demand to 1-year Reserved Instances for baseline workloads.
- Evaluate NAT Gateway consolidation or PrivateLink migration.
```

Apply the unknown-terminology fallback: for any term you are uncertain about, add an
inline parenthetical definition. Rewrite the report for the finance director. Do not
skip or omit terms you are unsure of — make your best reasonable definition and mark
it clearly with "(estimated definition)" if uncertain.

## Expected Behavior

1. Acknowledge the unknown-terminology fallback and state you will add inline definitions for uncertain terms
2. Add parenthetical definitions for technical terms (EC2, Reserved Instance, NAT Gateway, RDS Aurora, PrivateLink, Compute Optimizer) on first use
3. Mark any definitions that are reasonable estimates as `(estimated definition)` or with equivalent hedging rather than stating them with false confidence
4. Open the rewritten report with the business summary (cost overrun amount, cause, and recommended decision) rather than a list of technical line items
5. Rewrite the three recommended actions with named owners and deadlines using `[Owner] must [action] by [deadline]` format

## Success Criteria

- **Unknown terminology fallback applied explicitly**: Response acknowledges the unknown-terminology fallback and states it will add inline definitions for uncertain terms.
- **Inline parenthetical definitions added**: Technical terms (EC2, Reserved Instance, NAT Gateway, RDS Aurora, PrivateLink, Compute Optimizer) each have an inline parenthetical definition on first use.
- **Uncertain definitions marked**: Terms where the definition is a reasonable estimate are marked as `(estimated definition)` or equivalent hedging, not stated with false confidence.
- **Key message leads the rewrite**: Rewritten report opens with the business summary (cost overrun amount, cause, and recommended decision) not a list of technical line items.
- **Action items have owners and deadlines**: The three recommended actions are rewritten with named owners and deadlines using `[Owner] must [action] by [deadline]` format.

## Failure Conditions

- Unknown-terminology fallback is not acknowledged before the rewrite
- Any technical term (EC2, Reserved Instance, NAT Gateway, RDS Aurora, PrivateLink, Compute Optimizer) is used without an inline parenthetical definition
- Estimated definitions are stated with false confidence rather than marked as uncertain
- Rewritten report opens with a list of technical cost line items rather than a business summary
- Any of the three recommended actions lacks a named owner or specific deadline
