# Scenario 02: Control OpenCode Programmatically via the SDK Client

## User Prompt

Write a Node.js/TypeScript script that uses the OpenCode SDK client to:
1. Connect to a running OpenCode instance on port 4096
2. Create a new session
3. Send the message "List the files in the current directory"
4. Print the response to console

Show the correct imports, client initialization, and API calls.

## Expected Behavior

1. Import `createOpencode` or `createOpencodeClient` from `@opencode-ai/sdk`
2. Initialize the client with the correct hostname/port (4096) or equivalent `baseUrl`
3. Call `client.session.create()` to create a new session before sending any prompts
4. Use `client.session.prompt` with a `parts` array containing the text message to send the prompt
5. Access `.data` from the response and print or log it to the console

## Success Criteria

- **Imports from @opencode-ai/sdk**: Imports `createOpencode` or `createOpencodeClient` from `@opencode-ai/sdk`
- **Correct client initialization**: Uses `createOpencode` with hostname/port or `createOpencodeClient` with baseUrl
- **Creates a session**: Calls `client.session.create()` to create a new session
- **Sends prompt via client.session.prompt**: Uses `client.session.prompt` with parts array containing text message
- **Handles response data**: Accesses `.data` from the response and prints/logs it

## Failure Conditions

- Imports from `@opencode-ai/plugin` instead of `@opencode-ai/sdk`
- Omits client initialization or uses an incorrect connection method
- Skips session creation and sends a prompt without an active session
- Uses an incorrect API method to send the prompt (e.g., not `client.session.prompt`)
- Does not access or print `.data` from the response
