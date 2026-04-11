# Scenario 02: Show Toast Notifications on File Edit Events

## User Prompt

Create an OpenCode plugin that shows a toast notification whenever a file is edited. The toast should say "File edited: {filename}". Use the event hook to listen for file edit events.

## Expected Behavior

1. Implement the plugin as an async factory function with correct TypeScript typing
2. Use the `event` hook to listen for events from OpenCode
3. Declare the event handler as `async`
4. Filter incoming events to handle only file edit/write events
5. Call the appropriate TUI API (e.g., `client.tui.showToast`) to display the notification with the filename

## Success Criteria

- **Uses event hook**: Plugin implements the `event` hook to listen for events
- **Async event handler**: The event handler is async
- **Filters for file edit events**: Checks event type to handle only file edit/write events
- **Uses client.tui.showToast or equivalent**: Calls the appropriate TUI API to display the toast notification
- **Plugin structure is correct**: Async factory function returning hooks object with proper TypeScript typing

## Failure Conditions

- Uses a synchronous hook handler instead of `async`
- Does not filter events and reacts to all event types indiscriminately
- Fails to call any TUI API to display the toast (e.g., only logs to console)
- Implements the plugin as a plain object rather than an async factory function
- Does not use the `event` hook (e.g., polls for changes instead)
