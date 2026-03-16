# Include File Validation: Errors in Locally Included Templates

## Problem/Feature Description

A platform team has split a large `.gitlab-ci.yml` into modular include files. The main file passes the GitLab CI Lint tool with no issues, but pipelines are failing at queue time with cryptic error messages. A senior engineer suspects the problem is in the included templates, which were never validated independently.

Validate both the main file and the included template below. Identify all errors in any file, explain why entry-point-only validation does not surface these issues, and produce corrected versions of all files.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` (main file)
- A corrected `templates/jobs.gitlab-ci.yml`
- An `include-validation-report.md` that: (1) lists all errors found with the file each error is in, and (2) includes a section explaining why validating only the main file is insufficient

## Input Files

The following files are provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
include:
  - local: 'templates/jobs.gitlab-ci.yml'

stages:
  - build
  - test
  - deploy

variables:
  REGISTRY: registry.gitlab.example.com
  APP_NAME: myapp

=============== FILE: inputs/templates/jobs.gitlab-ci.yml ===============
build_image:
  stage: build
  image: docker:24.0
  services:
    - docker:24.0-dind
  script:
    - docker build -t $REGISTRY/$APP_NAME:$CI_COMMIT_SHA .
    - docker push $REGISTRY/$APP_NAME:$CI_COMMIT_SHA
  rules:
    - if: '$CI_COMMIT_BRANCH == "main"'

run_tests:
  stage: testing
  image: python:3.11-slim
  script:
    - pip install -r requirements.txt
    - pytest tests/
  artifacts:
    reports:
      junit: test-results.xml

deploy_app:
  stage: deploy
  image: bitnami/kubectl:latest
  needs:
    - run_tests
    - build_image
  script:
    - kubectl apply -f k8s/
  rules:
    - if: '$CI_COMMIT_BRANCH == "main"'
