# Scenario 06: Secure Deployment Pipeline with External API Calls

## User Prompt

You are building a deployment pipeline for a service called `billing-service`. The pipeline must perform an authenticated HTTP call to a service registry API after deploying to staging, and upload a deployment report to an artifact server.

Two credentials exist in the Jenkins Credentials Store:
- `service-registry-token` — a secret text bearer token for the registry API
- `artifact-server-creds` — a username/password pair for the artifact server

The pipeline must include stages for: build, deploy-staging, register-version (API call with bearer token), and upload-report. Secrets must never appear in shell command strings via Groovy interpolation.
