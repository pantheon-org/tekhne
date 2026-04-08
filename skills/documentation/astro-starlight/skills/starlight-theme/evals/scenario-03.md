# Scenario 03: Custom Font with Fontsource

## User Prompt

"How do I use the Inter font in my Starlight docs?"

## Expected Behavior

1. Runs `npm install @fontsource/inter`
2. Adds `'@fontsource/inter/400.css'` and `'@fontsource/inter/600.css'` to `customCss`
3. Sets `--sl-font: 'Inter', sans-serif` in a CSS file

## Success Criteria

- Installs the font package
- Registers specific weight files (not `@fontsource/inter` — no default export)
- Sets `--sl-font` variable

## Failure Conditions

- Registers `@fontsource/inter` without a weight path (no default export exists)
- Omits setting the `--sl-font` CSS variable
- Does not add font files to `customCss`
