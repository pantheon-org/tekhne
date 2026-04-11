# Scenario 02: Add Deployment Stage with External Service Credentials

## User Prompt

A payments platform team is migrating their manual deployment process into Azure DevOps Pipelines. Their application needs to connect to a third-party payment gateway API during build-time integration tests and also during deployment. The API credentials consist of a client ID and a client secret. Additionally, the deployment stage needs to push a Docker image to an Azure Container Registry and deploy it to an App Service.

The team has asked you to generate a pipeline that covers build, integration test, and deploy to production. They have raised a concern that previous pipelines at their company stored connection strings directly in the YAML file, which caused a security incident. They want to make sure this pipeline handles secrets properly.

## Output Specification

Produce `azure-pipelines.yml` with a complete multi-stage pipeline. Wherever credentials or secrets are needed, include placeholder comments or references that show the correct pattern — the actual values should not appear in the file.

## Expected Behavior

1. Reference credentials via variable groups or Azure Key Vault links rather than hardcoding them in the YAML
2. Use a service connection reference for the Docker registry rather than inline credentials
3. Structure the production deploy stage as a `deployment:` job with an environment specified
4. Use PascalCase for all stage and job names
5. Add displayName to every task and script step
6. Pin the vmImage to a specific version and pin all task versions with @N syntax

## Success Criteria

- **No hardcoded secrets**: The pipeline YAML does NOT contain any literal secret values — no passwords, API keys, or connection strings as plain text in variables or task inputs
- **Variable group or Key Vault reference**: The pipeline references a variable group or Azure Key Vault task/link for the payment gateway credentials, rather than inline YAML variables
- **Service connection for registry**: The Docker registry connection uses a service connection reference (not hardcoded registry credentials)
- **Deployment job in deploy stage**: The production deploy stage uses a `deployment:` job (not a regular `job:`) with an environment specified
- **PascalCase stage names**: All stage names use PascalCase (e.g., Build, IntegrationTest, DeployProduction) — NOT snake_case, kebab-case, or ALL_CAPS
- **PascalCase job names**: All job names use PascalCase
- **displayName on all tasks**: Every task and script step has a displayName property
- **Pinned vmImage**: Pool vmImage uses a specific version (e.g., ubuntu-22.04) not ubuntu-latest
- **Task versions pinned**: All tasks include explicit version pins (@N) and none use @latest
- **Explicit trigger**: The pipeline has an explicit trigger with branch includes, not `trigger: none`
- **dependsOn between stages**: Deploy stage has a `dependsOn` referencing the prior stage

## Failure Conditions

- Any literal secret, password, API key, or connection string appears as a plain text value in the YAML
- No variable group or Key Vault reference is used for the payment gateway credentials
- Docker registry credentials are hardcoded instead of using a service connection
- The production deploy stage uses a regular `job:` instead of `deployment:` with an environment
- Stage or job names use snake_case, kebab-case, or ALL_CAPS instead of PascalCase
- Any step is missing a displayName property
- vmImage uses ubuntu-latest or any task uses @latest
- Deploy stage has no dependsOn referencing a prior stage
