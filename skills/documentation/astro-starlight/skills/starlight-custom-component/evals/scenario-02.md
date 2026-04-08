# Scenario 02: Add a Banner Above the Header

## User Prompt

"I want to add an announcement banner above the Starlight header without replacing it."

## Expected Behavior

1. Creates a component that imports `Default` from `@astrojs/starlight/components/Header.astro`
2. Renders custom banner before `<Default><slot /></Default>`
3. Includes `<slot />` inside `<Default>`

## Success Criteria

- Uses wrapping pattern (not full replacement)
- `<slot />` present inside `<Default>`
- Banner rendered above (before) the `<Default>` call

## Failure Conditions

- Replaces the default header entirely instead of wrapping it
- Omits `<slot />` inside `<Default>`, breaking child content
- Renders banner after the `<Default>` call instead of before
