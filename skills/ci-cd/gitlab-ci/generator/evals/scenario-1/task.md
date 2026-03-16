# Monorepo Pipeline with Frontend and Backend Components

## Problem/Feature Description

A mid-sized SaaS company has a monorepo containing a React frontend and a Python backend service. Currently their `.gitlab-ci.yml` runs all jobs strictly in stage order, meaning the backend tests cannot start until the frontend build has finished — even though the two components are completely independent. As the codebase has grown, pipelines now take 18 minutes end-to-end on a fast runner. The engineering manager wants to bring this down to under 10 minutes without splitting the repository.

A separate concern: the pipeline has occasionally hung when a flaky integration test stalls an external database fixture, blocking the runner for over an hour. There is no mechanism to cancel the stalled pipeline when a developer pushes a follow-up fix to the same branch.

## Output Specification

Produce a `.gitlab-ci.yml` that builds and tests the frontend and backend components. The pipeline structure should allow independent component tests to start as soon as their own build step finishes, rather than waiting for all build jobs in the stage to complete. Jobs that do not need to run to completion before a developer pushes again should be cancelable by a new pipeline. Long-running jobs should have a safeguard so they cannot run indefinitely.
