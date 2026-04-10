# Scenario 02: Syntax Validation: Stage References, Job Definitions, and Dependency Errors

## User Prompt

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

## Expected Behavior

1. Identify that the `lint` job references stage `validate`, which is not declared in the top-level `stages:` list
2. Identify that `unit_tests` has `needs: integration_tests` but both jobs are in the same stage — this creates an invalid forward dependency since `needs:` between same-stage jobs requires the referenced job to run before this one
3. Correct the `lint` job to reference a declared stage (e.g., `test`) or add `validate` to the `stages:` list
4. Remove or restructure the `needs: integration_tests` entry from `unit_tests` to create a valid dependency
5. Identify the job name for each error and explain the type of violation

## Success Criteria

- **Undeclared stage detected**: syntax-report.md (or equivalent) flags that the `lint` job references stage `validate`, which is not declared in the top-level `stages:` list
- **Invalid needs dependency detected**: syntax-report.md flags that `unit_tests` has `needs: integration_tests` but both jobs are in the same stage — this creates an invalid forward/circular dependency
- **Undeclared stage corrected in output**: In the corrected .gitlab-ci.yml, the `lint` job either references a declared stage (e.g., `test`) or a new stage (e.g., `validate`) has been added to the top-level `stages:` list
- **Invalid needs dependency corrected in output**: In the corrected .gitlab-ci.yml, the `needs: integration_tests` entry is removed from `unit_tests` or the dependency structure is reorganized so it is valid
- **Job name provided for each error**: syntax-report.md identifies the job name where each error occurs (not just a line number)
- **Violation type explained**: syntax-report.md explains the type of violation (e.g., undeclared stage reference, invalid DAG dependency) and not just that an error exists

## Failure Conditions

- The undeclared stage reference in the `lint` job (stage: validate) is not identified
- The invalid `needs: integration_tests` dependency in `unit_tests` is not flagged
- Corrected pipeline leaves `lint` referencing an undeclared stage
- Corrected pipeline retains the invalid `needs: integration_tests` entry in `unit_tests`
- Report does not identify which job contains each error
- Report does not explain the type of violation (only states that errors exist)
