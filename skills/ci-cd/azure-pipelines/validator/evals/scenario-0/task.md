# Security Audit: Credentials and Unsafe Container Images

## Problem/Feature Description

A fintech startup's engineering team is preparing to open-source their build pipeline. Before publishing the repository they need a full security audit of `azure-pipelines.yml` to ensure no credentials are exposed and all container references are pinned. A peer reviewer has flagged "a few security concerns" but left no specifics.

Audit the pipeline below for security issues. Identify every security finding, explain why each finding is a risk, and produce a corrected version of the file with all HIGH and MEDIUM issues resolved.

## Output Specification

Produce:
- A corrected `azure-pipelines.yml` with all HIGH and MEDIUM security issues fixed
- A `security-report.md` listing each finding with: severity level, line reference, issue description, and fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/azure-pipelines.yml ===============
trigger:
  branches:
    include:
      - main

resources:
  containers:
    - container: build_env
      image: node:latest

variables:
  AWS_ACCESS_KEY_ID: AKIAIOSFODNN7EXAMPLE
  AWS_SECRET_ACCESS_KEY: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
  AZURE_SUBSCRIPTION_ID: 12345678-1234-1234-1234-123456789abc

pool:
  vmImage: ubuntu-latest

stages:
  - stage: Build
    jobs:
      - job: BuildJob
        container: build_env
        steps:
          - task: NodeTool@0
            inputs:
              versionSpec: '18.x'
          - script: npm ci
          - script: npm run build

  - stage: Deploy
    jobs:
      - job: DeployJob
        steps:
          - script: |
              curl -k https://internal-api.example.com/deploy \
                --data '{"env": "prod"}'
            displayName: 'Trigger deployment'
