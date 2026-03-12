# Scenario 1: Write an Architecture Decision Record Summary for Compliance Managers

## Context

You need to summarize an Architecture Decision Record (ADR) for compliance managers who must approve or reject the proposed change within 48 hours. They are not engineers but understand risk and regulatory requirements.

## Input ADR (excerpt)

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

## Task

Write a compliance-manager summary of this ADR so they can make an approve/reject decision. Follow the plain-english workflow.
