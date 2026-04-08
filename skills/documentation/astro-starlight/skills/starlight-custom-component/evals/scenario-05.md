# Scenario 05: Override TwoColumnContent Without Losing the Right Sidebar

## User Prompt

"I need to wrap TwoColumnContent but the right sidebar disappeared. What did I do wrong?"

## Expected Behavior

1. Identifies missing named slot transfer as the problem
2. Provides fix: `<slot name="right-sidebar" slot="right-sidebar" />`
3. Shows full corrected component

## Success Criteria

- Named slot transfer identified as the fix
- Correct slot syntax shown: `<slot name="right-sidebar" slot="right-sidebar" />`
- Both default `<slot />` and named slot included

## Failure Conditions

- Does not identify the missing named slot transfer as the root cause
- Shows incorrect slot syntax
- Corrected component omits either the default `<slot />` or the named slot
