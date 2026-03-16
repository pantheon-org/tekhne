# Syntax Validation: Stage References, Job Definitions, and Dependency Errors

## Problem/Feature Description

A backend team has migrated their CI/CD pipeline from a legacy tool to GitLab CI. The pipeline was assembled quickly and has never successfully run — it fails at pipeline creation with schema errors. The team has asked for a complete syntax analysis before they attempt to run it.

Validate the syntax of the `.gitlab-ci.yml` below. Identify all schema violations, invalid stage references, and dependency graph errors. For each error, explain the violation and the correction, then produce a corrected version of the file.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` with all syntax errors fixed
- A `syntax-report.md` listing each error with: job or key name, violation type, and correction applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - build
  - test
  - deploy

default:
  image: node:18-alpine

build_app:
  stage: build
  script:
    - npm ci
    - npm run build
  artifacts:
    paths:
      - dist/
    expire_in: 1 hour

unit_tests:
  stage: test
  needs:
    - build_app
    - integration_tests
  script:
    - npm test

integration_tests:
  stage: test
  script:
    - npm run test:integration

lint:
  stage: validate
  script:
    - npm run lint

security_scan:
  stage: test
  needs:
    - build_app
  script:
    - npm audit
  allow_failure: true

deploy_staging:
  stage: deploy
  needs:
    - unit_tests
    - lint
  script:
    - echo "Deploying to staging..."
  environment:
    name: staging
