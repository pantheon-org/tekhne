# Scenario 08: Template Validation: Errors in Referenced Template Files

## User Prompt

A DevOps engineer has set up a modular Azure Pipelines configuration with a main entry point that delegates build and deploy stages to YAML templates. The main file passes validation on its own, but the pipeline fails at queue time with errors in the template files. The engineer realizes they only validated the entry point.

Validate both the main pipeline file and the referenced template below. Identify all errors in the template files (not just the entry point), explain why validating only the entry point is insufficient, and produce corrected versions of all files.
