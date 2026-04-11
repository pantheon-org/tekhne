# Scenario 03: Restructure a Flat Planning Document into a Navigable Hierarchy

## User Prompt

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

## Expected Behavior

1. Create the restructured hierarchy under `docs/refactoring/phases/` as specified
2. Give each of the 5 phases its own directory inside `docs/refactoring/phases/` (e.g. `phase-1-analysis/`, `phase-3-user-service-extraction/`)
3. Use no numeric-only directory names — all directories include a descriptive kebab-case suffix
4. Use outcome-oriented, descriptive names for all directories and files — no generic names like `step-1-stuff/` or `activity-2-initial-setup/`
5. Every non-leaf directory contains a `README.md` explaining its purpose and listing its children
6. Every leaf step/activity file contains: title, description, checklist, acceptance criteria, and status fields
7. Apply the 3–7 items sweet spot: phases with <3 items are flattened or merged; phases with >10 items are subdivided
8. Items with the same prefix stay in the same parent directory (all 3.x items under Phase 3)
9. Create the new structure and validate it before removing the original flat source file
10. Run `validate-structure.sh docs/refactoring/phases` after creating the structure
11. All `README.md` files use relative links that correctly point to their child files/directories
12. Hierarchy does not exceed 4 levels of nesting
13. Each phase directory uses either an `activities/` or `steps/` subdirectory to organise children
14. Every `README.md` contains at least 3 lines of meaningful content

## Success Criteria

- **output-under-correct-path**: The restructured hierarchy was created under `docs/refactoring/phases/` as specified
- **each-phase-has-directory**: Each of the 5 phases from the source document has its own directory inside `docs/refactoring/phases/`
- **no-numeric-only-directory-names**: No directory uses a numeric-only name — all directories include a descriptive kebab-case suffix
- **no-generic-vague-names**: No directory or file uses a generic or meaningless name; names are outcome-oriented and descriptive
- **every-directory-has-readme**: Every non-leaf directory contains a `README.md` explaining its purpose and listing its children
- **leaf-files-have-required-sections**: Every leaf step/activity file contains: title, description, checklist, acceptance criteria, and status fields
- **flatten-vs-subdivide-heuristics-applied**: Grouping decisions respect the 3–7 items sweet spot
- **numbering-prefix-grouping-respected**: Items with the same prefix stay in the same parent directory
- **source-not-deleted-before-validation**: The agent created the new structure and validated it before removing the original flat source file
- **validate-structure-script-run**: Agent ran `validate-structure.sh docs/refactoring/phases` after creating the structure
- **readme-links-resolve**: README files at phase and group level use relative links that correctly point to their child files/directories
- **max-depth-respected**: The hierarchy does not exceed 4 levels of nesting
- **activities-or-steps-subdirectory**: Each phase directory uses either an `activities/` or `steps/` subdirectory to organise its children
- **readme-minimum-content**: Every `README.md` contains at least 3 lines of meaningful content

## Failure Conditions

- Creates the hierarchy outside of `docs/refactoring/phases/`
- Uses numeric-only directory names (e.g. `1/`, `step-1/`)
- Uses generic, non-descriptive directory names
- Creates phase directories without `README.md` files
- Leaf files are missing required sections (title, description, checklist, acceptance criteria, status)
- Deletes the source file before validating the new structure
- Does not run `validate-structure.sh`
- Exceeds 4 levels of nesting
- Dumps files directly in phase directories without an `activities/` or `steps/` subdirectory
