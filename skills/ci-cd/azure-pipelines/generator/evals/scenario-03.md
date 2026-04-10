# Scenario 03: Build and Deploy a Containerized Service to AKS

## User Prompt

An e-commerce platform team is moving their product catalog service to Kubernetes on Azure (AKS). Currently they build a Docker image locally and push it manually to an Azure Container Registry. They want to automate this with a pipeline that:

1. Builds and pushes the Docker image to ACR
2. Deploys the new image to their AKS staging environment
3. After manual approval, deploys to production AKS

The team has had incidents where a production deployment pulled the wrong image because the tag was ambiguous. They want the pipeline to tag images in a way that makes every deployment traceable back to a specific build.

## Output Specification

Produce `azure-pipelines.yml` with a complete multi-stage pipeline covering the Docker build, staging deployment, and production deployment. The ACR service connection name is `myACR` and the image repository is `catalog-service`. The AKS service connections are `aks-staging` and `aks-production`. Kubernetes manifests are in `k8s/deployment.yml` and `k8s/service.yml`.

## Expected Behavior

1. Tag Docker images with `$(Build.BuildId)` so every deployment is traceable to a specific build
2. Also push a `latest` tag alongside the build-specific tag
3. Use only the specific build tag (never `:latest`) when deploying to Kubernetes environments
4. Use `deployment:` jobs for both staging and production deploy stages with `environment:` specified
5. Use KubernetesManifest@0 for Kubernetes deployments and Docker@2 for the build/push step
6. Add `dependsOn` between stages and use PascalCase stage names

## Success Criteria

- **Build.BuildId tag**: Docker images are tagged with `$(Build.BuildId)` (or a variable holding it, like `$(tag)` where tag = $(Build.BuildId))
- **Also pushes latest tag**: The Docker push step includes both the build-specific tag AND `latest` in the tags list
- **Deploy uses specific tag only**: The Kubernetes deployment task references the specific build tag (e.g., $(tag)) and NOT `:latest` as the image tag
- **Deployment jobs used**: Both staging and production deploy stages use `deployment:` jobs (not regular `job:`) with `environment:` specified
- **KubernetesManifest@0 task**: The pipeline uses `KubernetesManifest@0` (pinned at @0) for Kubernetes deployments
- **Docker@2 task used**: The Docker build/push step uses `Docker@2` (pinned at @2)
- **Pinned vmImage**: Pool vmImage uses a specific version (e.g., ubuntu-22.04) not ubuntu-latest
- **displayName on all tasks**: Every task and script step has a displayName property
- **PascalCase stage names**: All stage names use PascalCase
- **dependsOn between stages**: Deploy stages have `dependsOn` referencing prior stages
- **No @latest task usage**: No task in the file uses @latest version

## Failure Conditions

- Docker images are not tagged with Build.BuildId (or equivalent build-specific identifier)
- The `latest` tag is not pushed alongside the build-specific tag
- Kubernetes deployment task uses `:latest` as the image tag instead of the build-specific tag
- Staging or production deploy stages use regular `job:` instead of `deployment:` with `environment:`
- KubernetesManifest or Docker tasks are missing version pins or use incorrect versions
- Any task uses @latest version
- Any step is missing a displayName property
- Deploy stages have no `dependsOn` referencing prior stages
