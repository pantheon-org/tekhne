# Scenario 01: PubMed Search — Basic Keyword Discovery

## User Prompt

The user wants to find recent papers on "CRISPR gene therapy" to build a candidate list for literature review.

Search PubMed for this topic and return 10 candidates. No MCP server is currently configured.

Do not triage any of the results automatically.

## Expected Behavior

1. Check whether a PubTator or PubMed MCP server is configured before falling back to the Python script
2. Invoke the script with the `search` subcommand and `--keywords 'CRISPR gene therapy'`, returning up to 10 results
3. Present a formatted candidate list (titles and ideally authors, PMIDs, or DOIs) framed for user review
4. Refrain from invoking triage-paper or creating any reference/analysis file without explicit user confirmation
5. Ask the user which candidates they would like to triage rather than proceeding silently

## Success Criteria

- **MCP availability check performed before invoking script**: The agent explicitly checks whether a PubTator or PubMed MCP server is configured before falling back to the Python script.
- **search subcommand used with --keywords flag**: The agent invokes the script with the 'search' subcommand and passes --keywords 'CRISPR gene therapy' (or equivalent). The --results flag is set to 10 or left at default.
- **Candidate list is presented to the user**: The agent displays a list of result titles (and ideally authors, PMIDs, or DOIs) framed as candidates for the user to review.
- **No paper is triaged automatically**: The agent does not invoke triage-paper or create any reference/analysis file without explicit user confirmation.
- **User is offered a handoff to triage-paper**: The agent asks the user which (if any) of the candidates they would like to triage, rather than proceeding silently.

## Failure Conditions

- Agent invokes the Python script without first checking for an MCP server
- Agent uses the wrong subcommand or omits the --keywords flag
- Agent does not present results as a candidate list for user review
- Agent automatically triages one or more papers without user confirmation
- Agent proceeds to triage-paper without asking which papers the user wants to triage
