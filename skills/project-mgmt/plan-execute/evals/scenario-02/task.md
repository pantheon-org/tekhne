# Scenario 02: Multi-Phase with Divergence and Amendment

## User Prompt

"Continue the plan from .context/plans/standards-compliance-remediation. Execute Phase 02 and Phase 03."

## Setup

Phase 01 is already completed and committed. The CHECKLIST.md exists with Phase 01 marked VALIDATED. Phase 02 involves splitting a file (`helpers.ts`) into per-function modules. During implementation, the agent discovers the file `stars-filter.ts` also needs splitting because it re-exports filter logic that Phase 02 was supposed to cover but the plan missed it.

## Expected Behavior

1. Verify Phase 01 is VALIDATED in the checklist before starting Phase 02.
2. For Phase 02 tasks, follow the single-phase execution protocol (baseline already exists, so use it for regression comparison).
3. When the agent discovers `stars-filter.ts` also needs splitting:
   a. **Stop** working on the current task.
   b. Document the divergence in CHECKLIST.md under a `## Divergence` section:
      - Expected: Only helpers.ts is split
      - Actual: stars-filter.ts also needs splitting
      - Reason: It re-exports filter logic covered by Phase 02 scope
   c. Amend the plan using `plan-create` amendment pattern:
      - Add a new task to Phase 02 (or Phase 03 if more appropriate)
      - Update the phase README.md
   d. Update the checklist with the amended task.
   e. Continue from where stopped, implementing the amended tasks.
4. After Phase 02 is validated and committed, verify Phase 02 is VALIDATED before starting Phase 03.
5. Execute Phase 03 with the same checklist discipline.
6. At the end of Phase 03, run the phase validation suite, commit, and update the checklist.

## Success Criteria

- Phase 01 is verified as VALIDATED before Phase 02 begins.
- Divergence is documented with Expected, Actual, and Reason fields.
- Plan is amended with a new task before continuing.
- Checklist is updated to reflect the amended scope.
- Phase 02 and Phase 03 are each committed separately with evidence.
- Regression diff shows no new failures vs baseline.

## Failure Conditions

- Agent starts Phase 02 without verifying Phase 01 is done.
- Agent silently includes the extra file split without documenting divergence.
- Agent continues working without amending the plan.
- Agent squashes Phase 02 and Phase 03 into a single commit.
- Agent claims DONE but the divergence section is missing from the checklist.
