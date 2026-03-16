# Template Validation: Errors in Referenced Template Files

## Problem/Feature Description

A DevOps engineer has set up a modular Azure Pipelines configuration with a main entry point that delegates build and deploy stages to YAML templates. The main file passes validation on its own, but the pipeline fails at queue time with errors in the template files. The engineer realizes they only validated the entry point.

Validate both the main pipeline file and the referenced template below. Identify all errors in the template files (not just the entry point), explain why validating only the entry point is insufficient, and produce corrected versions of all files.

## Output Specification

Produce:
- A corrected `azure-pipelines.yml` (main entry point)
- A corrected `templates/build-stage.yml`
- A `template-validation-report.md` that: (1) lists all errors found across all files, (2) notes which file each error is in, and (3) explains why the entry-point-only validation approach would have missed these issues

## Input Files

The following files are provided as input.

=============== FILE: inputs/azure-pipelines.yml ===============
trigger:
  branches:
    include:
      - main

pool:
  vmImage: ubuntu-latest

stages:
  - template: templates/build-stage.yml
    parameters:
      nodeVersion: '18.x'
      runTests: true

  - stage: Deploy
    dependsOn: Build
    jobs:
      - job: DeployJob
        displayName: 'Deploy to production'
        steps:
          - script: echo "Deploying..."
            displayName: 'Run deploy script'

=============== FILE: inputs/templates/build-stage.yml ===============
parameters:
  - name: nodeVersion
    type: string
  - name: runTests
    type: boolean
    default: false

stages:
  - stage: Build
    jobs:
      - job: BuildJob
        steps:
          - task: NodeTool@0
            inputs:
              versionSpec: ${{ parameters.nodeVersion }}
          - script: npm ci
          - script: npm run build
          - ${{ if eq(parameters.runTests, true) }}:
            - script: npm test
              displayName: 'Run tests'
          - task: PublishBuildArtifacts
            inputs:
              pathToPublish: 'dist'
              artifactName: 'drop'
