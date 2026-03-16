# Shared Deployment Workflow for Microservices Platform

## Problem/Feature Description

A platform engineering team manages ten microservices that all deploy to the same Kubernetes cluster. Currently each repository copy-pastes a 150-line deployment workflow. When the deployment process changes, engineers must manually update all ten repositories — a process that regularly leads to drift and outages when some repositories are missed.

The team wants to extract the shared deployment logic into a single reusable workflow that can be called from each microservice repository. The reusable workflow should accept the target environment (`staging` or `production`) as a parameter, receive a deploy token as a secret, and expose the deployed URL as an output for downstream jobs in the calling workflow. The calling workflow lives in the same repository (`.github/workflows/deploy.yml`) and should wire up the reusable workflow correctly.

## Output Specification

Produce two YAML files:
1. `.github/workflows/reusable-deploy.yml` — the reusable workflow definition
2. `.github/workflows/deploy.yml` — a caller workflow that invokes the reusable workflow

Both files should demonstrate the correct pattern for shared deployment workflows in GitHub Actions.
