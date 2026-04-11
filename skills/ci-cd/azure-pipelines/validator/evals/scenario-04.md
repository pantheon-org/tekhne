# Scenario 04: YAML Lint: Whitespace and Indentation Errors

## User Prompt

A junior developer has been editing an Azure Pipelines file using a text editor that mixes tabs and spaces and sometimes adds trailing spaces. The pipeline appears to work most of the time, but occasionally a step runs as part of the wrong job, and a stage dependency was silently lost last week, causing a production deploy to run before tests finished.

The team lead suspects YAML formatting issues are causing silent structural changes. Perform a YAML lint analysis on the file below, identify every formatting violation, explain the structural impact of each one (i.e., what the parser would actually interpret vs. what was intended), and produce a corrected version of the file.

## Output Specification

Produce:
- A corrected `azure-pipelines.yml` with all YAML formatting issues fixed
- A `lint-report.md` listing each violation with: violation type, location (approximate line or block), what the parser interprets vs. what was intended, and the correction applied

## Input Files

The following file is provided as input. NOTE: Pay careful attention to indentation and whitespace — that is the subject of this validation.

=============== FILE: inputs/azure-pipelines.yml ===============
trigger:
  branches:
    include:
      - main

pool:
  vmImage: ubuntu-latest

stages:
  - stage: Test
    jobs:
      - job: UnitTest
        steps:
          - task: NodeTool@0
            inputs:
              versionSpec: '18.x'
            displayName: 'Use Node.js 18'
          - script: npm ci
            displayName: 'Install dependencies'
          - script: npm test
        displayName: 'Run unit tests'
  - stage: Build
    dependsOn: Test
    jobs:
      - job: BuildApp
        steps:
           - script: npm run build
             displayName: 'Build application'
           - script: echo "Build complete"
             displayName: 'Confirm build'

## Expected Behavior

1. Identify that `displayName: 'Run unit tests'` is indented at step level rather than job level for the UnitTest job
2. Identify the extra leading spaces on the steps list items in the BuildApp job (3 spaces instead of 2 before `-`)
3. Explain the structural impact of the misplaced displayName (parsed as a step attribute rather than a job-level display name)
4. Explain the structural impact of the over-indented steps block (YAML parse error or steps not belonging to the job)
5. Move `displayName: 'Run unit tests'` to the correct job level in the corrected pipeline
6. Fix the BuildApp steps indentation to consistent 2-space alignment

## Success Criteria

- **Misplaced displayName on UnitTest detected**: lint-report.md (or equivalent) identifies that the `displayName: 'Run unit tests'` line appears at step indentation level rather than job level, meaning it is parsed as a step property or causes ambiguity rather than setting the job display name
- **Over-indented steps in BuildApp detected**: lint-report.md identifies that the steps under BuildApp have extra leading spaces (3 spaces before `-` instead of 2), which may cause them to be misinterpreted or fail YAML parsing
- **Structural impact explained for misplaced displayName**: lint-report.md explains what the YAML parser interprets for the misplaced displayName (e.g., it becomes an orphaned key, it is ignored, or it is parsed as a step-level attribute) versus the intended job-level display name
- **Structural impact explained for over-indentation**: lint-report.md explains the impact of the extra indentation on the steps block (e.g., parser error, steps not belonging to the job)
- **displayName moved to correct job level in output**: In the corrected azure-pipelines.yml, `displayName: 'Run unit tests'` appears at the job level (same indentation as `steps:`) for the UnitTest job
- **BuildApp steps indentation corrected in output**: In the corrected azure-pipelines.yml, the steps under BuildApp use consistent 2-space indentation for the `-` list marker (matching the rest of the file)

## Failure Conditions

- The misplaced displayName in the UnitTest job is not identified as a formatting violation
- The over-indented steps in the BuildApp job are not flagged
- No structural impact is explained for the misplaced displayName
- No structural impact is explained for the over-indented steps
- Corrected pipeline leaves displayName at the wrong indentation level in the UnitTest job
- Corrected pipeline retains the extra leading spaces on BuildApp steps
