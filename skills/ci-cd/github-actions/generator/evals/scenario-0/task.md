# Node.js CI Pipeline

## Problem/Feature Description

A small software consultancy maintains a Node.js 20 API service on GitHub. The team has recently had a security incident on a different project where a compromised third-party GitHub Action injected malicious code into their pipeline. Leadership now mandates that all new CI pipelines meet a higher security bar before being merged to the main branch.

The team wants a standard CI workflow for the API repository that runs on every push to `main` and on every pull request. The workflow should install dependencies, run the test suite, and produce a build artifact from the `dist/` directory. The project uses npm and a `package-lock.json` file is present.

## Output Specification

Produce a complete GitHub Actions workflow YAML file saved at `.github/workflows/ci.yml`. The workflow should be production-ready and follow current security and performance best practices.
