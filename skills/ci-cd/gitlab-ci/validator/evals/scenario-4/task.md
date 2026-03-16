# DAG Optimization: Sequential Pipeline Bottlenecks and needs-based Parallelism

## Problem/Feature Description

A data engineering team's GitLab CI pipeline takes 22 minutes end-to-end. The team lead suspects that independent jobs are running sequentially because of how the stage model was set up, and that switching to DAG-based execution with `needs:` would cut pipeline time significantly.

Analyze the pipeline below for parallelism opportunities. Identify which jobs are genuinely dependent on each other and which can run in parallel. Produce a DAG-optimized version of the pipeline and document the expected improvement.

## Output Specification

Produce:
- An optimized `.gitlab-ci.yml` using `needs:` to enable parallel execution where appropriate
- A `dag-optimization-report.md` that: (1) lists each parallelism opportunity found with the jobs involved, (2) shows the before/after critical path length (in stages), and (3) explains how `needs:` achieves the parallelism

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - build
  - lint
  - test
  - security
  - package
  - deploy

default:
  image: python:3.11-slim

build:
  stage: build
  script:
    - pip install -r requirements.txt
  artifacts:
    paths:
      - .venv/
    expire_in: 1 hour

lint_code:
  stage: lint
  script:
    - pip install flake8
    - flake8 src/

lint_types:
  stage: lint
  script:
    - pip install mypy
    - mypy src/

unit_tests:
  stage: test
  script:
    - pytest tests/unit/
  artifacts:
    reports:
      junit: unit-test-results.xml

integration_tests:
  stage: test
  script:
    - pytest tests/integration/

sast_scan:
  stage: security
  script:
    - pip install bandit
    - bandit -r src/

dependency_scan:
  stage: security
  script:
    - pip install safety
    - safety check

package:
  stage: package
  script:
    - python setup.py bdist_wheel
  artifacts:
    paths:
      - dist/
    expire_in: 1 week

deploy:
  stage: deploy
  script:
    - ./deploy.sh
  rules:
    - if: '$CI_COMMIT_BRANCH == "main"'
