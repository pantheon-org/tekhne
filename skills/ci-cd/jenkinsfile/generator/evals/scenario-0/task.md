# CI Pipeline for Python REST API

## Problem/Feature Description

A data engineering team maintains a Python REST API called `data-gateway`. They currently trigger deployments by SSHing into servers and running scripts by hand — there is no automated CI pipeline. The team lead wants a Jenkins pipeline that checks out the code, runs linting and unit tests, builds a Docker image, and pushes it to their private registry.

The registry requires authentication. A previous attempt at a pipeline by a junior developer hardcoded the Docker Hub password directly in a `parameters` block so it would show up in the Jenkins UI during builds — this caused a security incident and the team wants to make sure the new pipeline never does this.

The pipeline should be ready for a production Jenkins instance without any manual configuration beyond registering credentials in the Credentials Store.

## Output Specification

Produce a `Jenkinsfile` at the repo root for the `data-gateway` service. The pipeline should cover at minimum: checkout, lint, test, Docker build, and Docker push stages. Registry credentials should be referenced by a credentials ID (e.g., `registry-creds`).
