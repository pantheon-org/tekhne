# Full-Stack Application CI Pipeline with Parallel Quality Gates

## Problem/Feature Description

A product team builds a full-stack web application with a React frontend and a Java Spring Boot backend. Each pull request should run four quality checks: frontend linting, frontend unit tests, backend unit tests, and a SAST security scan. Currently these run one after another in a single Jenkins job and take over 20 minutes — the slowdown is causing developers to skip waiting for CI results before merging.

The engineering lead wants the pipeline restructured so that all four quality checks run simultaneously, and the pipeline fails fast if any one of them fails. After the quality gates pass, a Docker image should be built and pushed to a registry. The pipeline must also clean up the workspace at the end of every run to prevent the Jenkins agent disk from filling up.

## Output Specification

Produce a `Jenkinsfile` at the repo root. The pipeline should include the four parallel quality check stages and a sequential Docker build/push stage after they pass.
