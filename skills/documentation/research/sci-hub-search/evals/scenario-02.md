# Scenario 02: Sci-Hub Search — Keyword Discovery

## User Prompt

The user wants to find papers on "federated learning privacy" to decide which ones to retrieve.

No MCP server is configured. Use the script to search by keyword and return up to 8 candidates.

Do not download or triage any results automatically.

## Expected Behavior

1. Invoke the script with `search --keyword "federated learning privacy"` and set `--results` to 8 or a reasonable value
2. Display a candidate list of result titles, authors, years, and DOIs framed for user review
3. Note that results are returned by CrossRef relevance ranking and do not represent all papers on the topic
4. Refrain from downloading any PDF or invoking triage-paper without explicit user confirmation

## Success Criteria

- **Keyword subcommand used with --keyword flag**: The agent invokes the script with 'search --keyword "federated learning privacy"' and sets --results to 8 or a reasonable value.
- **Candidate list is presented to the user**: The agent displays a list of result titles, authors, years, and DOIs framed as candidates for user review.
- **Results qualified as CrossRef-ranked, not exhaustive**: The agent notes that results are returned by CrossRef relevance ranking and do not represent all papers on the topic.
- **No automatic download or triage occurs**: The agent does not download any PDF or invoke triage-paper without explicit user confirmation.

## Failure Conditions

- Agent uses a DOI or title search instead of keyword search, or omits `--results`
- Agent does not display a formatted candidate list with titles, authors, years, and DOIs
- Agent presents results without qualifying them as CrossRef-ranked or noting they are non-exhaustive
- Agent automatically downloads PDFs or invokes triage-paper without user confirmation
