# Task: Detect must-land-together runtime dependency

A team is decommissioning a third-party SDK from two application services.
Create a wave execution plan from the phases listed below.

## Phases

**Phase 0 — Validation (no dependencies)**
Confirm the SDK is serving only one code path in production. Confirm the
configuration parameter storing the SDK key is not shared with any other
service. Read-only investigation; nothing is changed. 5 tasks.

**Phase 1 — Infrastructure cleanup (depends on Phase 0)**
Remove the configuration parameter, the permission grant that allows the
services to read it, and the environment variable injected into both service
handlers. 4 tasks.

**Phase 2 — Service A code cleanup (depends on Phase 0)**
Remove the SDK client initialisation, the evaluation call, and all code paths
that execute when the SDK returns the legacy value. Touches only the Service A
handler and its unit tests. 9 tasks.

**Phase 3 — Service B code cleanup (depends on Phase 0)**
Same as Phase 2 but for Service B. Touches entirely different files from
Phase 2. 11 tasks.

**Phase 4 — Test cleanup (depends on Phases 1, 2, 3)**
Delete all unit tests that cover the removed code paths across both services.
16 tasks.

**Phase 5 — Remove SDK dependency (depends on Phases 2, 3)**
Remove the third-party SDK package from the package manifest and regenerate
the lock file. 3 tasks.

**Phase 6 — Final verification and release (depends on Phases 4, 5)**
Run the full test suite, confirm no SDK references remain, open a PR, merge
to all environments, then remove the SDK configuration from the third-party
dashboard. 7 tasks.

## Output constraints

- Write the wave document to `.context/plans/decommission-third-party-sdk.md`
- Each wave must have a `Verification:` checklist with at least one concrete command
- Include a note on any waves that must deploy atomically
