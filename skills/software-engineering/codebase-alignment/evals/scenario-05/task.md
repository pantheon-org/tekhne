# Scenario 05: Remediation Workflow — Report Then Fix

## User Prompt

Standards provided by the user:
1. "No `var` keyword anywhere"
2. "Files must be kebab-case"

Report produced by the agent:
- `var` found in `src/old/legacy.js:23`, `src/utils/migration.js:45`, `src/utils/migration.js:88`
- PascalCase file found: `src/components/StarChart.vue` (should be `star-chart.vue`)
- All other files comply with kebab-case

## Context

The agent has completed the scan and presented the report. Now the user says:

"Great, please fix all of those."

## Expected Behavior

1. Before fixing, the agent confirms the scope with the user:
   - Convert `var` to `const`/`let` in 3 locations
   - Rename `StarChart.vue` to `star-chart.vue` (and update all imports)
2. The agent notes any ambiguity: renaming a component file may break imports across the project.
3. The agent applies the fixes after user confirms the scope.
4. The agent reports what was changed after applying fixes.

## Success Criteria

- The agent does NOT auto-fix without first confirming the scope.
- Renaming a file that may affect imports is flagged as a dependency risk, not silently done.
- The agent reports the changes made after applying fixes.
- Fixes are minimal and correct (e.g. `var` → `const` where appropriate, not a blanket regex).

## Failure Conditions

- The agent says "done" without confirming first.
- The agent renames a component file without checking or updating imports.
- The agent uses a blanket regex replacement that changes unintended code.
- The agent reports but does not offer to fix when asked.
