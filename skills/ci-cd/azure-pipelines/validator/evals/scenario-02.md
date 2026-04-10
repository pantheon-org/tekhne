# Scenario 02: Syntax Validation: Stage/Job/Step Hierarchy and Task Format Errors

## User Prompt

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

## Expected Behavior

1. Identify the `task: Npm` entry (missing @version suffix) as a task format violation
2. Identify the bare `dependsOn:` key with no value in the CD stage as a syntax error
3. Correct the unversioned Npm task to a versioned form (e.g., Npm@1) in the output
4. Fix the empty `dependsOn:` in the CD stage to either reference CI, be removed, or have a valid value
5. Report the location (stage/job/step name) and correction applied for each error

## Success Criteria

- **Unversioned task format detected**: syntax-report.md (or equivalent) flags the `task: Npm` entry (missing @version suffix) as a task format violation
- **Empty dependsOn detected**: syntax-report.md flags the `dependsOn:` key with no value in the CD stage as a syntax error or schema violation
- **Task format corrected in output**: In the corrected azure-pipelines.yml, the unversioned `task: Npm` entry is replaced with a versioned form (e.g., Npm@1)
- **dependsOn corrected in output**: In the corrected azure-pipelines.yml, the CD stage dependsOn is either removed, set to CI, or given a valid value — no bare `dependsOn:` without a value
- **Location/context provided for each error**: syntax-report.md identifies where each error occurs (e.g., stage name, job name, task name, or approximate line)
- **Correction explained**: syntax-report.md explains the correction applied for each error, not just that an error exists

## Failure Conditions

- The unversioned `task: Npm` is not identified as a task format violation
- The empty `dependsOn:` in the CD stage is not flagged as a syntax error
- Corrected pipeline still contains an unversioned `task: Npm` without @version suffix
- Corrected pipeline retains the bare `dependsOn:` key with no value in the CD stage
- Report does not identify where each error occurs (no location context)
- Report lists errors without explaining what correction was applied
