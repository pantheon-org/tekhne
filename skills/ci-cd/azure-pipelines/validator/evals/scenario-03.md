# Scenario 03: Best Practices Audit: Display Names, Caching, and Timeouts

## User Prompt

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

## Expected Behavior

1. Identify missing displayName properties on both BuildJob and PackageJob
2. Identify missing displayName properties on task steps within both jobs
3. Flag the absence of a Cache@2 task before the npm install step as a performance issue
4. Flag the missing timeoutInMinutes on jobs or stages as a reliability concern
5. Flag the `latest` tag in the Docker tags list as an unpinned tag best practices concern
6. Add displayNames to all jobs and steps in the corrected pipeline
7. Add a Cache@2 task before the npm install step in the corrected pipeline
8. Assign a category (readability, performance, or reliability) to each finding in the report

## Success Criteria

- **Missing displayName on jobs detected**: best-practices-report.md (or equivalent) flags that BuildJob and/or PackageJob lack a displayName property
- **Missing displayName on steps detected**: best-practices-report.md flags that one or more task steps lack a displayName property
- **Missing cache for npm dependencies detected**: best-practices-report.md flags the absence of a Cache@2 task before the npm install step
- **Missing timeout detected**: best-practices-report.md flags that no timeoutInMinutes is set on jobs or stages
- **latest Docker tag flagged**: best-practices-report.md flags the `latest` entry in the Docker tags list as a best practices concern (unpinned image tag)
- **displayNames added in corrected file**: In the corrected azure-pipelines.yml, at least one job and the majority of task steps have a displayName property added
- **Cache task added in corrected file**: In the corrected azure-pipelines.yml, a Cache@2 (or equivalent) task is present before the npm install step, with a key based on package-lock.json or similar
- **Category assigned to each finding**: best-practices-report.md assigns a category (readability, performance, or reliability) to each finding
- **Fix described for each finding**: best-practices-report.md includes a description of the fix applied (not just the problem) for each finding

## Failure Conditions

- Missing displayName on jobs is not identified as a finding
- Missing displayName on task steps is not identified as a finding
- Absence of Cache@2 for npm is not flagged as a performance concern
- Missing timeoutInMinutes is not flagged as a reliability concern
- The `latest` Docker tag is not flagged as a best practices violation
- Corrected pipeline does not add displayName to jobs or steps
- Corrected pipeline does not include a Cache@2 task before npm install
- Report does not assign a category (readability/performance/reliability) to findings
