# Scenario 05: Refactor a Monolithic Pipeline into Templates

## User Prompt

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

## Expected Behavior

1. Create a `templates/build-service.yml` file that encapsulates the shared build, test, and publish logic
2. Define template parameters using `${{ parameters.name }}` syntax for service-specific values (Node version, artifact name)
3. Reference the template from the main pipeline using `- template:` syntax for all four services
4. Fix the vmImage from ubuntu-latest to a pinned version (e.g., ubuntu-22.04)
5. Add a Cache@2 task in the template for npm packages
6. Add displayName to all steps and condition: succeededOrFailed() to PublishTestResults@2

## Success Criteria

- **Template file created**: A file `templates/build-service.yml` (or similar path) exists and contains reusable step or job definitions
- **Parameter syntax correct**: The template uses `${{ parameters.name }}` syntax for parameter references (not $(parameters.name) or other forms)
- **Template referenced from main**: The main `azure-pipelines.yml` references the template using `- template:` syntax for all four services
- **Pinned vmImage**: The vmImage is changed from `ubuntu-latest` to a specific version (e.g., ubuntu-22.04)
- **Cache@2 in template**: The build template includes a Cache@2 task for npm packages
- **displayName on all steps**: Every task and script step in both the template and main pipeline has a displayName property
- **PublishTestResults with succeededOrFailed**: The PublishTestResults@2 task in the template has `condition: succeededOrFailed()`
- **No @latest task usage**: No task in any generated file uses @latest version
- **Parameters for nodeVersion**: The template defines a parameter for the Node.js version (so each service can specify 18.x or 20.x)
- **Parameters for artifactName**: The template defines a parameter for the artifact name (so api-drop, worker-drop, etc. are set per-service)
- **Explicit trigger preserved**: The main pipeline retains the explicit trigger with main and develop branch includes

## Failure Conditions

- No template file is created; build logic remains duplicated across four stage definitions
- Template uses $(parameters.name) or other non-standard parameter syntax instead of `${{ parameters.name }}`
- Main pipeline does not use `- template:` syntax for all four services
- vmImage remains ubuntu-latest in the refactored pipeline
- No Cache@2 task is added to the template
- Any step is missing a displayName property
- PublishTestResults@2 lacks condition: succeededOrFailed()
- Any task uses @latest version
- Template has no parameter for Node.js version or artifact name, forcing hardcoding in the template body
- Main pipeline trigger is removed or changed to `trigger: none`
