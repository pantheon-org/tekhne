# Scenario 03: Google Scholar Search — Blocked Response Handling

## User Prompt

The user asks to search for papers on "mixture of experts". You run the script and the response body is unusually short (~800 bytes) with no results — a sign of a CAPTCHA redirect or rate-limit block.

Handle this situation correctly: detect the block, retry once, and if still blocked, offer the semantic-scholar MCP as a fallback. Do not retry in a loop.

No MCP server is currently configured, but you should inform the user they can configure one.

## Expected Behavior

1. Detect that the response body is unusually short or empty and report a potential CAPTCHA or rate-limit block to the user
2. Retry the search exactly once with at least a short delay (60 seconds or more recommended)
3. Stop after the second attempt fails — do not enter a retry loop
4. Inform the user that a semantic-scholar MCP server can be configured as a more reliable fallback, providing or referencing the MCP config
5. Honestly report the block — do not claim the query returned zero results

## Success Criteria

- **Block condition is detected and reported**: The agent identifies that the response body is unusually short or empty and explicitly reports a potential CAPTCHA or rate-limit block to the user.
- **Exactly one retry is performed after a delay**: The agent retries the search exactly once, with at least a short delay (recommended: 60 seconds or more). It does not retry immediately without delay.
- **No retry loop**: The agent does not retry more than once. After the second attempt fails, it stops and does not continue retrying.
- **semantic-scholar MCP is offered as fallback**: The agent informs the user that a semantic-scholar MCP server can be configured as a more reliable alternative, and provides or references the MCP config.
- **Block is documented honestly**: The agent does not claim the query returned zero results. It explicitly states that the search could not be completed due to a block.

## Failure Conditions

- Agent fails to detect or report the block condition
- Agent retries immediately without a delay, or performs more than one retry
- Agent enters a retry loop (more than two total attempts)
- Agent does not mention the semantic-scholar MCP as a fallback
- Agent falsely reports "zero results found" instead of acknowledging the block
