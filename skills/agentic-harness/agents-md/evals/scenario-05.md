# Scenario 05: Update AGENTS.md After Repository Changes

## User Prompt

Six months ago, your team created an AGENTS.md file for the project. Since then, significant changes have been made:

- Migrated from Jest to Vitest for testing
- Added ESLint flat config
- Removed Storybook
- Added a new `lint:fix` script

The current AGENTS.md is now outdated and contains commands that no longer work. You need to update the documentation to reflect the current state of the repository.

## Output Specification

Update the existing `AGENTS.md` to:

- Replace Jest commands with Vitest equivalents
- Update linting commands for ESLint flat config
- Remove Storybook-related instructions
- Add the new `lint:fix` script
- Ensure all commands are still valid

## Expected Behavior

1. Compare the current AGENTS.md against the repository's config files to detect what has changed (new scripts, removed dependencies, updated tooling)
2. Remove or update stale content: replace Jest commands with Vitest equivalents, remove Storybook instructions
3. Add new content: document the `lint:fix` script and any ESLint flat config changes
4. Verify the documentation structure is still appropriate for the project's current state
5. Validate that all commands in the updated file are still correct and executable
6. Avoid introducing duplication while adding new content

## Success Criteria

- **Change detection**: Agent detected what changed in the repository (new scripts, removed dependencies, etc.)
- **Removed stale content**: Removed or updated instructions for removed tooling/commands
- **Added new content**: Added instructions for new tooling or commands
- **Structure still appropriate**: Documentation structure remains appropriate for project (simple vs hierarchical)
- **Command validation**: Validated that commands still work after changes
- **No duplication introduced**: Did not introduce new duplication during update
- **References updated**: References to external docs are still valid

## Failure Conditions

- Leaves Jest commands in the documentation after the migration to Vitest
- Keeps Storybook instructions that reference removed dependencies
- Fails to document the new `lint:fix` script
- Validates nothing and delivers an updated file without checking whether commands still work
- Introduces duplicate instructions by appending new content without removing the old equivalents
- Updates only part of the file, leaving other sections pointing to stale tool versions
