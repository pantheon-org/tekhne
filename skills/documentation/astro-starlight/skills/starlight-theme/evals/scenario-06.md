# Scenario 06: Anti-Pattern Detection — Tailwind Dark Mode

## User Prompt

"I installed Tailwind with Starlight but `dark:bg-gray-900` never applies. What's wrong?"

## Expected Behavior

1. Identifies the issue: Starlight uses `data-theme="dark"`, not Tailwind's `.dark` class
2. Solution: install `@astrojs/starlight-tailwind` and import it before Tailwind's own CSS
3. After installing, Tailwind's `dark:` variant works correctly

## Success Criteria

- Root cause identified (wrong dark mode mechanism)
- `@astrojs/starlight-tailwind` mentioned as the fix
- No suggestion to manually configure Tailwind's `darkMode` option (the compat package handles it)

## Failure Conditions

- Does not identify the `data-theme` vs `.dark` class mismatch as the root cause
- Suggests manually configuring Tailwind's `darkMode` option instead of using the compat package
- Does not mention `@astrojs/starlight-tailwind`
