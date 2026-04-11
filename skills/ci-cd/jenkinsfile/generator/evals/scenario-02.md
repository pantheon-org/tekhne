# Scenario 02: Full-Stack Application CI Pipeline with Parallel Quality Gates

## User Prompt

You are building a CI pipeline for a full-stack application. The team wants independent quality checks (unit tests, integration tests, linting, and security scan) to run concurrently to reduce pipeline duration, followed by a Docker build and push stage that only runs when all quality gates pass.

The pipeline must use Declarative syntax, run the quality checks in parallel, and clean up the workspace after every run.

## Expected Behavior

The agent should wrap the concurrent quality checks in a `parallel { }` block inside a single stage. `parallelsAlwaysFailFast()` must appear in the pipeline-level `options` block (not as a per-stage `failFast true`). The post block must include `always { cleanWs() }`. A Docker build/push stage must appear as a separate sequential stage after the parallel block.

Capability tested: Parallel stages & cleanWs post block.

1. Wrap concurrent quality checks in a `parallel { }` block containing at least 2 child stages
2. Add `parallelsAlwaysFailFast()` to the pipeline-level `options` block — not a per-stage `failFast true`
3. Include `post { always { cleanWs() } }` or `always { deleteDir() }` for workspace cleanup
4. The parallel block must contain at least 3 named child stages
5. Place the Docker build/push as a sequential stage after the parallel block
6. Use Declarative pipeline syntax (`pipeline { }`)

## Success Criteria

- **Parallel block used**: Jenkinsfile contains a `parallel { }` block wrapping at least 2 concurrent stages
- **parallelsAlwaysFailFast in options**: Jenkinsfile `options` block includes `parallelsAlwaysFailFast()` — NOT just a per-block `failFast true` inside the parallel stage
- **cleanWs in post always**: Jenkinsfile `post` block contains `always { cleanWs() }` or `always { deleteDir() }`
- **At least 3 parallel stages**: The parallel block contains at least 3 named child stages
- **Sequential stage after parallel**: A build or push stage appears as a separate sequential stage AFTER the parallel block
- **Declarative syntax**: Jenkinsfile uses Declarative pipeline syntax (`pipeline { ... }`), NOT Scripted (`node { ... }`)

## Failure Conditions

- No `parallel { }` block is used (e.g., stages are sequential or use only `failFast true`)
- `parallelsAlwaysFailFast()` is absent from the pipeline-level `options` block
- No `post` block with `cleanWs()` or `deleteDir()` is present
- The parallel block contains fewer than 3 named child stages
- No sequential build or push stage follows the parallel block
- Scripted (`node { }`) syntax is used instead of Declarative (`pipeline { }`)
