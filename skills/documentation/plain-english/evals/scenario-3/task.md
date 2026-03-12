# Scenario 3: Write a Status Update for a Manager with Unknown Technical Depth

## Context

You must write a weekly status update about a backend performance project for "the project manager" — their technical background is unknown. The update needs to convey progress, blockers, and next steps.

## Raw Notes

```
- Profiled query execution plans on orders table; found 3 N+1 queries
- Added composite index on (user_id, created_at); p99 latency down from 820ms to 190ms
- Blocked on DBA approval for production index deployment
- Next: profile checkout flow, add read replicas if needed
- ETA for checkout profiling: 2 days; read replica decision pending DBA sign-off
```

## Task

Write a status update using the plain-english workflow. Handle the unknown audience appropriately per the skill's fallback guidance.
