# Scenario 08: Edge Case - Missing tile.json

## User Prompt

"Publish the new custom-logging skill publicly to Tessl."

## Expected Behavior

1. Agent locates skill directory at `skills/observability/custom-logging/`
2. Agent checks for tile.json file existence
3. Agent discovers tile.json is missing
4. Agent recognizes this is a new skill without Tessl import yet
5. Agent suggests running `tessl skill import skills/observability/custom-logging` first
6. Agent explains import creates tile.json with proper structure
7. Agent blocks publication until tile.json exists and is configured
8. Agent provides step-by-step guidance for import → configure → publish workflow

## Success Criteria

- Agent checks for tile.json existence
- Agent identifies missing tile.json as blocker
- Agent suggests `tessl skill import` command
- Agent explains import process and purpose
- Agent does NOT attempt publication without tile.json
- Agent provides complete workflow for new skill publishing
- Agent mentions configuring `private: false` after import
- Agent sets proper expectations for multi-step process

## Failure Conditions

- Agent attempts `tessl skill publish` without tile.json
- Agent doesn't check for tile.json existence
- Agent manually creates tile.json instead of using import
- Agent skips import step entirely
- Agent doesn't explain why import is necessary
- Agent provides incomplete workflow guidance
- Agent doesn't mention private: false configuration requirement
