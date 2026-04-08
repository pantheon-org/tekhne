# Scenario 02: Add Tailwind to Existing Starlight Project

## User Prompt

"I want to add Tailwind CSS to my existing Starlight project."

## Expected Behavior

1. Runs `npx astro add tailwind`
2. Runs `npm install @astrojs/starlight-tailwind`
3. Replaces `global.css` with correct layer order starting with `@import '@astrojs/starlight-tailwind'`
4. Updates `astro.config.mjs` with `vite: { plugins: [tailwindcss()] }` and registers the CSS file in `customCss`

## Success Criteria

- Compatibility package installed
- Layer order: `@astrojs/starlight-tailwind` before `tailwindcss/theme.css`
- `customCss` updated to point to the CSS file

## Failure Conditions

- Omits installing `@astrojs/starlight-tailwind`
- CSS layer order puts Tailwind before the Starlight compatibility import
- Does not register the CSS file in `customCss`
