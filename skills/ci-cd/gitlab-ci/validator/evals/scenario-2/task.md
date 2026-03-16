# Best Practices: Deprecated Keywords, Caching, Artifact Expiration, and Image Pinning

## Problem/Feature Description

A platform team is reviewing a `.gitlab-ci.yml` that was written before GitLab 14.x. The pipeline runs correctly today but the team has been warned it will break when GitLab completes the removal of deprecated keywords. Additionally, build times have increased 40% over the past quarter and the team suspects missing caches and inefficient artifact configuration.

Review the pipeline below for best practices violations. For each finding, explain the problem and its impact, and produce a fully improved version of the pipeline.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` with all best practice improvements applied
- A `best-practices-report.md` listing each finding with: category (deprecation/performance/reliability), job name, problem description, and fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - install
  - build
  - test
  - deploy

install_deps:
  stage: install
  image: node:latest
  script:
    - npm ci
  artifacts:
    paths:
      - node_modules/

build_app:
  stage: build
  image: node:latest
  script:
    - npm run build
  artifacts:
    paths:
      - dist/
  only:
    - main
    - merge_requests

test_unit:
  stage: test
  image: node:latest
  script:
    - npm test
  except:
    - tags

test_e2e:
  stage: test
  image: cypress/base:latest
  script:
    - npx cypress run
  artifacts:
    paths:
      - cypress/screenshots/
      - cypress/videos/
  only:
    - main
    - merge_requests

deploy_production:
  stage: deploy
  image: alpine:latest
  script:
    - ./deploy.sh production
  only:
    - main
