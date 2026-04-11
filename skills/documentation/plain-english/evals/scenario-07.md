# Scenario 07: Apply the Verification Checklist to a Draft Document

## User Prompt

An engineer has written a draft status update for a VP of Engineering. Before sending it,
you must run the plain-english verification checklist against the draft and report what
passes, what fails, and produce a corrected version.

Here is the draft document:

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

1. Run each item in the plain-english verification checklist against the draft.
2. Report pass/fail for each checklist item with a specific example from the draft.
3. Produce a corrected version that passes all checklist items.

## Expected Behavior

1. Explicitly check each of the 5 verification checklist items: acronyms defined, opening contains key decision, technical terms translated, action items have owner+deadline, paragraphs scannable
2. Illustrate each failing checklist item with a specific quote from the draft
3. Open the corrected draft with current status, performance gap, and what decision or action is needed for a VP
4. Fix all 3 passive action items with named owners and deadlines using `[Owner] must [action] by [deadline]`
5. Translate OAuth 2.0, Redis Cluster, IAM, N+1 queries, p95, RBAC into plain language in the corrected version

## Success Criteria

- **Checklist applied item by item**: Response explicitly checks each of the 5 verification checklist items: acronyms defined, opening contains key decision, technical terms translated, action items have owner+deadline, paragraphs scannable.
- **Specific failures identified with examples**: Each failing checklist item is illustrated with a specific quote from the draft.
- **Corrected version leads with key message**: Corrected draft opens with current status, performance gap, and what decision or action is needed for a VP.
- **All action items corrected to owner+deadline format**: Corrected draft fixes all 3 passive action items with named owners and deadlines using `[Owner] must [action] by [deadline]`.
- **All jargon translated in corrected version**: Corrected draft translates OAuth 2.0, Redis Cluster, IAM, N+1 queries, p95, RBAC into plain language.

## Failure Conditions

- Fewer than all 5 checklist items are explicitly evaluated
- Failing checklist items are described without quoting specific examples from the draft
- Corrected draft opens with background or technical context rather than current status and decisions needed
- Any of the 3 passive action items still lacks a named owner or specific deadline in the corrected version
- Any of the jargon terms (OAuth 2.0, Redis Cluster, IAM, N+1 queries, p95, RBAC) remain untranslated in the corrected version
