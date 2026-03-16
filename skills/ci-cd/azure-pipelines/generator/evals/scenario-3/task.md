# Create a CI/CD Pipeline for a Go Microservice

## Problem/Feature Description

A platform team is standardizing their Go microservice CI pipelines in Azure DevOps. They have a new `auth-service` written in Go 1.22 and need a complete pipeline that:

- Runs static analysis and vets the code
- Executes unit tests with race detection and coverage reporting
- Builds a production binary for Linux/amd64 (to be packaged in a container)
- Publishes test and coverage results to ADO

The team lead has mentioned that previous Go pipelines at the organization had issues with module download times because caching wasn't set up properly, and that the test command didn't catch data races until production.

## Output Specification

Produce `azure-pipelines.yml` with a complete Go CI pipeline.
