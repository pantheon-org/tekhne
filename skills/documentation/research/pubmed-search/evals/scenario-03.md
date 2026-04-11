# Scenario 03: PubMed Search — Advanced Filtered Search with Rate-Limit Handling

## User Prompt

The user wants to find papers about "Alzheimer's disease biomarkers" published in the journal "Nature Neuroscience" between 2020 and 2024. They want results exported to a JSON file at `/tmp/alzheimer-candidates.json`.

Run the advanced search with the appropriate filters. If a rate-limit error occurs, handle it correctly (retry once after a delay; do not loop). Present the candidate list and ask the user which papers to triage.

## Expected Behavior

1. Invoke the script with `--term 'Alzheimer disease biomarkers'`, `--journal 'Nature Neuroscience'`, `--start-date '2020'`, `--end-date '2024'`, `--format json`, and `--output /tmp/alzheimer-candidates.json`
2. Confirm that results were saved to `/tmp/alzheimer-candidates.json`
3. If a 429 or empty response occurs, retry exactly once after a delay — do not loop; recommend obtaining an API key or using the PubTator MCP as fallback
4. Present the result list and ask the user which papers to triage rather than auto-triaging

## Success Criteria

- **search subcommand used with correct filters**: The agent invokes the script with --term 'Alzheimer disease biomarkers' (or equivalent), --journal 'Nature Neuroscience', --start-date '2020', --end-date '2024', --format json, and --output /tmp/alzheimer-candidates.json.
- **JSON output saved to the correct path**: The agent confirms that results were saved to /tmp/alzheimer-candidates.json.
- **Rate-limit handling is correct**: If a 429 or empty response occurs, the agent retries once after a delay (not in a loop) and recommends obtaining an API key or using the PubTator MCP as fallback.
- **Candidate list presented and user asked for triage confirmation**: The agent presents the result list and asks the user which papers to triage, rather than auto-triaging.

## Failure Conditions

- Agent omits one or more required filters (term, journal, date range, format, or output path)
- Agent does not confirm that results were saved to the specified output path
- Agent retries more than once, retries without a delay, or enters a retry loop on rate-limit errors
- Agent automatically triages papers without asking the user for confirmation
