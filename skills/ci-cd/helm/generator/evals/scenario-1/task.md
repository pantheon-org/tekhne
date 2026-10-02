# Scenario 1: New Microservice Helm Chart

## User Prompt

Your team is deploying a new Node.js API service called `user-api` to Kubernetes. The service runs on port 3000, exposes a `/healthz` liveness endpoint and a `/ready` readiness endpoint, and uses the `company/user-api` container image versioned at `v2.1.0`. The ops team wants a complete Helm chart that can be installed directly with `helm install`.

The platform team has complained that previous charts caused node pressure evictions because resource constraints were missing, and that image tags were baked into charts making rollbacks difficult. The new chart must address both of these operational concerns.
