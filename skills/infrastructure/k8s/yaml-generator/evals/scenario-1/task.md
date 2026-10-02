# Scenario 1: Generates a production-ready Deployment and Service from a short brief

## User Prompt

"Generate Kubernetes manifests to run our orders API."

## Repository State

The container image is `registry.example.com/shop/orders-api` and version 2.4.1 has been built. It listens on port 8080 and exposes `/health` and `/ready` endpoints. The service should run three replicas and only be reachable from inside the cluster. There are no existing Kubernetes manifests in the repository.
