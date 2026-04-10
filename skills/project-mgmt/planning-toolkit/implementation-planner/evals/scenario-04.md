# Scenario 04: Create an Implementation Plan for a Large-Scale SaaS Platform

## User Prompt

Create a complete implementation plan for the following system.

## Project: Multi-Tenant SaaS Analytics Platform

Build a production-grade multi-tenant analytics platform with the following capabilities:

### Authentication & Authorisation
- OAuth 2.0 / OpenID Connect with Google and GitHub providers
- Role-based access control (owner, admin, analyst, viewer)
- API key management for service accounts

### Data Ingestion
- HTTP event ingestion endpoint (100k events/sec throughput)
- Kafka consumer for batch ingestion
- CSV and JSON bulk import via S3
- Webhook delivery system for downstream integrations

### Data Pipeline
- Real-time processing with Apache Flink
- Hourly and daily aggregation jobs
- Data quality validation layer
- Schema registry integration

### Storage
- PostgreSQL for tenant metadata and config
- ClickHouse for analytical queries
- S3 for raw event archive
- Redis for caching and session storage

### Query Engine
- SQL-over-HTTP query API
- Saved queries with parameterisation
- Query result caching with TTL
- Export to CSV/Parquet/JSON

### Visualisation
- Dashboard builder with drag-and-drop layout
- Chart library (line, bar, pie, funnel, cohort)
- Scheduled report delivery by email
- Embeddable dashboard iframes with token auth

### Multi-Tenancy
- Tenant provisioning and onboarding flow
- Per-tenant resource limits and quota enforcement
- Tenant data isolation at storage layer
- Billing integration with Stripe

### Operations
- Kubernetes deployment with Helm charts
- Observability stack (Prometheus, Grafana, Loki)
- Automated database migrations
- Zero-downtime deployments

### Developer Experience
- TypeScript SDK with full type coverage
- OpenAPI specification and docs site
- Sandbox environment with sample data
- CLI tool for data management

## Constraints

- Each of the 9 sections above (Authentication, Data Ingestion, Data Pipeline, Storage, Query Engine, Visualisation, Multi-Tenancy, Operations, Developer Experience) represents a distinct delivery milestone and **must be treated as a separate phase**. Do not merge sections.
- The stakeholder has confirmed the scope is fixed — no reduction is possible.

Create a complete implementation plan under `.context/plans/` for this project.

## Expected Behavior

1. Before producing files, perform a scope analysis step to count likely phases (explicitly note the scope is very large — 9+ phases required)
2. If analysis suggests more than 8 phases are needed, ask the user a clarifying question before creating the plan (do not silently create 12+ phases)
3. If asking, offer specific options — e.g. "split into two plans by layer" or "consolidate domains" — rather than vaguely asking "how many phases do you want?"
4. If the user is asked a question about phase count, do not proceed to create plan files before receiving an answer
5. If no question is asked and a plan is created directly, the plan must contain 8 or fewer phases
6. If files are created, the root `README.md` is a navigation index without embedded implementation detail
7. Any created phase `README.md` contains a concrete gate with a runnable shell command
8. Any task files use the correct `P{NN}T{NN}` identifier format
9. Any task files include a verification section with a runnable command
10. Use scaffold scripts (`new-plan.sh`, `new-phase.sh`, `new-task.sh`) and `validate-plan.sh` if creating files

## Success Criteria

- **scope-analysis-performed**: Before producing files, the agent performed a scope analysis step to count likely phases or explicitly noted the scope is very large
- **user-asked-before-exceeding-8-phases**: If the agent's analysis suggested more than 8 phases were needed, the agent asked the user a clarifying question before creating the plan
- **question-is-specific-not-generic**: If the agent asked a question, it offered specific options (e.g. "split into two plans by layer" or "consolidate domains") rather than vaguely asking "how many phases do you want?"
- **no-plan-created-without-answer**: If the agent asked the user a question about phase count, it did not proceed to create plan files before receiving an answer
- **plan-within-8-phases-if-user-not-asked**: If the agent did not ask and created a plan directly, the plan must contain 8 or fewer phases
- **root-readme-is-navigation-index**: If files were created, the root `README.md` is a navigation index without embedded implementation detail
- **phase-readmes-have-gates**: Any created phase `README.md` contains a concrete gate with a runnable shell command
- **task-identifiers-correct-format**: Any task files use the correct `P{NN}T{NN}` identifier format
- **task-verification-sections-present**: Any task files include a verification section with a runnable command
- **scripts-used-for-scaffolding**: If files were created, the scaffold scripts were used
- **validate-plan-run-if-files-created**: If files were created, `validate-plan.sh` was run after writing all files
- **slug-format-correct**: All created directories and files use lowercase kebab-case slugs

## Failure Conditions

- Does not perform any scope analysis before creating files
- Silently creates 9+ phases without asking the user
- Asks a question but makes it too vague (e.g. "how many phases?") without offering specific options
- Proceeds to create plan files without waiting for the user's answer to a phase-count question
- Creates a plan with 9+ phases without prior user confirmation
