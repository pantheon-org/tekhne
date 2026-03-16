# Modernize a Legacy CI Workflow

## Problem/Feature Description

A company's engineering team has inherited a CI workflow written two years ago. Their security scan tooling has started flagging the workflow as using outdated dependencies, but the team isn't sure what specifically needs updating or why it matters. They need the workflow reviewed, any outdated or deprecated actions identified, and the workflow brought up to current best practices before deploying it to their new repository.

Review the workflow below, identify all actions that are outdated or deprecated, explain the risk each outdated action poses, update them to current versions, and produce a final corrected workflow.

## Output Specification

Produce:
- A corrected `ci.yml` with all actions updated
- A `modernization-report.md` with a table showing: each action found, its original version, the updated version used, and why the update matters

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/ci.yml ===============
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm ci
      - run: npm test
      - uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: coverage/

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm run deploy
