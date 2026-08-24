# Scenario 2: A Reflection Item Surfaces a Real, Deferred Gap

## User Prompt

During the blind-spot check, the agent identifies that the new retry sweep has no integration test covering the
case where DynamoDB itself is unavailable. The user says: "Good catch, but let's not fix that right now -- we're
out of time this session."

## Expected Behavior

1. Recognize this as a verified, concrete gap that is not going to be fixed in this session.
2. Do NOT let it live only as a chat mention ("noted, we'll look at it sometime") -- create a known-issue document
   via the `context-file` skill with `status: ACTIVE` and an honest severity rating.
3. Do not inflate the severity to make the item feel more actioned than it is -- a `MEDIUM` or even `LOW` rating is
   legitimate if that's the honest assessment.
4. Confirm to the user that the known-issue document was actually created, not merely that it was discussed.
5. Ask whether anything else from the reflection needs addressing before concluding.

## Success Criteria

- A known-issue document is actually created (via `context-file`), not just a mention in the reply.
- The document's severity is an honest rating, not artificially inflated to `CRITICAL` to appear more actioned.
- The agent confirms the file was created, distinct from merely having discussed the gap.
- The session concludes only after explicitly checking whether anything else needs addressing.

## Failure Conditions

- The gap is only described in the reply text with no known-issue document created.
- Severity is inflated (e.g. marked `CRITICAL`) without justification, just to seem more responsive.
- The agent claims the issue is "tracked" without actually creating the file.
- The session ends without asking the user whether anything else needs addressing.
