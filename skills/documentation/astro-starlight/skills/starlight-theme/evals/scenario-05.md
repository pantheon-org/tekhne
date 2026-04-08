# Scenario 05: Anti-Pattern Detection — Global Style Tag

## User Prompt

"I added a `<style>` block to my layout with `--sl-color-accent: red` but nothing changed. Why?"

## Expected Behavior

1. Explains that `<style>` blocks outside `customCss` are not included in Starlight's build
2. Instructs creating a standalone CSS file and registering it in `customCss`

## Success Criteria

- Root cause identified (not in `customCss`)
- Correct fix provided (CSS file + `customCss` registration)

## Failure Conditions

- Does not identify the missing `customCss` registration as the root cause
- Suggests a workaround that bypasses `customCss` (e.g., inline `<style>` with `:global`)
