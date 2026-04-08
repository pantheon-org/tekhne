# Scenario 01: Override the Footer Component

## User Prompt

"How do I replace the Starlight footer with my own custom footer?"

## Expected Behavior

1. Creates `src/components/CustomFooter.astro` with custom markup
2. Registers `Footer: './src/components/CustomFooter.astro'` in `components: {}` in `astro.config.mjs`
3. Uses path without leading slash

## Success Criteria

- Correct path format (`'./src/...'` not `'/src/...'`)
- Exact component name `Footer` as key in `components: {}`
- `astro.config.mjs` shows full integration registration

## Failure Conditions

- Path uses a leading slash (`'/src/...'`)
- Component key in `components: {}` is incorrect (e.g., `footer` instead of `Footer`)
- Registration in `astro.config.mjs` is omitted or incomplete
