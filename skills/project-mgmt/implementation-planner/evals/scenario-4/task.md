# Task: Create an implementation plan for a large-scale SaaS platform

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

- Each of the 9 sections above (Authentication, Data Ingestion, Data Pipeline,
  Storage, Query Engine, Visualisation, Multi-Tenancy, Operations, Developer
  Experience) represents a distinct delivery milestone and **must be treated as a
  separate phase**. Do not merge sections.
- The stakeholder has confirmed the scope is fixed — no reduction is possible.

## What to produce

Create a complete implementation plan under `.context/plans/` for this project.
