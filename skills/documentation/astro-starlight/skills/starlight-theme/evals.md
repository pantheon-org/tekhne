# Eval Scenarios: starlight-theme

## Scenario 1: Custom accent color

**Prompt:** "How do I change the accent color in my Starlight site to indigo?"

**Expected outputs:**
- Creates a CSS file with `--sl-color-accent-low`, `--sl-color-accent`, and `--sl-color-accent-high` set
- Registers the file in `customCss` array in `astro.config.mjs`
- Does NOT add a raw `<style>` block in a layout component

**Success criteria:**
- Correct variable names (`--sl-color-accent-*`)
- File registered in `customCss`
- Three accent variables provided (low/mid/high for full light+dark coverage)

---

## Scenario 2: Add Tailwind to existing Starlight project

**Prompt:** "I want to add Tailwind CSS to my existing Starlight project."

**Expected outputs:**
1. `npx astro add tailwind`
2. `npm install @astrojs/starlight-tailwind`
3. Replaces `global.css` with correct layer order starting with `@import '@astrojs/starlight-tailwind'`
4. Updates `astro.config.mjs` with `vite: { plugins: [tailwindcss()] }` and registers the CSS file in `customCss`

**Success criteria:**
- Compatibility package installed
- Layer order: `@astrojs/starlight-tailwind` before `tailwindcss/theme.css`
- `customCss` updated to point to the CSS file

---

## Scenario 3: Custom font with Fontsource

**Prompt:** "How do I use the Inter font in my Starlight docs?"

**Expected outputs:**
- `npm install @fontsource/inter`
- `customCss` includes `'@fontsource/inter/400.css'` and `'@fontsource/inter/600.css'`
- CSS sets `--sl-font: 'Inter', sans-serif`

**Success criteria:**
- Installs the font package
- Registers specific weight files (not `@fontsource/inter` — no default export)
- Sets `--sl-font` variable

---

## Scenario 4: Light and dark mode color variants

**Prompt:** "How do I set different background colors for light and dark mode in Starlight?"

**Expected outputs:**
- Uses `:root[data-theme='light']` and `:root[data-theme='dark']` selectors
- Does NOT use `.dark` class selectors
- Registers the CSS in `customCss`

**Success criteria:**
- Correct `data-theme` attribute selectors used
- No `.dark` class selectors
- Changes registered in `customCss`

---

## Scenario 5: Anti-pattern detection — global style tag

**Prompt:** "I added a `<style>` block to my layout with `--sl-color-accent: red` but nothing changed. Why?"

**Expected outputs:**
- Explains that `<style>` blocks outside `customCss` are not included in Starlight's build
- Instructs creating a standalone CSS file and registering it in `customCss`

**Success criteria:**
- Root cause identified (not in `customCss`)
- Correct fix provided (CSS file + `customCss` registration)

---

## Scenario 6: Anti-pattern detection — Tailwind dark mode

**Prompt:** "I installed Tailwind with Starlight but `dark:bg-gray-900` never applies. What's wrong?"

**Expected outputs:**
- Identifies the issue: Starlight uses `data-theme="dark"`, not Tailwind's `.dark` class
- Solution: install `@astrojs/starlight-tailwind` and import it before Tailwind's own CSS
- After installing, Tailwind's `dark:` variant works correctly

**Success criteria:**
- Root cause identified (wrong dark mode mechanism)
- `@astrojs/starlight-tailwind` mentioned as the fix
- No suggestion to manually configure Tailwind's `darkMode` option (the compat package handles it)
