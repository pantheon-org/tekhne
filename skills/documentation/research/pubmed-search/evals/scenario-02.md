# Scenario 02: PubMed Search — Metadata Lookup by PMID

## User Prompt

The user has a specific PubMed ID (PMID 33303479) and wants to retrieve the full metadata for that article, including title, authors, abstract, journal, and DOI.

Fetch and display the metadata in JSON format. Do not perform any triage or analysis unless the user explicitly asks.

## Expected Behavior

1. Use the `metadata` subcommand with `--pmid '33303479'` and `--format json`
2. Return output that includes all required metadata fields: PMID, Title, Authors, Journal, Publication Date, Abstract, and DOI
3. Present the output as valid, parseable JSON (not plain text or a markdown table)
4. Refrain from invoking triage-paper or the `analyze` subcommand without an explicit user request

## Success Criteria

- **metadata subcommand used with correct --pmid flag**: The agent invokes the script with the 'metadata' subcommand and passes --pmid '33303479' and --format json.
- **All required metadata fields are present in output**: The returned JSON includes PMID, Title, Authors, Journal, Publication Date, Abstract, and DOI fields.
- **Output is valid JSON**: The agent presents output that is parseable JSON (not plain text or markdown table).
- **No automatic triage or analysis performed**: The agent does not invoke triage-paper or the analyze subcommand without explicit user request.

## Failure Conditions

- Agent uses the wrong subcommand or incorrect PMID value
- Agent omits one or more required metadata fields (PMID, Title, Authors, Journal, Publication Date, Abstract, DOI)
- Agent presents output as plain text or a markdown table rather than valid JSON
- Agent automatically invokes triage-paper or analysis without user request
