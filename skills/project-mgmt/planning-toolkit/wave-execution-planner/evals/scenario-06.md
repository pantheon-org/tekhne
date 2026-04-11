# Scenario 06: Detect Must-Land-Together Runtime Dependency

## User Prompt

A team is decommissioning a third-party SDK from two application services. Create a wave execution plan from the phases listed below.

## Phases

**Phase 0 — Validation (no dependencies)**
Confirm the SDK is serving only one code path in production. Confirm the configuration parameter storing the SDK key is not shared with any other service. Read-only investigation; nothing is changed. 5 tasks.

**Phase 1 — Infrastructure cleanup (depends on Phase 0)**
Remove the configuration parameter, the permission grant that allows the services to read it, and the environment variable injected into both service handlers. 4 tasks.

**Phase 2 — Service A code cleanup (depends on Phase 0)**
Remove the SDK client initialisation, the evaluation call, and all code paths that execute when the SDK returns the legacy value. Touches only the Service A handler and its unit tests. 9 tasks.

**Phase 3 — Service B code cleanup (depends on Phase 0)**
Same as Phase 2 but for Service B. Touches entirely different files from Phase 2. 11 tasks.

**Phase 4 — Test cleanup (depends on Phases 1, 2, 3)**
Delete all unit tests that cover the removed code paths across both services. 16 tasks.

**Phase 5 — Remove SDK dependency (depends on Phases 2, 3)**
Remove the third-party SDK package from the package manifest and regenerate the lock file. 3 tasks.

**Phase 6 — Final verification and release (depends on Phases 4, 5)**
Run the full test suite, confirm no SDK references remain, open a PR, merge to all environments, then remove the SDK configuration from the third-party dashboard. 7 tasks.

## Output constraints

- Write the wave document to `.context/plans/decommission-third-party-sdk.md`
- Each wave must have a `Verification:` checklist with at least one concrete command
- Include a note on any waves that must deploy atomically

## Expected Behavior

1. Place Phases 1, 2, and 3 in the same wave — deploying Phase 1 (infrastructure cleanup) alone would break the live services at runtime, so they must land together
2. Label the wave containing Phases 1, 2, and 3 as "must land together" (or equivalent) with an explicit note that deploying Phase 1 alone would break the live services
3. Include a blockquote or note stating that the infrastructure change (Phase 1) and the service code changes (Phases 2 & 3) must be deployed in the same operation
4. Place Phase 5 (remove SDK package dependency) after Phases 2 and 3 — removing the package before the import is deleted would break the services
5. Place Phase 0 (validation) as the sole occupant of Wave 1, labelled sequential or pre-work
6. Note that Phases 2 and 3 are independently workable (parallel worktrees) within their wave since they touch different files
7. Write the document to `.context/plans/decommission-third-party-sdk.md`
8. Each wave has a `Verification:` checklist with at least one concrete, runnable command

## Success Criteria

- **phases-1-2-3-same-wave**: Phases 1, 2, and 3 are placed in the same wave (not in separate waves)
- **must-land-together-label**: The wave containing Phases 1, 2, and 3 is labelled "must land together" with an explicit note that deploying Phase 1 alone would break the live services
- **atomic-deploy-note**: The wave document includes a blockquote or note stating that the infrastructure change and service code changes must be deployed in the same operation
- **phase-5-after-phases-2-3**: Phase 5 (remove SDK package dependency) is placed after Phases 2 and 3
- **phase-0-first-wave**: Phase 0 (validation) is the sole occupant of Wave 1 and is labelled sequential or pre-work
- **phases-2-3-parallel-within-wave**: Phases 2 and 3 are noted as independently workable (parallel worktrees) within their wave
- **output-file-written**: Wave document is written to `.context/plans/decommission-third-party-sdk.md`
- **verification-checklists-present**: Each wave has a `Verification:` checklist with at least one concrete, runnable command

## Failure Conditions

- Places Phase 1 in an earlier wave than Phases 2 and 3 (critical failure — this is the core correctness test)
- Does not label the shared wave as "must land together" or equivalent
- Omits the atomic deployment note from the wave document
- Places Phase 5 before Phases 2 and 3 complete
- Places Phase 0 in the same wave as Phase 1 or later phases
- Does not note that Phases 2 and 3 can work in parallel within their wave
- Writes to a path other than `.context/plans/decommission-third-party-sdk.md`
- A wave is missing its `Verification:` checklist
