# Multi-Environment Deployment Pipeline

## Problem/Feature Description

A fintech startup runs a `payments-api` service across three environments: development, staging, and production. Each environment uses a different replica count, database connection string, and ingress hostname. The team is setting up a GitHub Actions workflow to deploy to each environment on merge.

Currently, the chart's `values.yaml` contains production database URLs and replica counts — meaning a developer deploying to dev accidentally inherits prod-scale settings. The team wants a clean separation between the chart's built-in defaults and what changes per environment. They also experienced a painful incident last month where a `helm upgrade` left a release in a broken state after a failed deployment, and they want CI to automatically roll back on failure.

## Output Specification

Produce:
1. A minimal Helm chart directory named `payments-api/` with `Chart.yaml`, a clean `values.yaml` (defaults only), and at least one template file.
2. An environment override file `values-production.yaml` at the repo root demonstrating at least 2 production-specific overrides (replica count and an ingress hostname are good examples).
3. A shell script `deploy.sh` that performs the Helm deployment in a way that handles failures gracefully and can be called as: `bash deploy.sh <environment> <release-name> <chart-path>`.

The `values.yaml` inside the chart must not contain any environment-specific values.
