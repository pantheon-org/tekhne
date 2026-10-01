# Scenario 05: Full-Stack Application CI Pipeline with Parallel Quality Gates

## User Prompt

You are building a CI pipeline for a full-stack application. The team wants independent quality checks (unit tests, integration tests, linting, and security scan) to run concurrently to reduce pipeline duration, followed by a Docker build and push stage that only runs when all quality gates pass.

The pipeline must use Declarative syntax, run the quality checks in parallel, and clean up the workspace after every run.
