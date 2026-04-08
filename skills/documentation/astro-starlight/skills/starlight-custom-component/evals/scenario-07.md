# Scenario 07: Anti-Pattern Detection — Wrong Registration Key

## User Prompt

"I created `Footer.astro` and put it in `src/components/` but the default footer still shows. I didn't register anything in config. Is naming it Footer.astro enough?"

## Expected Behavior

1. Explains that file name has no effect on which slot is overridden
2. Instructs registering `Footer: './src/components/Footer.astro'` in `components: {}` in `astro.config.mjs`

## Success Criteria

- Clarifies file name is irrelevant
- Correct `components: {}` registration shown
- No suggestion that naming conventions alone work

## Failure Conditions

- Implies that naming a file `Footer.astro` is sufficient for the override to take effect
- Does not show the required `components: {}` registration in `astro.config.mjs`
