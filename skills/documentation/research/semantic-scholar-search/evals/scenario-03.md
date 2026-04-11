# Scenario 03: Semantic Scholar Search — Author Profile Lookup

## User Prompt

The user asks for the Semantic Scholar author profile of "Noam Shazeer" including their h-index and recent publications.

The `author` subcommand requires a Semantic Scholar author ID, not a name. You must first locate the author ID, then fetch the profile.

## Expected Behavior

1. Recognise that the `author` subcommand requires a numeric Semantic Scholar author ID — do not attempt to pass the name `"Noam Shazeer"` directly to `--author-id`
2. Run a keyword or paper search and extract the `authorId` field from the response to find Noam Shazeer's Semantic Scholar author ID
3. Call `author --author-id <numeric-id>` with the correct ID found in the previous step
4. Present at minimum the author's h-index and a list of recent publications from the API response

## Success Criteria

- **Agent recognises that author subcommand requires an ID, not a name**: The agent does not attempt to run 'author --author-id "Noam Shazeer"'. It explicitly notes that a numeric Semantic Scholar author ID is required.
- **search subcommand used to locate the author ID**: The agent runs a keyword or paper search and extracts the authorId field from the response to find Noam Shazeer's Semantic Scholar author ID.
- **author subcommand called with the correct numeric author ID**: After locating the ID, the agent calls 'author --author-id <numeric-id>' with the correct ID found in the previous step.
- **Profile output includes h-index and publications**: The agent presents at minimum the author's h-index and a list of recent publications from the API response.

## Failure Conditions

- Agent passes the author name directly to `--author-id` without first resolving the numeric ID
- Agent does not use the `search` subcommand (or equivalent) to locate the author ID
- Agent calls `author --author-id` with an incorrect or mismatched numeric ID
- Agent omits the h-index or the list of recent publications from the presented profile
