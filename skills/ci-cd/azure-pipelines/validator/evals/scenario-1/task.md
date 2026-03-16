# Syntax Validation: Stage/Job/Step Hierarchy and Task Format Errors

## Problem/Feature Description

A developer on a distributed team has written their first Azure DevOps pipeline. The pipeline fails to queue in Azure DevOps with a generic "YAML parsing error" message. The developer suspects there are multiple structural issues but cannot identify them from the error message alone.

Validate the pipeline syntax below. Identify every structural and schema violation, explain each error clearly, and produce a corrected version of the file.

## Output Specification

Produce:
- A corrected `azure-pipelines.yml` with all syntax errors fixed
- A `syntax-report.md` listing each error with: the location (stage/job/step name or line reference), the rule violated, and the correction applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/azure-pipelines.yml ===============
trigger:
  - main

stages:
  - stage: CI
    jobs:
      - job: Test
        steps:
          - task: Npm
            inputs:
              command: 'install'
          - task: Npm@1
            inputs:
              command: 'test'
          - script: echo "Tests passed"
      - job: Build
        dependsOn: Test
        steps:
          - task: Docker@2
            inputs:
              command: buildAndPush
              repository: myrepo/myimage
              dockerfile: Dockerfile
              containerRegistry: myServiceConnection

  - stage: CD
    dependsOn:
    jobs:
      - job: Deploy
        steps:
          - task: AzureWebApp@1
            inputs:
              azureSubscription: $(AZURE_SUBSCRIPTION)
              appName: my-app
              package: '$(System.DefaultWorkingDirectory)/**/*.zip'
