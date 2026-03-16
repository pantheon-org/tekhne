# Security Audit: Hardcoded Secrets and Insecure Script Patterns

## Problem/Feature Description

A SaaS company's security team has flagged a GitLab CI/CD pipeline during a routine audit. The pipeline was written quickly for a hackathon and then promoted directly to production. The security team suspects hardcoded credentials and unsafe script patterns but has not provided specifics.

Audit the `.gitlab-ci.yml` below for security issues. Identify every security finding, assign a severity level, explain the risk each finding introduces, and produce a corrected version of the file with all critical and high-severity issues resolved.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` with all critical and high-severity security issues fixed
- A `security-report.md` listing each finding with: severity (CRITICAL/HIGH/MEDIUM/INFO), job name, description of the issue, and the fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - build
  - test
  - deploy

variables:
  DOCKER_REGISTRY: registry.example.com
  DEPLOY_TOKEN: glpat-xxxxxxxxxxxxxxxxxxxx
  DB_PASSWORD: SuperSecret123!

build:
  stage: build
  image: docker:latest
  script:
    - docker build -t $DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA .
    - docker push $DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA

test:
  stage: test
  image: node:18
  script:
    - npm ci
    - npm test
    - curl -sSL https://install.example.com/agent.sh | bash

deploy:
  stage: deploy
  image: alpine:3.18
  script:
    - apk add --no-cache curl
    - |
      curl -k https://deploy.internal.example.com/api/deploy \
        --header "Authorization: Bearer $DEPLOY_TOKEN" \
        --data '{"image": "$DOCKER_REGISTRY/myapp:$CI_COMMIT_SHA"}'
  only:
    - main
