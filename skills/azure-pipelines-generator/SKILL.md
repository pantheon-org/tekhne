---
name: azure-pipelines-generator
description: Generates production-ready Azure DevOps Pipelines (azure-pipelines.yml) following current best practices, security standards, and naming conventions. Use when creating or updating ADO YAML pipelines, configuring build triggers, defining multi-stage deployments, setting up template references, creating variable groups, writing release pipelines, or structuring CI/CD workflows for Azure DevOps Services or Azure DevOps Server. Handles build pipelines, YAML pipelines, Docker container builds, Kubernetes/AKS deployments, language-specific pipelines (.NET, Node.js, Python, Go, Java), and reusable step/job/stage templates. All generated configurations are validated using the devops-skills:azure-pipelines-validator skill before delivery.
---

# Azure Pipelines Generator

## Overview

Generate production-ready Azure DevOps Pipeline configurations following current best practices, security standards, and naming conventions. After generating any **complete** pipeline file, always validate it using the `devops-skills:azure-pipelines-validator` skill, fix any reported issues, and re-validate before presenting to the user. Skip validation only for partial snippets, documentation examples, or when the user explicitly requests it.

## Core Capabilities

### 1. Basic CI Pipelines

**Triggers:** "Create an Azure Pipeline for...", "Build a CI pipeline...", "Generate azure-pipelines.yml..."

**Process:**
1. Clarify language, framework, and testing needs
2. Read `references/yaml-schema.md`, `references/best-practices.md`, `references/tasks-reference.md`
3. Read `assets/examples/basic-ci.yml` for patterns
4. Generate following mandatory standards (see **Best Practices** below)

**Example:**
```yaml
trigger:
  branches:
    include:
    - main
    - develop

pool:
  vmImage: 'ubuntu-22.04'

variables:
  buildConfiguration: 'Release'

steps:
- task: NodeTool@0
  displayName: 'Install Node.js'
  inputs:
    versionSpec: '20.x'

- task: Cache@2
  displayName: 'Cache npm packages'
  inputs:
    key: 'npm | "$(Agent.OS)" | package-lock.json'
    path: $(Pipeline.Workspace)/.npm

- script: npm ci --cache $(Pipeline.Workspace)/.npm
  displayName: 'Install dependencies'

- script: npm run build
  displayName: 'Build application'

- script: npm test
  displayName: 'Run tests'

- task: PublishTestResults@2
  condition: succeededOrFailed()
  inputs:
    testResultsFormat: 'JUnit'
    testResultsFiles: '**/test-results.xml'
```

---

### 2. Multi-Stage CI/CD Pipelines

**Triggers:** "Create a full CI/CD pipeline...", "Multi-stage pipeline...", "Deploy to multiple environments..."

**Process:**
1. Identify all stages (Build, Test, Deploy), dependencies, and branch conditions
2. Read `references/yaml-schema.md` and `assets/examples/multi-stage-cicd.yml`
3. Use deployment jobs for environment tracking; publish artifacts between stages

**Example:**
```yaml
stages:
- stage: Build
  displayName: 'Build Stage'
  jobs:
  - job: BuildJob
    displayName: 'Build Application'
    pool:
      vmImage: 'ubuntu-22.04'
    steps:
    - script: npm run build
      displayName: 'Build'
    - publish: $(Build.SourcesDirectory)/dist
      artifact: drop

- stage: Test
  displayName: 'Test Stage'
  dependsOn: Build
  jobs:
  - job: TestJob
    displayName: 'Run Tests'
    steps:
    - script: npm test
      displayName: 'Test'

- stage: DeployProd
  displayName: 'Deploy to Production'
  dependsOn: Test
  condition: and(succeeded(), eq(variables['Build.SourceBranch'], 'refs/heads/main'))
  jobs:
  - deployment: DeployProd
    environment: production
    strategy:
      runOnce:
        deploy:
          steps:
          - script: echo "Deploying"
```

---

### 3. Docker Build Pipelines

**Triggers:** "Build Docker image...", "Push to container registry...", "Create Docker pipeline..."

**Process:**
1. Identify registry (ACR, Docker Hub), image naming, and tagging strategy
2. Read `references/tasks-reference.md` for Docker@2 and `assets/examples/kubernetes-deploy.yml`
3. Use service connection for authentication; tag with `$(Build.BuildId)` as primary

**Example:**
```yaml
variables:
  dockerRegistryServiceConnection: 'myACR'
  imageRepository: 'myapp'
  containerRegistry: 'myregistry.azurecr.io'
  tag: '$(Build.BuildId)'

steps:
- task: Docker@2
  displayName: 'Build and Push'
  inputs:
    command: buildAndPush
    repository: $(imageRepository)
    dockerfile: '$(Build.SourcesDirectory)/Dockerfile'
    containerRegistry: $(dockerRegistryServiceConnection)
    tags: |
      $(tag)
      latest
```

> **Tagging rule:** Push with `$(tag)` AND `latest`; deploy/pull using only the specific `$(tag)` — never `:latest` in production deployments.

---

### 4. Kubernetes Deployment Pipelines

**Triggers:** "Deploy to Kubernetes...", "K8s deployment pipeline...", "Deploy to AKS..."

**Process:**
1. Identify deployment method (kubectl, Helm, manifests) and cluster connection
2. Read `references/tasks-reference.md` and `assets/examples/kubernetes-deploy.yml`
3. Use KubernetesManifest@0 or Kubernetes@1; include namespace management and health checks

**Example:**
```yaml
- task: KubernetesManifest@0
  displayName: 'Deploy to Kubernetes'
  inputs:
    action: 'deploy'
    kubernetesServiceConnection: 'myK8sCluster'
    namespace: 'production'
    manifests: |
      k8s/deployment.yml
      k8s/service.yml
    containers: '$(containerRegistry)/$(imageRepository):$(tag)'
```

---

### 5. Language-Specific Pipelines

**Supported languages:** .NET/C#, Node.js, Python, Java, Go, Docker multi-stage

**Process:**
1. Read `references/tasks-reference.md` for language-specific tasks
2. Read the matching example file before generating:

| Language | Example File |
|----------|-------------|
| Go | `assets/examples/go-cicd.yml` |
| .NET/C# | `assets/examples/dotnet-cicd.yml` |
| Python | `assets/examples/python-cicd.yml` |
| Node.js | `assets/examples/basic-ci.yml` or `multi-stage-cicd.yml` |

3. Include: runtime setup, package manager caching, build, test with reporting, artifact publish

**Go-specific notes:**
- Use `GoTool@0` (only major version available — @0 is correct)
- Cache Go modules at `$(GOPATH)/pkg/mod` using `go.sum` as key
- Run `go vet ./...` before tests; use `-race -coverprofile` flags for test coverage
- Build with `CGO_ENABLED=0` for container images

**Matrix testing pattern:**
```yaml
strategy:
  matrix:
    node18:
      nodeVersion: '18.x'
    node20:
      nodeVersion: '20.x'
    node22:
      nodeVersion: '22.x'
  maxParallel: 3
steps:
- task: NodeTool@0
  inputs:
    versionSpec: $(nodeVersion)
- script: npm test
```

---

### 6. Template-Based Pipelines

**Triggers:** "Create reusable template...", "Build modular pipeline...", "DRY pipeline..."

**Process:**
1. Design template parameters with types and defaults
2. Read `references/templates-guide.md` and `assets/examples/templates/`
3. Use `${{ parameters.name }}` syntax; generate both template and consuming pipeline

**Example:**
```yaml
# templates/build.yml
parameters:
- name: nodeVersion
  type: string
  default: '20.x'

steps:
- task: NodeTool@0
  inputs:
    versionSpec: ${{ parameters.nodeVersion }}
- script: npm ci
- script: npm run build

# Main pipeline
steps:
- template: templates/build.yml
  parameters:
    nodeVersion: '20.x'
```

---

### 7. Task Documentation Lookup

**When local docs are sufficient (most cases):**
- `references/tasks-reference.md` covers .NET, Node.js, Python, Go, Docker, Kubernetes, Azure tasks
- `references/yaml-schema.md` covers complete YAML syntax

**When to use external sources** (tasks not in local docs, version-specific questions, troubleshooting):
- **Context7 MCP (preferred):** `mcp__context7__resolve-library-id` → query "azure-pipelines" → `mcp__context7__get-library-docs`
- **WebSearch (fallback):** `"[TaskName]@[version] Azure Pipelines task documentation"`

Analyze retrieved docs for: task name/version, required vs optional inputs, service connection requirements, and outputs.

```yaml
# Example: task found via documentation lookup
- task: AzureFunctionApp@1
  displayName: 'Deploy Azure Function'
  inputs:
    azureSubscription: 'AzureServiceConnection'   # Required: ARM service connection
    appType: 'functionAppLinux'
    appName: 'myfunctionapp'
    package: '$(Build.ArtifactStagingDirectory)/**/*.zip'
    runtimeStack: 'NODE|20'
```

---

## Best Practices to Enforce

Reference `references/best-practices.md` for comprehensive guidelines.

### Mandatory Standards

1. **Security:** Never hardcode secrets; use service connections; mark variables as secret in ADO
2. **Version pinning:**
   - vmImage: `ubuntu-22.04` not `ubuntu-latest`
   - Tasks: `Docker@2` not `Docker` (pin to major version; @0 is correct for `GoTool@0`, `NodeTool@0`, `KubernetesManifest@0`)
   - Runtimes: `'20.x'` for Node.js, explicit Go versions
3. **Performance:** Use `Cache@2` for all package managers; use `dependsOn` for parallelism; set artifact expiration; shallow clone when full history is unnecessary
4. **Naming conventions:**
   - Stages/Jobs: PascalCase (`BuildAndTest`, `DeployProduction`)
   - `displayName`: Sentence case (`'Build application'`, `'Run tests'`)
   - Variables: camelCase or snake_case (be consistent)
5. **Organization:** Use stages for complex pipelines; deployment jobs for environment tracking; templates for reusable logic; variable groups for environment-specific config
6. **Error handling:** Set `timeoutInMinutes`; use `condition: succeededOrFailed()` for test publishing; `continueOnError` for non-critical steps
7. **Testing:** Always publish test results (`PublishTestResults@2`) and code coverage (`PublishCodeCoverageResults@1`)

---

## Typical Workflow

**User request:** "Create a CI/CD pipeline for a Node.js app with Docker deployment to AKS"

```
1. UNDERSTAND  — Node.js build/test → Docker image → push to ACR → deploy to AKS (staging + prod)
2. READ        — references/yaml-schema.md, tasks-reference.md, best-practices.md
                 assets/examples/multi-stage-cicd.yml, kubernetes-deploy.yml
3. LOOKUP      — Context7 or WebSearch for any tasks not covered locally
4. GENERATE    — Stage 1: NodeTool@0 + npm ci (cached) + build + test + PublishTestResults
                 Stage 2: Docker@2 buildAndPush (tag with BuildId + latest)
                 Stage 3: Deployment job → KubernetesManifest@0 → health check
5. VALIDATE    — Invoke devops-skills:azure-pipelines-validator; fix errors; re-validate
6. PRESENT     — Validated pipeline + stage explanation + setup notes (service connections, environments)
```

---

## Resources

### Documentation (load as needed)

| File | Purpose |
|------|---------|
| `references/yaml-schema.md` | Pipeline structure, triggers, pools, variables, conditions, expressions |
| `references/tasks-reference.md` | Task catalog with inputs, outputs, service connection requirements |
| `references/best-practices.md` | Security, performance, naming, anti-patterns |
| `references/templates-guide.md` | Template types, parameter definitions, expressions, iteration |

### Examples (read before generating matching pipeline type)

| File | When to read |
|------|-------------|
| `assets/examples/basic-ci.yml` | Simple CI, single-stage builds |
| `assets/examples/multi-stage-cicd.yml` | Multi-environment deployments |
| `assets/examples/kubernetes-deploy.yml` | Docker + K8s/AKS deployments |
| `assets/examples/go-cicd.yml` | Go/Golang applications |
| `assets/examples/dotnet-cicd.yml` | .NET/C# applications |
| `assets/examples/python-cicd.yml` | Python applications |
| `assets/examples/template-usage.yml` | Template-consuming pipelines |
| `assets/examples/templates/build-template.yml` | Reusable build templates |
| `assets/examples/templates/deploy-template.yml` | Reusable deployment templates |

---

## Troubleshooting

### If devops-skills:azure-pipelines-validator reports errors

| Error type | Resolution |
|-----------|-----------|
| Syntax errors | Fix YAML indentation or structure |
| Task version errors | Ensure format is `TaskName@version` |
| Pool/vmImage errors | Use specific versions, not `latest` |
| Stage/Job errors | Verify stages → jobs → steps hierarchy |
| Security warnings | Remove hardcoded secrets; avoid `:latest` in deployments |

### If task documentation is not found

1. Try alternative search queries
2. Check [Microsoft Learn task reference](https://learn.microsoft.com/azure/devops/pipelines/tasks/reference/)
3. Check [azure-pipelines-tasks on GitHub](https://github.com/microsoft/azure-pipelines-tasks)
4. Ask the user for specific version requirements
