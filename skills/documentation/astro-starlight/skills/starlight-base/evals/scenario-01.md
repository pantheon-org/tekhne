# Scenario 01: New Project Setup

## User Prompt

"I want to create a new Astro Starlight documentation site. Show me the setup steps."

## Expected Behavior

1. Runs `npm create astro@latest -- --template starlight` (or pnpm/yarn equivalent)
2. Runs `npm run dev` to start dev server
3. Does NOT manually install `@astrojs/starlight` separately (template handles it)

## Success Criteria

- Correct scaffolding command used
- Dev server command included
- No unnecessary manual steps added

## Failure Conditions

- Instructs `npm install @astrojs/starlight` as a separate step
- Omits the dev server command
- Requires manual `astro.config.mjs` editing when using the template
