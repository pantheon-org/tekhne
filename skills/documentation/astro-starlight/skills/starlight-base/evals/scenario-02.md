# Scenario 02: Add Starlight to Existing Astro Project

## User Prompt

"I have an existing Astro project and want to add Starlight to it."

## Expected Behavior

1. Runs `npx astro add starlight`
2. Notes that this updates `astro.config.mjs` automatically
3. Does NOT instruct manual `npm install @astrojs/starlight` + manual config editing

## Success Criteria

- Uses `astro add` integration command
- Mentions automatic config update

## Failure Conditions

- Suggests manual `npm install @astrojs/starlight` instead of `astro add starlight`
- Instructs manual edits to `astro.config.mjs` when `astro add` handles it
