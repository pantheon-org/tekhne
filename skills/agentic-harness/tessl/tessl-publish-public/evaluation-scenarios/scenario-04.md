# Scenario 04: Comprehensive Tile.json Validation

## User Prompt

"Validate the tile.json for ansible-generator skill and ensure ALL required fields are present for public publishing."

## Expected Behavior

1. Agent locates tile.json at `skills/infrastructure/ansible-generator/tile.json`
2. Validates JSON syntax is correct
3. Checks ALL required fields with specific validation rules:
   - **private**: Must be explicitly `false` (boolean, not string)
   - **name**: Must match `workspace/tile-name` format (lowercase, kebab-case)
   - **version**: Must follow semantic versioning `x.y.z` (e.g., `1.0.0`, no `v` prefix)
   - **summary**: Must be 150-300 chars, descriptive (not generic), includes use cases
   - **skills**: Must have at least one skill with valid path to existing SKILL.md
4. Validates skill paths point to existing files
5. Checks each SKILL.md has YAML frontmatter with `name` and `description`
6. Reviews optional but recommended fields:
   - **keywords**: 3-8 relevant terms for discoverability
   - **docs**: Path to tile documentation (if present)
7. Reports any missing, invalid, or incorrectly formatted fields
8. Provides specific remediation for each issue found
9. References `references/tile-json-schema.md` for detailed field requirements

## Success Criteria

- Agent validates all 5 required fields (private, name, version, summary, skills)
- Agent catches `private: true` as blocker (must be `false`)
- Agent validates name format matches `workspace/tile-name` regex
- Agent validates version matches `x.y.z` regex pattern
- Agent checks summary length (100-500 chars) and quality (not generic)
- Agent verifies skills object is not empty
- Agent confirms all skill paths point to existing SKILL.md files
- Agent checks SKILL.md frontmatter for required fields
- Agent reports specific error messages for each validation failure
- Agent provides concrete examples of correct format for each failed field
- Agent does NOT proceed to publishing if any required field is missing/invalid
- Agent references tile-json-schema.md for detailed documentation

## Failure Conditions

- Agent only checks `private` field, ignores other required fields
- Agent accepts `private: "false"` (string) instead of requiring boolean `false`
- Agent accepts invalid name formats (UpperCase, underscores, no slash)
- Agent accepts invalid version formats (`1.0`, `v1.0.0`, `1.0.0-beta`)
- Agent accepts generic summaries ("useful skill", "helpful tool")
- Agent doesn't validate skill paths exist
- Agent doesn't check SKILL.md frontmatter
- Agent provides vague errors ("something wrong with tile.json")
- Agent doesn't reference tile-json-schema.md documentation
- Agent suggests workarounds instead of fixing invalid fields
- Agent proceeds to publish with validation failures
