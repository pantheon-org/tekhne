# Scenario 02: Semantic Scholar Search — Paper Lookup by arxiv ID

## User Prompt

The user provides the arxiv ID `2307.09288` (Llama 2) and asks you to fetch the full metadata and then triage it.

1. Use the `paper` subcommand to fetch the metadata for this paper.
2. Present the result to the user.
3. Ask for confirmation before triaging.
4. If the user confirms, hand the arxiv ID to `triage-paper`.

## Expected Behavior

1. Invoke the script with the `paper` subcommand and `--paper-id 'arXiv:2307.09288'` — include the `arXiv:` prefix; the raw ID alone is incorrect
2. Display at minimum: title, authors, year, abstract summary, and citation count from the API response
3. Ask the user whether to proceed with triage-paper before invoking it
4. When handing off to triage-paper, pass the arxiv ID (`2307.09288`) rather than the Semantic Scholar internal paper ID
5. Refrain from creating `references/<slug>.md` or any other triage artifact before the user confirms

## Success Criteria

- **paper subcommand used with arXiv: prefix**: The agent invokes the script with the 'paper' subcommand and passes --paper-id 'arXiv:2307.09288' (with the arXiv: prefix). Using the raw ID '2307.09288' without the prefix is incorrect.
- **Full metadata is presented to the user**: The agent displays at minimum: title, authors, year, abstract summary, and citation count from the API response.
- **User confirmation requested before triaging**: The agent asks the user whether to proceed with triage-paper before invoking it.
- **triage-paper is handed the arxiv ID, not the S2 paper ID**: When handing off to triage-paper, the agent passes the arxiv ID (2307.09288) rather than the Semantic Scholar internal paper ID.
- **No file is created before user confirmation**: The agent does not create references/<slug>.md or any other triage artifact before the user confirms.

## Failure Conditions

- Agent passes the raw ID `2307.09288` without the `arXiv:` prefix to the paper subcommand
- Agent omits one or more required metadata fields (title, authors, year, abstract summary, citation count)
- Agent invokes triage-paper without first asking for user confirmation
- Agent passes the Semantic Scholar internal paper ID to triage-paper instead of the arxiv ID
- Agent creates a triage artifact file before the user confirms
