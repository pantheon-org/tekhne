# Secure Deployment Pipeline with External API Calls

## Problem/Feature Description

A security-conscious SaaS company runs a `billing-service` that needs to be deployed to staging and production. The deployment pipeline must call an external API to register the new version in their internal service registry, and must upload the build report to an artifact server that requires bearer token authentication.

A recent security audit identified two high-severity findings in the existing pipeline code: secrets were being passed through Groovy string interpolation into shell commands (causing them to appear in build logs and the OS process list), and the API token was declared as a pipeline string parameter so it appeared in the Jenkins UI.

The team wants a rewritten Jenkinsfile that passes the security audit. The auditor specifically looks for whether secrets reach the shell via Groovy interpolation or via the environment.

## Output Specification

Produce a `Jenkinsfile` at the repo root for the `billing-service` deployment pipeline. Include at minimum: build, deploy-staging, register-version (API call with bearer token), and upload-report stages.

The pipeline must reference two credentials: `service-registry-token` (for the API call) and `artifact-server-creds` (username/password for artifact upload). Use these credential IDs in your Jenkinsfile.
