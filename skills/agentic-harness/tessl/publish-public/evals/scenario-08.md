# Scenario 08: Edge Case - Missing plugin.json

## User Prompt

"Publish the new custom-logging skill publicly to Tessl."

## Expected Behavior

1. Agent locates skill directory at `skills/observability/custom-logging/`
2. Agent checks for `.tessl-plugin/plugin.json` file existence
3. Agent discovers plugin.json is missing
4. Agent recognizes this is a new skill without Tessl plugin manifest yet
5. Agent explains plugin.json is required and must be created
6. Agent shows the required fields: `name`, `version`, `private`, `description`, `skills`
7. Agent blocks publication until plugin.json exists and is configured correctly
8. Agent provides step-by-step guidance for creating plugin.json and configuring it

## Success Criteria

- Agent checks for plugin.json existence
- Agent identifies missing plugin.json as blocker
- Agent explains the plugin.json format (name, version, private, description, skills array)
- Agent does NOT attempt publication without plugin.json
- Agent provides complete workflow for new plugin publishing
- Agent mentions configuring `private: false`
- Agent shows the correct plugin.json structure (object config, not tile.json format)
- Agent sets proper expectations for multi-step process

## Failure Conditions

- Agent attempts `tessl plugin publish` without plugin.json
- Agent doesn't check for plugin.json existence
- Agent suggests using deprecated tile.json format
- Agent skips manifest creation step entirely
- Agent doesn't explain why plugin.json is necessary
- Agent provides incomplete workflow guidance
- Agent doesn't mention `private: false` configuration requirement
