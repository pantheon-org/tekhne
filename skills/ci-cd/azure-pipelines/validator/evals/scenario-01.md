# Scenario 01: Security Audit: Credentials and Unsafe Container Images

## User Prompt

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

## Expected Behavior

1. Detect and flag the hardcoded AWS_ACCESS_KEY_ID value as a credential exposure finding
2. Detect and flag the hardcoded AWS_SECRET_ACCESS_KEY value as a credential exposure finding
3. Detect and flag the node:latest container image as an unpinned image reference
4. Detect and flag the `curl -k` flag in the deploy step as an SSL/TLS verification bypass
5. Replace the literal AWS credential values with variable group references or remove them from the variables block
6. Pin the container image to a concrete version tag or SHA digest
7. Remove the `-k` flag from the curl command in the corrected pipeline

## Success Criteria

- **Hardcoded AWS_ACCESS_KEY_ID detected**: security-report.md (or equivalent) flags the AWS_ACCESS_KEY_ID variable value as a hardcoded credential finding
- **Hardcoded AWS_SECRET_ACCESS_KEY detected**: security-report.md flags the AWS_SECRET_ACCESS_KEY variable value as a hardcoded credential finding
- **Container :latest tag detected**: security-report.md flags node:latest as an unpinned container image
- **SSL bypass (curl -k) detected**: security-report.md flags the curl -k flag as a TLS/SSL verification bypass
- **Credentials replaced with variable group or secret references**: In the corrected azure-pipelines.yml, AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY are replaced with $(VAR_NAME) references or removed from the variables block — no literal key values remain
- **Container image pinned**: In the corrected azure-pipelines.yml, the container image reference no longer uses :latest — it specifies a concrete version tag or SHA digest
- **SSL bypass remediated**: In the corrected azure-pipelines.yml, -k is removed from the curl command (or the step is updated to use TLS verification)
- **Severity levels assigned**: security-report.md assigns a severity level (HIGH, MEDIUM, or INFO) to each finding
- **Risk explanation provided**: security-report.md includes a brief explanation of the risk for at least two of the findings (e.g., key exposure, unpinned images, MITM)

## Failure Conditions

- Hardcoded AWS_ACCESS_KEY_ID value is not identified as a security finding
- Hardcoded AWS_SECRET_ACCESS_KEY value is not identified as a security finding
- node:latest container tag is not flagged as an unpinned image
- `curl -k` SSL bypass is not identified as a security finding
- Corrected pipeline still contains literal AWS credential values in the variables block
- Corrected pipeline still uses node:latest for the container image
- Corrected pipeline retains the `-k` flag in the curl command
- No severity levels are assigned to findings in the report
