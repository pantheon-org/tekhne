# Scenario 2: Multi-Environment Deployment Pipeline

## User Prompt

A fintech startup runs a `payments-api` service across three environments: development, staging, and production. Each environment uses a different replica count, database connection string, and ingress hostname. The team is setting up a GitHub Actions workflow to deploy to each environment on merge.

Currently, the chart's `values.yaml` contains production database URLs and replica counts — meaning a developer deploying to dev accidentally inherits prod-scale settings. The team wants a clean separation between the chart's built-in defaults and what changes per environment. They also experienced a painful incident last month where a `helm upgrade` left a release in a broken state after a failed deployment, and they want CI to automatically roll back on failure.
