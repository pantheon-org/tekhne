# Scenario 02: Google Scholar Search — Advanced Author + Year Filter

## User Prompt

The user wants papers by "Ilya Sutskever" on the topic of "transformers" published between 2017 and 2021.

Run an advanced Google Scholar search using the appropriate subcommand and filters. No MCP server is configured.

Return the candidate list and make clear that the results are a non-exhaustive sample.

## Expected Behavior

1. Use the `advanced` subcommand rather than `search` to perform the filtered query
2. Pass `--author 'Ilya Sutskever'` to the script (case-insensitive)
3. Pass both `--year-start 2017` and `--year-end 2021` flags to the script
4. Qualify the returned results as a non-exhaustive sample, noting that Google Scholar's year filter is a ranking hint
5. Present results to the user as candidates without automatically triaging any paper

## Success Criteria

- **advanced subcommand is used**: The agent invokes the script with the 'advanced' subcommand rather than 'search'.
- **--author flag is set to 'Ilya Sutskever'**: The --author flag is passed with the correct author name (case-insensitive).
- **--year-start and --year-end flags are set**: Both --year-start 2017 and --year-end 2021 are passed to the script.
- **Results are qualified as non-exhaustive**: The agent explicitly notes that Google Scholar's year filter is a ranking hint and that results may not include all papers in the range.
- **Candidate list is returned without auto-triaging**: The results are presented to the user as candidates. No triage-paper invocation occurs without confirmation.

## Failure Conditions

- Agent uses the `search` subcommand instead of `advanced`
- Agent omits or incorrectly sets the `--author` flag
- Agent omits `--year-start` or `--year-end` (or sets incorrect values)
- Agent presents results without qualifying them as non-exhaustive
- Agent automatically triages papers without user confirmation
