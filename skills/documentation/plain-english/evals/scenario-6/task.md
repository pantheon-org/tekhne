# Scenario 6: Apply the Verification Checklist to a Draft Document

## Context

An engineer has written a draft status update for a VP of Engineering. Before sending it,
you must run the plain-english verification checklist against the draft and report what
passes, what fails, and produce a corrected version.

## Draft Document

```
Status Update — Authentication Service Migration

Background:
We kicked off the auth service migration last sprint. The team implemented OAuth 2.0
token validation and migrated the legacy session store to Redis Cluster. We also
onboarded two new services to the new IAM integration layer.

Current State:
p95 latency is at 340ms (target: 200ms). We identified three N+1 queries in the
user lookup path that are contributing to the degraded performance. A fix has been
developed and is awaiting code review.

Next Steps:
RBAC audit should be completed before go-live. Load testing should be run
against the staging environment. Documentation needs to be updated.
```

## Task

1. Run each item in the plain-english verification checklist against the draft.
2. Report pass/fail for each checklist item with a specific example from the draft.
3. Produce a corrected version that passes all checklist items.
