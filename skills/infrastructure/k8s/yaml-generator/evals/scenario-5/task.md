# Scenario 5: Routes a request to validate an existing manifest to the validator

## User Prompt

"Can you check whether this deployment.yaml is valid?"

## Repository State

The repository contains `deployment.yaml`, a hand-written Deployment for `inventory-api` in the `shop` namespace. It uses `apps/v1`, three replicas and the image `inventory-api:1.8.0`. It has not been through any validation yet.
