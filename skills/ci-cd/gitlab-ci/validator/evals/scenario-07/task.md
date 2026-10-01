# Scenario 07: Include File Validation: Errors in Locally Included Templates

## User Prompt

A platform team has split a large `.gitlab-ci.yml` into modular include files. The main file passes the GitLab CI Lint tool with no issues, but pipelines are failing at queue time with cryptic error messages. A senior engineer suspects the problem is in the included templates, which were never validated independently.

Validate both the main file and the included template below. Identify all errors in any file, explain why entry-point-only validation does not surface these issues, and produce corrected versions of all files.
