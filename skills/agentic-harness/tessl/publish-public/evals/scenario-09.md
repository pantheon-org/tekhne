# Scenario 09: Invalid plugin.json Field Detection

## User Prompt

"Check if this plugin.json is ready for public publishing:

```json
{
  "name": "MyWorkspace/Cool_Skill",
  "version": "v1.0",
  "private": "false",
  "description": "A useful plugin",
  "skills": []
}
```"

## Expected Behavior

1. Agent analyzes the provided plugin.json content
2. Systematically validates each field against publication requirements
3. Identifies ALL validation errors present:
   - **name**: Invalid format (UpperCase, underscore) - must be lowercase kebab-case
   - **version**: Invalid format (`v` prefix, missing patch) - must be `x.y.z`
   - **private**: Wrong type (string `"false"`) - must be boolean `false`
   - **description**: Too generic and short - needs descriptive content with use cases
   - **skills**: Empty array - must have at least one skill path defined
4. Explains WHY each field fails validation
5. Provides correct examples for each failed field
6. Clearly states the plugin.json is NOT ready for publishing
7. Provides corrected version with all issues fixed

## Success Criteria

- Agent identifies all 5 validation errors (not partial)
- Agent explains the specific problem with each field
- Agent provides correct format examples for each error
- Agent shows corrected plugin.json with all fixes applied
- Agent explains the requirements clearly
- Agent confirms plugin.json is NOT ready for publishing
- Agent does NOT suggest workarounds or "it might work anyway"
- Agent emphasizes boolean vs string for `private` field
- Agent shows correct name format: `myworkspace/cool-skill`
- Agent shows correct version format: `1.0.0`
- Agent shows example of descriptive description with use cases
- Agent shows example of skills array with at least one skill path

## Failure Conditions

- Agent misses any of the 5 validation errors
- Agent says "looks good" or "mostly ready"
- Agent suggests "try publishing anyway"
- Agent doesn't explain WHY fields are invalid
- Agent provides vague error messages
- Agent doesn't show corrected version
- Agent accepts string `"false"` for private field
- Agent accepts version `v1.0` or `1.0`
- Agent accepts generic description "useful plugin"
- Agent accepts empty skills array
- Agent accepts name with uppercase or underscores
- Agent proceeds to publish command despite errors
