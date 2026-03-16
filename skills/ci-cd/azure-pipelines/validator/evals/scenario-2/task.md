# Best Practices Audit: Display Names, Caching, and Timeouts

## Problem/Feature Description

A platform engineering team is reviewing an inherited Azure Pipelines configuration for a Node.js application. The pipeline works but build times have been growing and the team cannot easily track which step is slow from the Azure DevOps UI. They have asked for a best practices review covering: readability, build speed improvements, and reliability hardening.

Review the pipeline below for best practices violations. For each finding, explain the problem, the impact, and provide a concrete corrected snippet. Then produce a fully corrected version of the pipeline.

## Output Specification

Produce:
- A corrected `azure-pipelines.yml` applying all best practice improvements
- A `best-practices-report.md` listing each finding with: category (readability/performance/reliability), description of the problem, and the fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/azure-pipelines.yml ===============
trigger:
  branches:
    include:
      - main
      - develop

pool:
  vmImage: ubuntu-latest

stages:
  - stage: Build
    jobs:
      - job: BuildJob
        steps:
          - task: NodeTool@0
            inputs:
              versionSpec: '18.x'
          - task: Npm@1
            inputs:
              command: 'install'
          - task: Npm@1
            inputs:
              command: 'custom'
              customCommand: 'run build'
          - task: Npm@1
            inputs:
              command: 'custom'
              customCommand: 'run test'
          - task: PublishTestResults@2
            inputs:
              testResultsFormat: 'JUnit'
              testResultsFiles: '**/test-results.xml'

  - stage: Package
    jobs:
      - job: PackageJob
        steps:
          - task: Docker@2
            inputs:
              command: buildAndPush
              repository: myorg/myapp
              dockerfile: Dockerfile
              containerRegistry: myDockerConnection
              tags: |
                $(Build.BuildId)
                latest
