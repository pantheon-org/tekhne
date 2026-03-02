---
name: gitlab-ci-generator
description: Creates .gitlab-ci.yml files, configures pipeline stages, defines CI jobs and runners, sets up deployment workflows, and generates reusable GitLab CI/CD templates following current best practices and security standards. Use when users ask to create or build a GitLab CI/CD pipeline, CI config, build pipeline, deploy pipeline, GitLab YAML, CI jobs, or any .gitlab-ci.yml configuration from scratch or for a new project.
---

# GitLab CI/CD Pipeline Generator

## Overview

Generate production-ready GitLab CI/CD pipeline configurations following current best practices, security standards, and naming conventions. All generated resources are validated using the `devops-skills:gitlab-ci-validator` skill before delivery.

---

## MANDATORY PRE-GENERATION STEPS

**CRITICAL:** Before generating ANY pipeline, complete these steps in order:

### Step 1: Load Reference Files (ALL REQUIRED)

Use the **Read tool** to load all four reference files plus the relevant template:

```
1. references/best-practices.md       — Security, performance, and naming patterns
2. references/common-patterns.md      — Standard pipeline patterns as foundation
3. references/gitlab-ci-reference.md  — Syntax reference and keyword details
4. references/security-guidelines.md  — Security-sensitive configurations
```

**Template selection:**
- Docker pipelines → `assets/templates/docker-build.yml`
- Kubernetes deployments → `assets/templates/kubernetes-deploy.yml`
- Multi-project pipelines → `assets/templates/multi-project.yml`
- Basic pipelines → `assets/templates/basic-pipeline.yml`

### Step 2: Output Confirmation Before Generating

After reading references, output this confirmation before proceeding:

```
## Reference Analysis Complete

**Pipeline Pattern Identified:** [Pattern name] from common-patterns.md
- [Why this pattern fits]

**Best Practices to Apply:**
- [3–5 key best practices relevant to this pipeline]

**Security Guidelines:**
- [Security measures to implement]

**Template Foundation:** [Template file name]
- [What will be customized]
```

---

## Core Capabilities

### 1. Basic CI/CD Pipelines

Generate complete `.gitlab-ci.yml` files with proper structure, security best practices, and efficient CI/CD patterns.

**Process:**
1. Understand requirements (stages, jobs, dependencies, artifacts)
2. Use `assets/templates/basic-pipeline.yml` as structural foundation
3. Reference `references/best-practices.md` and `references/common-patterns.md`
4. Apply these principles:
   - Semantic stage and job names (kebab-case)
   - Pin Docker images to specific versions (never `:latest`)
   - Masked variables for secrets; never hardcode credentials
   - Caching for dependencies (npm, pip, maven, etc.)
   - Artifact expiration (`expire_in` always set)
   - `needs` keyword for DAG optimization
   - `rules` instead of deprecated `only`/`except`
   - **Explicit `timeout` on ALL jobs** (10–30 minutes typically)
   - `retry` for flaky operations (network, external APIs)
   - `resource_group` for deployment jobs
5. Validate per the [Validation Workflow](#validation-workflow) section

**Minimal example:**
```yaml
stages: [build, test, deploy]

variables:
  NODE_VERSION: "20"

default:
  image: node:20-alpine
  timeout: 20 minutes
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths: [node_modules/]
  interruptible: true

build-application:
  stage: build
  timeout: 15 minutes
  script: [npm ci, npm run build]
  artifacts:
    paths: [dist/]
    expire_in: 1 hour

test-unit:
  stage: test
  needs: [build-application]
  script: [npm run test:unit]
  artifacts:
    reports:
      junit: junit.xml

deploy-production:
  stage: deploy
  needs: [build-application, test-unit]
  script: [npm run deploy:production]
  environment:
    name: production
    url: https://example.com
  rules:
    - if: $CI_COMMIT_BRANCH == "main"
      when: manual
  resource_group: production
  timeout: 15 minutes
```

### 2. Docker Build Pipelines

Create pipelines for building, scanning, and pushing Docker images to container registries.

**Process:**
1. Use `assets/templates/docker-build.yml` as foundation
2. Implement Docker-in-Docker or Kaniko for builds
3. Configure registry authentication via GitLab CI predefined variables
4. Implement image tagging strategy (`$CI_COMMIT_SHORT_SHA`)
5. Add container security scanning (Trivy or GitLab template)
6. Validate per the [Validation Workflow](#validation-workflow) section

**Minimal example:**
```yaml
variables:
  IMAGE_NAME: $CI_REGISTRY_IMAGE
  IMAGE_TAG: $CI_COMMIT_SHORT_SHA

docker-build:
  stage: build
  image: docker:24-dind
  timeout: 20 minutes
  services: [docker:24-dind]
  before_script:
    - docker login -u $CI_REGISTRY_USER -p $CI_REGISTRY_PASSWORD $CI_REGISTRY
  script:
    - docker build --cache-from $IMAGE_NAME:latest --tag $IMAGE_NAME:$IMAGE_TAG .
    - docker push $IMAGE_NAME:$IMAGE_TAG
  retry:
    max: 2
    when: [runner_system_failure]
```

### 3. Kubernetes Deployment Pipelines

Create pipelines deploying to Kubernetes clusters via kubectl, Helm, or Kustomize.

**Process:**
1. Identify deployment method (kubectl, Helm, Kustomize)
2. Use `assets/templates/kubernetes-deploy.yml` as foundation
3. Configure cluster authentication via `$KUBE_CONTEXT`
4. Implement environment management and rollback capabilities
5. Validate per the [Validation Workflow](#validation-workflow) section

**Minimal example:**
```yaml
deploy-k8s:
  stage: deploy
  image: bitnami/kubectl:1.29
  timeout: 10 minutes
  before_script: [kubectl config use-context $KUBE_CONTEXT]
  script:
    - kubectl set image deployment/myapp myapp=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA -n $KUBE_NAMESPACE
    - kubectl rollout status deployment/myapp -n $KUBE_NAMESPACE --timeout=5m
  environment:
    name: production
    url: https://example.com
  rules:
    - if: $CI_COMMIT_BRANCH == "main"
      when: manual
  resource_group: k8s-production
```

### 4. Multi-Project Pipelines

Create pipelines that trigger other projects or use parent-child patterns for monorepos and microservices.

**Process:**
1. Use `assets/templates/multi-project.yml` or parent-child templates
2. Configure artifact passing between pipelines
3. Implement parallel execution where appropriate
4. Validate per the [Validation Workflow](#validation-workflow) section

### 5. Reusable Template Configurations

Create modular, DRY configurations using `extends`, YAML anchors, and `include`.

**Process:**
1. Extract common patterns into hidden jobs (`.template-name`)
2. Use `extends` for inheritance (preferred over YAML anchors in GitLab CI)
3. Organize into separate files with `include`
4. Include explicit `timeout` in all templates
5. Validate per the [Validation Workflow](#validation-workflow) section

**Minimal example:**
```yaml
.node-template:
  image: node:20-alpine
  timeout: 15 minutes
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths: [node_modules/]
  before_script: [npm ci]
  interruptible: true

build:
  extends: .node-template
  stage: build
  script: [npm run build]
```

### 6. GitLab Feature Documentation Lookup

When the user requests specific GitLab features (Auto DevOps, SAST, dependency scanning, etc.):

1. **Search for current docs:** `"GitLab CI/CD [feature] documentation 2025"`
2. **If Context7 MCP is available:** use `mcp__context7__resolve-library-id` then `mcp__context7__get-library-docs`
3. **If specific pages needed:** use WebFetch on `docs.gitlab.com`
4. **Generate pipeline** using discovered syntax, required variables, and template include paths

**When using GitLab include templates:**
```yaml
include:
  - template: Jobs/SAST.gitlab-ci.yml
  - template: Jobs/Dependency-Scanning.gitlab-ci.yml

variables:
  SAST_EXCLUDED_PATHS: "spec, test, tests, tmp"
```

> **Note:** Customize included template jobs via global `variables` rather than partial job overrides. GitLab merges at runtime, but local validators only see your file—partial overrides will fail validation.

---

## Validation Workflow

**Every complete pipeline MUST be validated before presenting to the user.**

### Process

1. After generating, invoke `devops-skills:gitlab-ci-validator`
2. Act on results by severity:

| Severity | Action |
|----------|--------|
| **CRITICAL** | Must fix before presenting |
| **HIGH** | Must fix before presenting |
| **MEDIUM** | Fix or explain why acceptable |
| **LOW** | Acknowledge in output |
| **SUGGESTIONS** | Review and apply if beneficial |

3. Fix CRITICAL/HIGH issues and re-validate until clear
4. Skip validation only for partial snippets, documentation examples, or when user explicitly requests it

### Presentation Requirements

When presenting the final pipeline, include:

1. **Validation status** — pass/fail with issue counts
2. **MEDIUM issues table** (if any) — Issue | Status (Fixed/Acceptable) | Explanation
3. **Suggestions review table** (if any) — Suggestion | Apply/Skip | Reason
4. **Usage instructions** — Required CI/CD variables, setup steps, pipeline behavior per branch/tag

**MEDIUM issue example:**

| Issue | Status | Explanation |
|-------|--------|-------------|
| `image-variable-no-digest` | Acceptable | Using `python:${PYTHON_VERSION}-alpine` allows flexible version management; `PYTHON_VERSION` is internally pinned to "3.12". |
| `git-strategy-none` | Acceptable | `stop-staging` only runs kubectl commands requiring no source code. |

---

## Best Practices

See `references/best-practices.md` for the full set of security, performance, reliability, naming, and organization guidelines that all generated pipelines must follow.

---

## Resources

### Reference Files
- `references/best-practices.md` — Security, performance, pipeline design, anti-patterns
- `references/common-patterns.md` — Standard patterns (basic CI, Docker, K8s, multi-project)
- `references/gitlab-ci-reference.md` — Full keyword and syntax reference
- `references/security-guidelines.md` — Secrets, image, script, and artifact security

### Template Files
- `assets/templates/basic-pipeline.yml` — Basic pipeline template
- `assets/templates/docker-build.yml` — Docker build pipeline template
- `assets/templates/kubernetes-deploy.yml` — Kubernetes deployment template
- `assets/templates/multi-project.yml` — Multi-project orchestration template

**Template usage:** Copy structure → replace `[PLACEHOLDERS]` → customize logic → remove unused sections → validate.
