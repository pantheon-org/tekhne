# Scenario 01: Custom Accent Color

## User Prompt

"How do I change the accent color in my Starlight site to indigo?"

## Expected Behavior

1. Creates a CSS file with `--sl-color-accent-low`, `--sl-color-accent`, and `--sl-color-accent-high` set
2. Registers the file in `customCss` array in `astro.config.mjs`
3. Does NOT add a raw `<style>` block in a layout component

## Success Criteria

- Correct variable names (`--sl-color-accent-*`)
- File registered in `customCss`
- Three accent variables provided (low/mid/high for full light+dark coverage)

## Failure Conditions

- Uses incorrect CSS variable names (e.g., `--accent-color` instead of `--sl-color-accent`)
- Adds a `<style>` block in a layout instead of a `customCss`-registered file
- Provides fewer than the three required accent variables
