---
name: starlight-theme
description: Create and apply custom themes to an Astro Starlight documentation site. Use when customizing colors, typography, or spacing via CSS custom properties, using the Tailwind CSS integration, applying light/dark mode variants, or loading custom fonts.
---

# Starlight Theming

Starlight exposes its entire visual design through CSS custom properties. Override any variable to change colors, typography, and layout — with or without Tailwind.

## When to Use

- Changing the color palette (accent color, grays, backgrounds)
- Swapping fonts or adjusting text sizes
- Applying a consistent brand identity
- Integrating Tailwind CSS into a Starlight project

## When Not to Use

- Replacing entire UI sections (header, footer, sidebar) — use `starlight-custom-component` instead
- Per-page layout changes without global theming intent

## Philosophy

**Starlight styles live in CSS cascade layers. Unlayered CSS always wins.**

- **CSS custom properties are the API.** Set `--sl-*` variables on `:root` to change every element that reads them. See [css-variables-reference.md](./references/css-variables-reference.md) for the full list.
- **Cascade layers determine priority.** Any CSS you add without an `@layer` block is unlayered and automatically takes precedence over Starlight's named layers.
- **Tailwind requires different wiring.** The `@astrojs/starlight-tailwind` compatibility package must be imported before Tailwind's own CSS or Starlight's styles break.

## Approach 1: Custom CSS File

**Step 1 — Create the file:**

```css
/* src/styles/custom.css */
:root {
  --sl-color-accent-low: #1a1a4e;
  --sl-color-accent: #3d52d5;
  --sl-color-accent-high: #b4bffe;
  --sl-font: 'Inter', sans-serif;
  --sl-content-width: 50rem;
}

:root[data-theme='dark'] {
  --sl-color-bg: #0f172a;
}
```

**Step 2 — Register in `astro.config.mjs`:**

```js
starlight({
  customCss: ['./src/styles/custom.css'],
})
```

## Approach 2: Tailwind CSS Integration

```bash
npx astro add tailwind
npm install @astrojs/starlight-tailwind
```

Replace `src/styles/global.css` with:

```css
@layer base, starlight, theme, components, utilities;

@import '@astrojs/starlight-tailwind';
@import 'tailwindcss/theme.css' layer(theme);
@import 'tailwindcss/utilities.css' layer(utilities);

@theme {
  --font-sans: 'Inter';
  --color-accent-500: var(--color-indigo-500);
  /* Map full accent and gray scales — see references for complete example */
}
```

Update `astro.config.mjs`:

```js
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  integrations: [starlight({ customCss: ['./src/styles/global.css'] })],
  vite: { plugins: [tailwindcss()] },
});
```

See [css-variables-reference.md](./references/css-variables-reference.md) for the full Tailwind `@theme` color scale mapping.

## Anti-Patterns

### NEVER override Starlight styles without the `customCss` array

**WHY:** CSS files not registered in `customCss` are excluded from the Starlight build. **Consequence:** Styles are silently ignored.

**BAD:** Add a `<style>` block in a layout component.

**GOOD:** Register a CSS file in `customCss` in `astro.config.mjs`.

### NEVER add raw Tailwind imports without the compatibility layer

**WHY:** Tailwind's Preflight reset conflicts with Starlight's base styles. **Consequence:** Visual regressions, broken dark mode toggle.

**BAD:** Import `tailwindcss` without `@astrojs/starlight-tailwind` first.

**GOOD:** `@import '@astrojs/starlight-tailwind'` before Tailwind imports.

### NEVER use `.dark` class selectors for dark mode

**WHY:** Starlight uses `data-theme="dark"` on `<html>`, not a `.dark` class. **Consequence:** Dark mode styles never activate.

**BAD:** `.dark:bg-gray-900` (never matches in Starlight)

**GOOD** (plain CSS):

```css
:root[data-theme='dark'] { --sl-color-bg: #0f172a; }
```

Use `dark:` Tailwind variants only after installing `@astrojs/starlight-tailwind`.

### NEVER mix `--sl-*` variables with Tailwind `@theme` overrides

**WHY:** The two systems map to each other through `@astrojs/starlight-tailwind`. Setting both creates conflicting sources of truth. **Consequence:** Colors partially apply or behave differently in light vs. dark mode.

**BAD:** Set `--sl-color-accent` in CSS while also defining `--color-accent-*` in `@theme`.

**GOOD:** Choose one approach and apply all overrides through it.

### NEVER forget `font-display: swap` in custom `@font-face`

**WHY:** Without it, browsers may block rendering until the font downloads. **Consequence:** Flash of invisible text (FOIT) on slower connections.

**BAD:** `@font-face { font-family: 'X'; src: url('...'); }`

**GOOD:** Add `font-display: swap;` to every `@font-face`.

### NEVER wrap your overrides in a named layer that Starlight's layers outrank

**WHY:** Unlayered CSS always takes precedence over Starlight's named layers; a lower-priority layer can lose to Starlight's own rules. **Consequence:** Overrides silently lose.

**BAD:** `@layer starlight { :root { --sl-color-accent: #3d52d5; } }`

**GOOD:** Put the `:root` overrides in plain CSS with no `@layer` block.

### NEVER drop the layer order declaration in the Tailwind setup

**WHY:** `@layer base, starlight, theme, components, utilities;` fixes the cascade between Tailwind and Starlight. **Consequence:** Utilities and Starlight styles override each other unpredictably.

**BAD:** Start `global.css` with the Tailwind imports only.

**GOOD:** Start `global.css` with the `@layer` order line, then the imports.

### NEVER set a single accent variable and expect a full palette

**WHY:** Accent colours are a low, mid and high set. **Consequence:** Backgrounds, links and highlights mismatch in light and dark mode.

**BAD:** `:root { --sl-color-accent: #3d52d5; }` alone.

**GOOD:** Set `--sl-color-accent-low`, `--sl-color-accent` and `--sl-color-accent-high` together.

### NEVER use this skill to replace UI sections such as header, footer or sidebar

**WHY:** Replacing markup needs component overrides, which CSS variables cannot do. **Consequence:** Fragile selectors that break on Starlight updates.

**BAD:** Hide the sidebar with CSS and rebuild it by hand.

**GOOD:** Use `starlight-custom-component` to override the component.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| CSS variables | [CSS Variables Reference](./references/css-variables-reference.md) | Looking up `--sl-*` variables and the Tailwind `@theme` scale mapping |
| Styling guide | [CSS & Styling Guide](https://starlight.astro.build/guides/css-and-tailwind/) | Learning the official theming flow |
| Source variables | [Starlight CSS variables (props.css)](https://github.com/withastro/starlight/blob/main/packages/starlight/style/props.css) | Checking the full variable list |
| Colour editor | [Color Theme Editor](https://starlight.astro.build/guides/css-and-tailwind/#theming) | Generating accent and gray scales |
| Fonts | [Fontsource](https://fontsource.org/) | Self-hosting custom fonts |
