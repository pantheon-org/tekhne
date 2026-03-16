# YAML Lint: Whitespace and Indentation Errors

## Problem/Feature Description

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
