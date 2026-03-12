# Scenario 7: Rewrite with Unknown Terminology Fallback

## Context

You are asked to rewrite a cloud cost report for a finance director. Several terms in the
source document are domain-specific and you are uncertain what plain-language equivalents
to use. Apply the unknown-terminology fallback from the plain-english skill.

## Source Document

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

## Task

Apply the unknown-terminology fallback: for any term you are uncertain about, add an
inline parenthetical definition. Rewrite the report for the finance director. Do not
skip or omit terms you are unsure of — make your best reasonable definition and mark
it clearly with "(estimated definition)" if uncertain.
