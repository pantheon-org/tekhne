# Typical Workflow Example

This document provides a concrete example of the end-to-end process for generating a complex Azure Pipeline.

## Example Request

**User request:** "Create a CI/CD pipeline for a Node.js app with Docker deployment to AKS"

## Step-by-Step Process

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

## Workflow Components Explained

### 1. UNDERSTAND

Break down the user's request into technical requirements:

- **Build:** Node.js application needs runtime setup, dependency installation, compilation
- **Test:** Run automated tests and publish results
- **Package:** Create Docker container image
- **Publish:** Push image to Azure Container Registry (ACR)
- **Deploy:** Deploy to Azure Kubernetes Service across multiple environments

### 2. READ

Load relevant documentation and examples:

- Core schema and task references for syntax accuracy
- Best practices for security, performance, naming conventions
- Example files matching the pipeline type (multi-stage, Kubernetes)

### 3. LOOKUP

Use external documentation sources when needed:

- Tasks not covered in local references
- Version-specific behavior questions
- Troubleshooting specific errors

### 4. GENERATE

Create the pipeline following:

- Multi-stage structure with proper dependencies
- Security best practices (service connections, no hardcoded secrets)
- Performance optimizations (caching, parallelism)
- Proper error handling and test result publishing

### 5. VALIDATE

**Critical step:** Run validation before delivery:

```bash
# Invoke validator skill
devops-skills:azure-pipelines-validator
```

Fix any reported issues and re-validate until clean.

### 6. PRESENT

Deliver complete solution with:

- Validated pipeline YAML
- Explanation of each stage's purpose
- Setup instructions (service connections, environment configuration)
- Any additional configuration needed in Azure DevOps
