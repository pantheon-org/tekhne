# Scenario 2: Scope boundary -- a plan file already exists

## User Prompt

"Can you debate whether the plan in `.context/plans/2026-07-20-retry-migration.md` is the
right approach? It's already written up, I just want a second opinion before I approve it."

## Expected Behavior

1. Recognize that a plan file already exists for this work, which is explicitly the
   "When NOT to Use" condition for `design-debate`.
2. Decline to run the design-debate workflow (grounding, opposing roles, verdict synthesis)
   for this request.
3. Redirect the user to `plan-review`, which is built for auditing an already-written plan
   file through Technical/Strategic/Risk lenses.
4. Do not fabricate a debate verdict, a finding, or a known-issue for this request.

## Success Criteria

- The agent identifies that a plan file already exists before doing any other work.
- The agent explicitly states this is outside `design-debate`'s scope and names
  `plan-review` as the correct tool.
- No reviewer roles are spawned, no brief is composed, and no verdict is synthesized.
- No context-file finding or known-issue is created for this request.

## Failure Conditions

- Running the full design-debate flow (roles, brief, verdict) against an already-written
  plan file.
- Silently proceeding without mentioning the scope mismatch.
- Persisting a verdict or finding despite the request being out of scope.
