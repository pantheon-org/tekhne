# Scenario 04: Include File Validation: Errors in Locally Included Templates

## User Prompt

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

## Expected Behavior

1. Detect that `run_tests` in the template references stage `testing`, which is not declared in the main file's `stages:` list
2. Correctly attribute this error to `templates/jobs.gitlab-ci.yml`, not to the main `.gitlab-ci.yml`
3. Explain why validating only the main file does not surface errors in included template files
4. Produce a corrected `templates/jobs.gitlab-ci.yml` where `run_tests` references a declared stage (e.g., `test`) OR the main `.gitlab-ci.yml` adds `testing` to the stages list
5. Ensure the corrected files introduce no new errors (e.g., `deploy_app`'s `needs:` references remain valid)

## Success Criteria

- **Undeclared stage in template detected**: include-validation-report.md (or equivalent) flags that run_tests in the template references stage `testing`, which is not declared in the main file's `stages:` list
- **Error correctly attributed to template file**: include-validation-report.md identifies the error as being in `templates/jobs.gitlab-ci.yml`, not in the main `.gitlab-ci.yml`
- **Entry-point limitation explained**: include-validation-report.md includes an explanation of why validating only the main file does not surface errors in included template files
- **Stage corrected in output**: In the corrected templates/jobs.gitlab-ci.yml, the `run_tests` job references a declared stage (e.g., `test`) OR the main .gitlab-ci.yml adds `testing` to the stages list
- **Corrected template file produced**: A corrected version of `templates/jobs.gitlab-ci.yml` is produced as a file (not just described in the report)
- **No new errors introduced in output**: The corrected files do not introduce new syntax or schema violations (e.g., the needs: references in deploy_app remain valid after stage corrections)

## Failure Conditions

- The undeclared `testing` stage in the template is not identified as an error
- The error is attributed to the main `.gitlab-ci.yml` instead of the template file
- The report does not explain why entry-point-only validation fails to catch template errors
- Corrected template still uses `stage: testing` and `testing` is not added to the main stages list
- No corrected template file is produced (only a description of the error in the report)
- Corrected files introduce new errors or break the `deploy_app` needs: references
