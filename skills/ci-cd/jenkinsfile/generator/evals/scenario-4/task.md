# Production Deployment Pipeline with Manual Approval Gate

## Problem/Feature Description

A regulated healthcare company deploys a `patient-records-api` to production. Due to compliance requirements, every production deployment must be manually approved by a member of the `release-managers` group before it proceeds. Deployments are triggered only on the `main` branch.

The team also needs audit-trail evidence that specific build artifacts were deployed — compliance officers require that each production artifact be traceable back to a specific build. The pipeline should archive the deployment package after a successful run.

Previous pipelines had two problems: the manual approval gate held a Jenkins executor for the entire wait period (sometimes hours), wasting agent capacity; and archived artifacts had no fingerprint so the compliance team couldn't verify which artifact went to which environment.

## Output Specification

Produce a `Jenkinsfile` at the repo root. Include stages for: build, test, a conditional production deployment (main branch only) with a manual approval gate, and artifact archiving.
