# Scenario 05: DAG Optimization: Sequential Pipeline Bottlenecks and needs-based Parallelism

## User Prompt

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

## Expected Behavior

1. Identify that `lint_code` and `lint_types` only depend on the `build` job and can run in parallel with each other immediately after build completes
2. Identify that `sast_scan` and/or `dependency_scan` are independent of the test stage results and can run earlier using `needs:`
3. Add `needs: [build]` to `lint_code` and `lint_types` so they start immediately after build
4. Add `needs:` to at least one of `sast_scan`, `dependency_scan`, or `package` to bypass unnecessary stage gating
5. Document the before/after critical path improvement in the report
6. Explain how `needs:` bypasses stage ordering to enable DAG-based execution
7. Ensure no `needs:` references a job in a later stage

## Success Criteria

- **lint jobs parallelism opportunity identified**: dag-optimization-report.md (or equivalent) identifies that lint_code and lint_types only depend on the build job and can run in parallel with each other immediately after build completes
- **security jobs parallelism opportunity identified**: dag-optimization-report.md identifies that sast_scan and/or dependency_scan are independent of the test stage results and can run in parallel with (or immediately after) the lint/test jobs
- **needs: added to lint jobs in output**: In the optimized .gitlab-ci.yml, lint_code and lint_types have `needs: [build]` (or equivalent) so they start immediately after build
- **needs: added to security or package jobs in output**: In the optimized .gitlab-ci.yml, at least one of sast_scan, dependency_scan, or package has a `needs:` clause that allows it to start before later stages would normally allow
- **Critical path improvement quantified**: dag-optimization-report.md includes a before/after comparison of critical path length (e.g., number of sequential blocking stages reduced from 6 to a smaller number)
- **needs: mechanism explained**: dag-optimization-report.md explains how `needs:` bypasses stage ordering and enables DAG-based execution
- **No invalid needs: dependencies introduced**: In the optimized .gitlab-ci.yml, no job has a `needs:` reference to a job in a later stage (which would be invalid)

## Failure Conditions

- `lint_code` and `lint_types` parallelism opportunity is not identified
- `sast_scan` or `dependency_scan` parallelism opportunity is not identified
- Optimized pipeline does not add `needs: [build]` to lint_code and lint_types
- No `needs:` clause is added to sast_scan, dependency_scan, or package jobs
- Report does not include a before/after critical path comparison
- Report does not explain how `needs:` achieves DAG-based parallelism
- Any job in the optimized pipeline has a `needs:` reference to a job in a later stage
