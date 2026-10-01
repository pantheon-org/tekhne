# Scenario 05: Syntax Validation: Stage References, Job Definitions, and Dependency Errors

## User Prompt

A backend team has migrated their CI/CD pipeline from a legacy tool to GitLab CI. The pipeline was assembled quickly and has never successfully run — it fails at pipeline creation with schema errors. The team has asked for a complete syntax analysis before they attempt to run it.

Validate the syntax of the `.gitlab-ci.yml` below. Identify all schema violations, invalid stage references, and dependency graph errors. For each error, explain the violation and the correction, then produce a corrected version of the file.
