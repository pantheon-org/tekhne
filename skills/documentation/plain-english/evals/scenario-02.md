# Scenario 02: Write an Architecture Decision Record Summary for Compliance Managers

## User Prompt

You need to summarize an Architecture Decision Record (ADR) for compliance managers who must approve or reject the proposed change within 48 hours. They are not engineers but understand risk and regulatory requirements.

Here is the input ADR (excerpt):

```
ADR-042: Migrate from on-prem PostgreSQL to AWS RDS Aurora (Multi-AZ)

Status: Proposed
Context: Current single-node PostgreSQL 12 instance lacks HA and PITR capabilities.
Decision: Adopt Aurora PostgreSQL 15 with Multi-AZ deployment and automated backups.
Consequences:
  - RPO reduced from ~24h (manual backup) to ~5 minutes (continuous WAL archiving).
  - RTO reduced from ~4h (manual restore) to ~15 minutes (failover).
  - Cost: $1,200/month increase. Requires SOC 2 Type II re-assessment for data residency.
  - Migration window: 6-hour downtime in Q3 maintenance window.
```

Write a compliance-manager summary of this ADR so they can make an approve/reject decision. Follow the plain-english workflow.

## Expected Behavior

1. Identify compliance managers as the audience and state the goal is an approve/reject decision
2. Open with the proposal, the key compliance impact, and the decision needed — not technical background
3. Translate technical terms (Aurora, Multi-AZ, PostgreSQL, PITR, WAL archiving) to plain language
4. Define acronyms (RTO, RPO, SOC 2, ADR, HA) on first use or in a glossary
5. Surface the SOC 2 re-assessment requirement, cost impact ($1,200/month), downtime window, and action items with owners and deadlines

## Success Criteria

- **Audience and outcome identified**: Response identifies compliance managers as the audience and states the goal is an approve/reject decision.
- **Decision request leads the document**: Opening paragraph states the proposal, the key compliance impact, and the decision needed — not technical background.
- **Technical terms translated**: Terms like Aurora, Multi-AZ, PostgreSQL, PITR, WAL archiving are translated to plain language.
- **Acronyms defined on first use**: RTO, RPO, SOC 2, ADR, HA are defined on first use or in a glossary.
- **Compliance, cost and action context included**: SOC 2 re-assessment requirement, cost impact ($1,200/month), downtime window, and action items with owners and deadlines are surfaced.

## Failure Conditions

- Audience and decision goal are not identified before the summary
- Opening paragraph starts with technical context rather than the proposal and decision needed
- Technical terms (Aurora, Multi-AZ, PostgreSQL, PITR, WAL archiving) appear without translation
- Acronyms (RTO, RPO, SOC 2, ADR, HA) are used without definitions
- SOC 2 re-assessment requirement, cost impact, or downtime window are omitted
