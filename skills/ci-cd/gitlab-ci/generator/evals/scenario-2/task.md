# Multi-Environment Deployment Pipeline

## Problem/Feature Description

A DevOps team at a fintech company manages three environments: `development`, `staging`, and `production`. They have experienced two incidents in the past year where simultaneous deployments to staging from different branches caused a partially deployed state that took hours to diagnose. Production deployments have no formal approval gate — a developer once accidentally triggered one at 2 AM and it resulted in a 45-minute outage.

The team wants a pipeline that deploys to development automatically on every push to `develop`, requires a manual click to promote to staging, and requires a separate manual approval for production that can only be triggered from `main`. The pipeline must also track which version is live in each environment so the team can see deployment history in the GitLab UI.

## Output Specification

Produce a `.gitlab-ci.yml` for this deployment pipeline. You may use placeholder `script:` blocks (e.g. `echo "deploying"`) for the actual deployment commands. The pipeline structure, conditions, and safety mechanisms are what matter.
