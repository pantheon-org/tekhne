# New Microservice Helm Chart

## Problem/Feature Description

Your team is deploying a new Node.js API service called `user-api` to Kubernetes. The service runs on port 3000, exposes a `/healthz` liveness endpoint and a `/ready` readiness endpoint, and uses the `company/user-api` container image versioned at `v2.1.0`. The ops team wants a complete Helm chart that can be installed directly with `helm install`.

The platform team has complained that previous charts caused node pressure evictions because resource constraints were missing, and that image tags were baked into charts making rollbacks difficult. The new chart must address both of these operational concerns.

## Output Specification

Produce a complete Helm chart directory named `user-api/` containing at minimum:
- `Chart.yaml`
- `values.yaml`
- `templates/deployment.yaml`
- `templates/service.yaml`

The chart should be self-contained and ready to install. Document your intended CPU/memory defaults inside `values.yaml`.
