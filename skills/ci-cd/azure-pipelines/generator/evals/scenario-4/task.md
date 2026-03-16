# Refactor a Monolithic Pipeline into Templates

## Problem/Feature Description

A growing engineering org has an `azure-pipelines.yml` that has grown to over 400 lines and now covers four services — `api`, `worker`, `scheduler`, and `notifier`. Each service follows the same build, test, and publish pattern but with slightly different parameters (language version, artifact name, test flags). The pipeline has become difficult to maintain: last month, a team member fixed a caching bug in the `api` section but forgot to apply the same fix to the other three services.

The team lead wants the shared build logic extracted into a reusable template that each service can call. This should make future changes to the shared pattern apply to all services at once. Generate the template file(s) and an updated main pipeline that uses them.

## Output Specification

Produce:
- `templates/build-service.yml`: A reusable build template accepting service-specific parameters
- `azure-pipelines.yml`: The refactored main pipeline that uses the template for all four services

The main pipeline should still trigger on pushes to main and develop branches.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: azure-pipelines.yml ===============
trigger:
  branches:
    include:
    - main
    - develop

pool:
  vmImage: ubuntu-latest

stages:
- stage: BuildApi
  jobs:
  - job: Build
    steps:
    - task: NodeTool@0
      inputs:
        versionSpec: '20.x'
    - script: npm ci
    - script: npm run build
    - script: npm test -- --reporter junit --output test-results.xml
    - task: PublishTestResults@2
      inputs:
        testResultsFiles: 'test-results.xml'
    - publish: dist
      artifact: api-drop

- stage: BuildWorker
  jobs:
  - job: Build
    steps:
    - task: NodeTool@0
      inputs:
        versionSpec: '18.x'
    - script: npm ci
    - script: npm run build
    - script: npm test -- --reporter junit --output test-results.xml
    - task: PublishTestResults@2
      inputs:
        testResultsFiles: 'test-results.xml'
    - publish: dist
      artifact: worker-drop

- stage: BuildScheduler
  jobs:
  - job: Build
    steps:
    - task: NodeTool@0
      inputs:
        versionSpec: '20.x'
    - script: npm ci
    - script: npm run build
    - script: npm test -- --reporter junit --output test-results.xml
    - task: PublishTestResults@2
      inputs:
        testResultsFiles: 'test-results.xml'
    - publish: dist
      artifact: scheduler-drop

- stage: BuildNotifier
  jobs:
  - job: Build
    steps:
    - task: NodeTool@0
      inputs:
        versionSpec: '20.x'
    - script: npm ci
    - script: npm run build
    - script: npm test -- --reporter junit --output test-results.xml
    - task: PublishTestResults@2
      inputs:
        testResultsFiles: 'test-results.xml'
    - publish: dist
      artifact: notifier-drop
