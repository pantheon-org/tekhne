# Scenario 05: Template Validation: Errors in Referenced Template Files

## User Prompt

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

## Expected Behavior

1. Detect the unversioned `task: PublishBuildArtifacts` (missing @N suffix) in the template and flag it
2. Detect the conditional insertion indentation error in the `${{ if ... }}:` block in the template
3. Correctly attribute both errors to `templates/build-stage.yml`, not to `azure-pipelines.yml`
4. Explain why validating only the entry point (`azure-pipelines.yml`) would not surface errors in the template
5. Produce corrected versions of both files with the issues fixed

## Success Criteria

- **Unversioned PublishBuildArtifacts task detected**: template-validation-report.md (or equivalent) flags `task: PublishBuildArtifacts` in the template as lacking a version suffix (should be PublishBuildArtifacts@2 or similar)
- **Conditional insertion indentation error detected**: template-validation-report.md identifies the `${{ if ... }}:` block — the list items under it must be indented one level deeper than the `- ${{ if ... }}:` line; a misalignment here causes the conditional step to be skipped or the YAML to parse incorrectly
- **Error file attribution correct**: template-validation-report.md correctly attributes each error to templates/build-stage.yml (not to azure-pipelines.yml)
- **Entry-point-only limitation explained**: template-validation-report.md includes an explanation of why validating only azure-pipelines.yml would not surface these template errors
- **Task version corrected in output template**: In the corrected templates/build-stage.yml, PublishBuildArtifacts is replaced with a versioned form (e.g., PublishBuildArtifacts@2)
- **Corrected template file produced**: A corrected version of templates/build-stage.yml is produced (not just a report), with the identified issues fixed

## Failure Conditions

- The unversioned `task: PublishBuildArtifacts` is not identified as a template error
- The conditional insertion indentation issue in the `${{ if ... }}:` block is not flagged
- Errors are attributed to azure-pipelines.yml instead of the template file
- The report does not explain why entry-point-only validation would miss these errors
- Corrected template still uses `task: PublishBuildArtifacts` without a version suffix
- No corrected template file is produced (only a report with descriptions)
