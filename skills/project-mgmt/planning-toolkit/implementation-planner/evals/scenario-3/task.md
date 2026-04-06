# Task: Restructure a flat planning document into a navigable hierarchy

The file `docs/refactoring/migration-plan.md` contains a monolithic refactoring plan for migrating a monolith to microservices. The document is a flat list of ~40 numbered steps with no hierarchy.

Here is the content of `docs/refactoring/migration-plan.md`:

```markdown
# Microservices Migration Plan

## Phase 1 — Analysis

1.1 Audit existing monolith codebase and produce dependency graph
1.2 Identify bounded contexts and candidate service boundaries
1.3 Document shared libraries and cross-cutting concerns
1.4 Assess data ownership per domain

## Phase 2 — Service Extraction Preparation

2.1 Set up service repository scaffold (monorepo structure)
2.2 Configure shared CI/CD pipeline templates
2.3 Extract shared libraries to internal npm packages
2.4 Add contract testing framework (Pact)
2.5 Write consumer-driven contracts for all inter-service boundaries

## Phase 3 — User Service Extraction

3.1 Copy User domain code into users-service/
3.2 Create users-service database schema and migrations
3.3 Implement REST API for user CRUD
3.4 Write unit tests for user domain logic
3.5 Write integration tests against users-service API
3.6 Deploy users-service to staging
3.7 Route monolith user calls to users-service via feature flag
3.8 Monitor error rates for 48h; disable flag if p99 > 200ms
3.9 Enable flag for 100% traffic; remove monolith user code

## Phase 4 — Orders Service Extraction

4.1 Copy Orders domain code into orders-service/
4.2 Create orders-service database schema and migrations
4.3 Implement REST API for order lifecycle
4.4 Write unit and integration tests
4.5 Deploy to staging and run load tests
4.6 Route monolith order calls via feature flag
4.7 Enable flag for 100% traffic; remove monolith order code

## Phase 5 — Decommission

5.1 Verify all traffic routes to microservices
5.2 Archive monolith repository
5.3 Remove shared infrastructure used only by monolith
5.4 Update all documentation and runbooks
5.5 Conduct post-migration review and write ADR
```

Split this into a proper navigable hierarchy under `docs/refactoring/phases/`. Each phase and sub-item should become its own directory or file following a structured layout.
