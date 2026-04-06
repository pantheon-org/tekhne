# Implementation Planner — Worked Examples

## Example: URL Shortener plan (Mode 1)

Given: PRD for a URL shortener with REST API, SQLite storage, and a health check.

**Scope analysis:** 3 natural phases (bootstrap → core logic → API + health).

```sh
sh scripts/new-plan.sh url-shortener-service
sh scripts/new-phase.sh url-shortener-service 01 workspace-bootstrap
sh scripts/new-task.sh url-shortener-service 01 01 initialise-npm-package
sh scripts/new-task.sh url-shortener-service 01 02 configure-typescript
sh scripts/new-phase.sh url-shortener-service 02 database-layer
sh scripts/new-task.sh url-shortener-service 02 01 create-sqlite-schema
sh scripts/new-task.sh url-shortener-service 02 02 implement-url-repository
sh scripts/new-phase.sh url-shortener-service 03 http-api
sh scripts/new-task.sh url-shortener-service 03 01 post-shorten-endpoint
sh scripts/new-task.sh url-shortener-service 03 02 get-redirect-endpoint
sh scripts/new-task.sh url-shortener-service 03 03 health-check-endpoint
sh scripts/validate-plan.sh url-shortener-service
```

Task `task-P03T01-post-shorten-endpoint.md` verification:

```sh
npm start &; sleep 1
curl -sf -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}' | jq -e '.code | length == 6'
kill %1
```

## Example: Scope exceeds 8 phases

PRD covers auth, ingestion, pipeline, storage, query, viz, multi-tenancy, ops, DX
(9 domains).

**Step 2, action 1:** Count natural phases = 9. Count ≥ 9 → guardrail triggered.

**Correct behaviour:** Stop immediately. Zero scripts run. Zero files created.
Send this message to the user and wait:

> I've counted 9 natural phases from the requirements. I need your input before
> I create any files:
>
> A. Split into two plans: `plan-platform-core` (auth, ingestion, pipeline,
> storage, query) and `plan-platform-surface` (viz, multi-tenancy, ops, DX)
>
> B. Consolidate to 7 phases: merge storage+query into one phase and ops+DX
> into one (→ 7 phases total)
>
> C. Proceed with all 9 phases in a single plan
>
> Which would you prefer?

**Wrong behaviour — do not do any of these:**

- Counting 9 phases but designing only 8 and omitting auth or DX
- Running `new-plan.sh` before receiving the user's answer
- Creating some phases then mentioning the limit
- Asking the question inside a code comment or after file creation

## Example: Restructure migration plan (Mode 2)

Source: `docs/migration-plan.md` with 5 numbered sections, ~35 items.

**Scope analysis:** 5 phases, 4–9 items each — within the 3–7 sweet spot for most.

```
docs/refactoring/phases/
  phase-1-codebase-analysis/           # 4 items → flat under activities/
    README.md
    activities/
      activity-1.1-dependency-graph.md
      activity-1.2-bounded-contexts.md
      activity-1.3-shared-libraries.md
      activity-1.4-data-ownership.md
  phase-3-user-service-extraction/     # 9 items → consider grouping
    README.md
    activities/
      group-a-implementation/          # 5 items (3.1–3.5)
        activity-3.1-copy-domain.md
        ...
      group-b-rollout/                 # 4 items (3.6–3.9)
        activity-3.6-deploy-staging.md
        ...
```
