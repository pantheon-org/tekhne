# Scenario 04: Comprehensive plugin.json Validation

## User Prompt

"Validate the .tessl-plugin/plugin.json for ansible-generator skill and ensure ALL required fields are present for public publishing."

## Expected Behavior

1. Agent locates plugin.json at `skills/infrastructure/ansible-generator/.tessl-plugin/plugin.json`
2. Validates JSON syntax is correct
3. Checks ALL required fields with specific validation rules:
   - **private**: Must be explicitly `false` (boolean, not string)
   - **name**: Must match `workspace/plugin-name` format (lowercase, kebab-case)
   - **version**: Must follow semantic versioning `x.y.z` (e.g., `1.0.0`, no `v` prefix)
   - **description**: Must be 150-300 chars, descriptive (not generic), includes use cases
   - **skills**: Must be an array of strings with at least one valid path to existing SKILL.md
4. Validates skill paths point to existing files
5. Checks each SKILL.md has YAML frontmatter with `name` and `description`
6. Reviews optional but recommended fields:
   - **files**: Additional bundled assets (if present)
7. Reports any missing, invalid, or incorrectly formatted fields
8. Provides specific remediation for each issue found

## Success Criteria

- Agent validates all 5 required fields (private, name, version, description, skills)
- Agent catches `private: true` as blocker (must be `false`)
- Agent validates name format matches `workspace/plugin-name` regex
- Agent validates version matches `x.y.z` regex pattern
- Agent checks description length (100-500 chars) and quality (not generic)
- Agent verifies skills array is not empty
- Agent confirms all skill paths point to existing SKILL.md files
- Agent checks SKILL.md frontmatter for required fields
- Agent detects legacy tile.json format (skills as object) and flags it
- Agent reports specific error messages for each validation failure
- Agent provides concrete examples of correct format for each failed field
- Agent does NOT proceed to publishing if any required field is missing/invalid

## Failure Conditions

- Agent only checks `private` field, ignores other required fields
- Agent accepts `private: "false"` (string) instead of requiring boolean `false`
- Agent accepts invalid name formats (UpperCase, underscores, no slash)
- Agent accepts invalid version formats (`1.0`, `v1.0.0`, `1.0.0-beta`)
- Agent accepts generic descriptions ("useful skill", "helpful tool")
- Agent doesn't validate skill paths exist
- Agent doesn't check SKILL.md frontmatter
- Agent accepts legacy object format for skills
- Agent provides vague errors ("something wrong with plugin.json")
- Agent proceeds to publish with validation failures
