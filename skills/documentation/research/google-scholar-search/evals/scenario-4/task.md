# Scenario 4: Author Profile and JSON Export on a First Run

## User Prompt

"This is my first time using the Google Scholar search tool on this machine. Look up the author profile for Geoffrey Hinton, and also search for 'prompt compression' and save the 10 candidates to `/tmp/prompt-compression-candidates.json` so I can review them in a batch."

## Repo state

The skill's `scripts/google-scholar-search.py` exists, but no virtual environment has been created and no dependencies are installed. No `semantic-scholar` or `google-scholar` MCP server is configured.
