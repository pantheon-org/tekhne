# Scenario 01: Create a Node.js CI Pipeline for a New Service

## User Prompt

A startup's backend team has just created a new Node.js microservice and needs a CI pipeline set up in Azure DevOps. The service uses npm for package management and Jest for testing. The team wants the pipeline to install dependencies, build the service, run tests, and publish test results so they show up in the ADO build summary. The pipeline should run on every push to the main and develop branches.

The DevOps engineer who normally handles this is on leave, so you've been asked to generate the pipeline YAML. The team has had problems in the past with pipelines that suddenly broke when agent software updated, so reliability is a priority.

## Output Specification

Produce a file called `azure-pipelines.yml` with a complete, working CI pipeline configuration.

## Expected Behavior

1. Pin the pool vmImage to a specific version (e.g., ubuntu-22.04) rather than ubuntu-latest
2. Pin all tasks using the @version syntax (e.g., NodeTool@0, PublishTestResults@2) without using @latest
3. Add a displayName property to every task and script step using Sentence case
4. Configure a Cache@2 task for npm packages to speed up repeated runs
5. Include a PublishTestResults@2 task with condition: succeededOrFailed()
6. Specify explicit trigger branches (main and develop) and use npm ci for dependency installation

## Success Criteria

- **Pinned vmImage**: The pool vmImage is set to a specific version (e.g., ubuntu-22.04) and NOT ubuntu-latest or windows-latest
- **Task version pinned**: All tasks use the @version syntax (e.g., NodeTool@0, PublishTestResults@2) and NOT @latest or bare task names without version
- **displayName on all tasks**: Every task and script step has a displayName property (no step without displayName)
- **Sentence case displayName**: All displayName values use Sentence case (first word capitalized, rest lowercase, e.g., 'Install dependencies', not 'INSTALL DEPENDENCIES' or 'Install Dependencies')
- **Cache@2 for npm**: A Cache@2 task is included to cache npm packages
- **PublishTestResults task**: A PublishTestResults@2 task is included with condition: succeededOrFailed()
- **Explicit trigger branches**: The trigger section specifies explicit branch includes (main and develop) and does NOT use `trigger: none`
- **NodeTool version specified**: NodeTool@0 (or equivalent) is used with an explicit versionSpec (e.g., '20.x', '18.x')
- **npm ci used**: The pipeline uses `npm ci` (not `npm install`) for dependency installation
- **No @latest usage**: No task uses @latest version (zero occurrences of @latest in the file)
- **camelCase variables**: Pipeline variable names use camelCase or snake_case (not mixed case like 'BuildConfiguration' or 'BUILD_CONFIGURATION')

## Failure Conditions

- Uses ubuntu-latest or windows-latest instead of a pinned vmImage version
- Any task references @latest or omits the @version suffix entirely
- Any step is missing a displayName property
- displayName values use title case or all caps instead of Sentence case
- No Cache@2 task is present for npm packages
- PublishTestResults task is absent or lacks condition: succeededOrFailed()
- Trigger block uses `trigger: none` or does not include main and develop branches
- Uses npm install instead of npm ci
- Any @latest version reference appears in the file
