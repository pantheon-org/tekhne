# LogQL Query for a New Team Member

## Problem Description

A junior platform engineer just joined a team that uses Grafana Loki for log management. They have been asked to build a query to investigate a recurring issue: sporadic database connection failures in the `auth-service`. The service runs in Kubernetes and logs in JSON format. Relevant log fields include `level`, `component` (value: `"db"` for database logs), `error_type`, and `message`. Stream labels available are `app="auth-service"`, `namespace="backend"`, and `env="staging"`.

The engineer is not yet confident with LogQL and wants to understand how the query is constructed, not just receive a final answer. They have asked for help understanding what each part of the query does before seeing the full version, and want to be able to take away a reference they can consult later.

The final goal is a log-filter query that returns only error-level lines from the database component of the auth-service, formatted to show the `error_type` and `message` fields prominently.

## Output Specification

Produce a file named `learning_session.md` that:

1. Shows the query being built one stage at a time — each step should be on its own, runnable, and there should be at least 3 intermediate steps before the final query.
2. Explains what each step adds and why it is done in that order.
3. Shows the final complete query.
4. Includes a section explaining how to run the final query (at least two different methods, e.g. Grafana UI and a CLI tool).
5. Includes a note identifying which labels in the stream selector or filters the engineer would need to change when moving to production.
