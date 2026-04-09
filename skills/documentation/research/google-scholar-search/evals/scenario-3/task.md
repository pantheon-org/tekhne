# Google Scholar Search — Blocked Response Handling

## Task

The user asks to search for papers on "mixture of experts". You run the script and the response body is unusually short (~800 bytes) with no results — a sign of a CAPTCHA redirect or rate-limit block.

Handle this situation correctly: detect the block, retry once, and if still blocked, offer the semantic-scholar MCP as a fallback. Do not retry in a loop.

No MCP server is currently configured, but you should inform the user they can configure one.
