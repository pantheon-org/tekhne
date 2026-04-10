# Scenario 03: Add a Custom Tool to a Plugin

## User Prompt

A user wants to create an OpenCode plugin that adds a custom tool called "jira-lookup" that the AI can invoke to look up JIRA ticket details. The tool should accept a "ticket_id" argument (string) and return the ticket summary as a string.

Show how to implement this plugin with the custom tool definition, including proper schema validation.

## Expected Behavior

1. Register the custom tool via the `tool` key in the returned hooks object — not via `client.registerTool()`
2. Import the `tool` helper from `@opencode-ai/plugin`
3. Define the `ticket_id` argument using `tool.schema.string()` (or a Zod equivalent) for schema validation
4. Add a `.describe()` call to the `ticket_id` schema field
5. Implement an `execute` function that returns a string value

## Success Criteria

- **Tool registered via tool key (not client.registerTool)**: Plugin uses `tool: { 'jira-lookup': tool({...}) }` in returned hooks, NOT `client.registerTool()`
- **Imports tool helper correctly**: Imports `tool` from `@opencode-ai/plugin`
- **Uses tool.schema for argument validation**: Uses `tool.schema.string()` (or Zod equivalent) for the ticket_id argument
- **Schema field has .describe()**: The ticket_id schema field includes a `.describe()` call
- **Execute returns string**: The execute function returns a string value

## Failure Conditions

- Registers the tool using `client.registerTool()` instead of the `tool` key in the hooks object
- Imports `tool` from a package other than `@opencode-ai/plugin`
- Defines the `ticket_id` argument without any schema validation
- Omits `.describe()` from the ticket_id schema field
- The `execute` function returns an object or `void` instead of a string
