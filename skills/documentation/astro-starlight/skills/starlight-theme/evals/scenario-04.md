# Scenario 04: Light and Dark Mode Color Variants

## User Prompt

"How do I set different background colors for light and dark mode in Starlight?"

## Expected Behavior

1. Uses `:root[data-theme='light']` and `:root[data-theme='dark']` selectors
2. Does NOT use `.dark` class selectors
3. Registers the CSS in `customCss`

## Success Criteria

- Correct `data-theme` attribute selectors used
- No `.dark` class selectors
- Changes registered in `customCss`

## Failure Conditions

- Uses `.dark` class selectors instead of `[data-theme='dark']`
- Does not register the CSS in `customCss`
- Uses incorrect attribute names for the theme selectors
