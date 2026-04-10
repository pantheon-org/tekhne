# Scenario 03: Plugin Develop/Test Cycle

## User Prompt

You are helping a developer iterate on their Obsidian plugin. The plugin ID is `my-task-tracker`. They have just saved edits to `main.ts` and want to pick up the changes and verify everything is working correctly.

Walk through the complete develop/test cycle for the plugin changes:

1. Reload the plugin to pick up the code changes.
2. Check whether any errors were thrown during load.
3. Verify the plugin's UI rendered correctly by capturing a screenshot or inspecting the relevant DOM element.
4. Check for any unexpected console warnings or errors.

Show the obsidian CLI commands you would run for each step.

## Expected Behavior

1. Use `obsidian plugin:reload id=my-task-tracker` to reload the plugin and pick up code changes
2. Run `obsidian dev:errors` to detect any errors thrown during plugin load
3. Capture a screenshot with `dev:screenshot` or inspect a DOM element with `dev:dom` to confirm the UI rendered correctly
4. Use `obsidian dev:console` to inspect console logs for unexpected warnings or errors

## Success Criteria

- **Reloads plugin with plugin:reload**: Uses `obsidian plugin:reload id=my-task-tracker` to pick up the code changes
- **Checks errors with dev:errors**: Runs `obsidian dev:errors` to detect any errors thrown during plugin load
- **Verifies visually with dev:screenshot or dev:dom**: Captures a screenshot with `dev:screenshot` or inspects a DOM element with `dev:dom` to confirm the UI rendered correctly
- **Checks console output with dev:console**: Uses `obsidian dev:console` to inspect console logs for unexpected warnings or errors

## Failure Conditions

- Does not use `plugin:reload` to reload the plugin after code changes
- Skips the `dev:errors` check after reloading
- Provides no visual verification step (neither `dev:screenshot` nor `dev:dom`)
- Omits the `dev:console` check for console warnings and errors
