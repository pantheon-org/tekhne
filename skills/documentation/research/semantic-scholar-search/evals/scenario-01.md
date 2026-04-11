# Scenario 01: Semantic Scholar Search — Keyword Discovery

## User Prompt

The user wants to find papers on "speculative decoding" to decide which ones to triage.

Search Semantic Scholar for this topic and return 8 candidates. No MCP server is configured.

Do not triage any results automatically.

## Expected Behavior

1. Invoke the script with the `search` subcommand and `--query 'speculative decoding'`, setting `--limit` to 8 or a reasonable value
2. Display a candidate list of result titles (and ideally authors, year, citation count) framed for user review
3. Note that results are ordered by Semantic Scholar relevance and do not represent all papers on the topic
4. Refrain from invoking triage-paper or creating any reference/analysis file without explicit user confirmation
5. Ask the user which candidates (if any) they would like to triage

## Success Criteria

- **search subcommand used with --query flag**: The agent invokes the script with the 'search' subcommand and passes --query 'speculative decoding' (or equivalent). --limit is set to 8 or a reasonable value.
- **Candidate list is presented to the user**: The agent displays a list of result titles (and ideally authors, year, citation count) framed as candidates for the user to review.
- **Results qualified as relevance-ranked, not exhaustive**: The agent notes that results are ordered by Semantic Scholar relevance and do not represent all papers on the topic.
- **No paper is triaged automatically**: The agent does not invoke triage-paper or create any reference/analysis file without explicit user confirmation.
- **User is offered a handoff to triage-paper**: The agent asks the user which candidates (if any) they would like to triage.

## Failure Conditions

- Agent uses the wrong subcommand or omits the `--query` flag
- Agent does not present a formatted candidate list with titles and metadata
- Agent presents results without qualifying them as relevance-ranked or noting they are non-exhaustive
- Agent automatically triages papers without user confirmation
- Agent does not offer the user a handoff to triage-paper
