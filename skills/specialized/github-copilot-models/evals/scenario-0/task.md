# Model Audit Script for Large Codebase Tasks

## Problem Description

A platform engineering team runs automated code review across large monorepos. They need a shell script that fetches their available GitHub Copilot models and outputs only the ones suitable for large-context analysis: must be currently active (not restricted or preview-only), must have at least 100,000 token context, and should show the actual context window size alongside the model ID. The output will be piped into another tool that selects the model for each job.

Write a shell script called `find-large-context-models.sh` that performs this query and filtering. The script should print one line per qualifying model in the format: `<model-id> <context-tokens>`.
