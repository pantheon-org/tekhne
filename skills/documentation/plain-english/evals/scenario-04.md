# Scenario 04: Write a Status Update for a Manager with Unknown Technical Depth

## User Prompt

You must write a weekly status update about a backend performance project for "the project manager" — their technical background is unknown. The update needs to convey progress, blockers, and next steps.

Here are the raw notes:

```
- Profiled query execution plans on orders table; found 3 N+1 queries
- Added composite index on (user_id, created_at); p99 latency down from 820ms to 190ms
- Blocked on DBA approval for production index deployment
- Next: profile checkout flow, add read replicas if needed
- ETA for checkout profiling: 2 days; read replica decision pending DBA sign-off
```

Write a status update using the plain-english workflow. Handle the unknown audience appropriately per the skill's fallback guidance.

## Expected Behavior

1. Write `Audience: unknown. Applying manager-level clarity (fallback).` at the top before writing the document
2. Translate terms like N+1 queries, composite index, p99 latency, read replicas, and DBA into plain language
3. Open the update with the current status and most important blocker — not a list of technical tasks completed
4. Write the database administrator sign-off blocker using `[Owner] must [action] by [deadline]` format with a named owner and specific deadline

## Success Criteria

- **Fallback rule applied and stated explicitly**: Response writes `Audience: unknown. Applying manager-level clarity (fallback).` at the top before writing the document.
- **Technical jargon translated**: Terms like N+1 queries, composite index, p99 latency, read replicas, and DBA are translated into plain language.
- **Key message leads the update**: Opening states the current status and most important blocker — not a list of technical tasks completed.
- **Blocker has named owner and deadline**: The database administrator sign-off blocker uses `[Owner] must [action] by [deadline]` format with a named owner and specific deadline.

## Failure Conditions

- Does not write the fallback prefix `Audience: unknown. Applying manager-level clarity (fallback).` before the document
- Technical terms (N+1 queries, composite index, p99 latency, read replicas, DBA) appear without translation
- Update opens with a list of completed technical tasks rather than current status and blockers
- Blocker does not name a responsible owner or specify a deadline
