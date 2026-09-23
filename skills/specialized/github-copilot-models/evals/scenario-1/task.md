# Scenario 01: Model Audit Script for Large Codebase Tasks

## User Prompt

A platform engineering team runs automated code review across large monorepos. They need a shell script that fetches their available GitHub Copilot models and outputs only the ones suitable for large-context analysis: must be currently active (not restricted or preview-only), must have at least 100,000 token context, and should show the actual context window size alongside the model ID. The output will be piped into another tool that selects the model for each job.

Write a shell script called `find-large-context-models.sh` that performs this query and filtering. The script should print one line per qualifying model in the format: `<model-id> <context-tokens>`.

## Expected Behavior

1. Invoke the `fetch-models.sh` skill script or an equivalent direct API query — not `opencode models` or OpenCode's built-in model listing
2. Pass the `--json` flag (or equivalent) to get structured output suitable for `jq` parsing
3. Filter models using `.policy.state == 'enabled'` (or equivalent) — not just listing all models
4. Read `.capabilities.limits.max_context_window_tokens` (or equivalent nested path) to check context size — not parsing the model name/ID string
5. Output the model ID and context window size per qualifying model
6. Filter for context window greater than or equal to 100,000 tokens

## Success Criteria

- **Uses fetch-models.sh**: Script invokes the `fetch-models.sh` skill script (or equivalent direct API query) — not `opencode models` or OpenCode's built-in model listing
- **JSON flag used**: Script passes `--json` flag (or equivalent) to get structured output suitable for `jq` parsing
- **Filters by policy.state**: Script filters models using `.policy.state == 'enabled'` (or equivalent) — not just listing all models
- **Uses context window field**: Script reads `.capabilities.limits.max_context_window_tokens` (or equivalent nested path) to check context size — not parsing the model name/ID string
- **Output format correct**: Script outputs model ID and context window size per qualifying model
- **Threshold applied**: Script filters for context window >= 100,000 tokens

## Failure Conditions

- Script uses `opencode models` or OpenCode's registry instead of querying the GitHub Copilot API directly
- Script does not use `--json` or equivalent flag for structured output
- Script lists all models without filtering by `policy.state`
- Context window size is inferred from the model name or ID string instead of reading the API capabilities field
- Output does not include both the model ID and context window size
- Script does not apply the 100,000 token threshold filter
