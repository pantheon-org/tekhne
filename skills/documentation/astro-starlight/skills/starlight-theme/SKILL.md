---
name: starlight-theme
description: Create and apply custom themes to an Astro Starlight documentation site. Use when customizing colors, typography, or spacing via CSS custom properties, using the Tailwind CSS integration, applying light/dark mode variants, or loading custom fonts.
---

# Starlight Theming

Starlight exposes its entire visual design through CSS custom properties. You can override any variable to change colors, typography, and layout — with or without Tailwind.

## When to Use

- Changing the color palette (accent color, grays, backgrounds)
- Swapping fonts or adjusting text sizes
- Applying a consistent brand identity across the docs site
- Integrating Tailwind CSS into a Starlight project

## When Not to Use

- Replacing entire UI sections (header, footer, sidebar) — use `starlight-custom-component` instead
- Per-page layout changes without global theming intent

## Mental Model

Starlight's styles are structured into [CSS cascade layers](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Cascade_layers). Any unlayered CSS you add automatically overrides Starlight's defaults. Custom properties (CSS variables) defined on `:root` propagate through the entire UI, making global theming very predictable.

## Approach 1: Custom CSS File

### Step 1 — Create the CSS file

```css
/* src/styles/custom.css */
:root {
  /* Accent color (links, active nav items) */
  --sl-color-accent-low: #1a1a4e;
  --sl-color-accent: #3d52d5;
  --sl-color-accent-high: #b4bffe;

  /* Typography */
  --sl-font: 'Inter', sans-serif;
  --sl-text-5xl: 3.5rem;
  --sl-content-width: 50rem;
}
```

All available variables are documented in [`props.css` on GitHub](https://github.com/withastro/starlight/blob/main/packages/starlight/style/props.css).

### Step 2 — Register the file in `astro.config.mjs`

```js
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'My Docs',
      customCss: [
        './src/styles/custom.css',
      ],
    }),
  ],
});
```

### Cascade layer ordering (advanced)

If you use `@layer` in your CSS, define the layer order explicitly so your overrides win:

```css
@layer my-reset, starlight, my-overrides;
```

Styles in `my-overrides` take precedence over all of Starlight's layers. Styles in `my-reset` are applied first (lowest priority).

## Approach 2: Tailwind CSS Integration

### New project with Tailwind

```bash
npm create astro@latest -- --template starlight/tailwind
```

### Add Tailwind to an existing Starlight project

**Step 1 — Install Tailwind:**

```bash
npx astro add tailwind
```

**Step 2 — Install Starlight's Tailwind compatibility package:**

```bash
npm install @astrojs/starlight-tailwind
```

**Step 3 — Replace `src/styles/global.css` with:**

```css
@layer base, starlight, theme, components, utilities;

@import '@astrojs/starlight-tailwind';
@import 'tailwindcss/theme.css' layer(theme);
@import 'tailwindcss/utilities.css' layer(utilities);
```

**Step 4 — Update `astro.config.mjs`:**

```js
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  integrations: [
    starlight({
      title: 'Docs with Tailwind',
      customCss: [
        './src/styles/global.css',
      ],
    }),
  ],
  vite: { plugins: [tailwindcss()] },
});
```

### Customizing Starlight via Tailwind theme

Add your brand values in the `@theme` block in `src/styles/global.css`:

```css
@theme {
  --font-sans: 'Inter';
  --font-mono: 'JetBrains Mono';

  /* Accent color scale — Indigo is closest to Starlight's defaults */
  --color-accent-50: var(--color-indigo-50);
  --color-accent-100: var(--color-indigo-100);
  --color-accent-200: var(--color-indigo-200);
  --color-accent-300: var(--color-indigo-300);
  --color-accent-400: var(--color-indigo-400);
  --color-accent-500: var(--color-indigo-500);
  --color-accent-600: var(--color-indigo-600);
  --color-accent-700: var(--color-indigo-700);
  --color-accent-800: var(--color-indigo-800);
  --color-accent-900: var(--color-indigo-900);
  --color-accent-950: var(--color-indigo-950);

  /* Gray scale — Zinc is closest to Starlight's defaults */
  --color-gray-50: var(--color-zinc-50);
  --color-gray-100: var(--color-zinc-100);
  --color-gray-200: var(--color-zinc-200);
  --color-gray-300: var(--color-zinc-300);
  --color-gray-400: var(--color-zinc-400);
  --color-gray-500: var(--color-zinc-500);
  --color-gray-600: var(--color-zinc-600);
  --color-gray-700: var(--color-zinc-700);
  --color-gray-800: var(--color-zinc-800);
  --color-gray-900: var(--color-zinc-900);
  --color-gray-950: var(--color-zinc-950);
}
```

Starlight reads `--color-accent-*` and `--color-gray-*` from your Tailwind theme to style its UI.

## Custom Fonts

### Option A: Local font files

```css
/* src/fonts/font-face.css */
@font-face {
  font-family: 'Custom Font';
  src: url('./CustomFont.woff2') format('woff2');
  font-weight: normal;
  font-style: normal;
  font-display: swap;
}
```

Register in `customCss`:

```js
customCss: ['./src/fonts/font-face.css'],
```

Apply via CSS variable:

```css
:root {
  --sl-font: 'Custom Font', sans-serif;
}
```

### Option B: Fontsource (npm-distributed fonts)

```bash
npm install @fontsource/inter
```

```js
customCss: [
  '@fontsource/inter/400.css',
  '@fontsource/inter/600.css',
],
```

```css
:root {
  --sl-font: 'Inter', sans-serif;
}
```

## Color Theme Editor

Use the interactive [Starlight color theme editor](https://starlight.astro.build/guides/css-and-tailwind/#theming) to generate CSS or Tailwind variables for any color palette. Copy the output directly into your custom CSS or Tailwind `@theme` block.

## Anti-Patterns

### NEVER override Starlight styles without the `customCss` array

**WHY:** Global `<style>` tags or arbitrary CSS files not registered in `customCss` are not included in the Starlight build.

**BAD:** Add a `<style>` block in a layout component to change global colors.
**GOOD:** Register a CSS file in the `customCss` array in `astro.config.mjs`.

**Consequence:** Styles are silently ignored.

### NEVER add raw Tailwind imports without the compatibility layer

**WHY:** Tailwind's default Preflight reset conflicts with Starlight's base styles. The `@astrojs/starlight-tailwind` import reconciles both.

**BAD:** Import `tailwindcss` directly without importing `@astrojs/starlight-tailwind` first.
**GOOD:** Follow the layer order: `@import '@astrojs/starlight-tailwind'` before Tailwind's own imports.

**Consequence:** Visual regressions in Starlight's UI, broken dark mode toggle behavior.

### NEVER use dark-mode class selectors directly

**WHY:** Starlight manages dark mode via its own data attribute (`data-theme="dark"`), not Tailwind's default `.dark` class. The `@astrojs/starlight-tailwind` compatibility layer configures Tailwind's `dark:` variant to match Starlight's mechanism.

**BAD:** Manually write `.dark:bg-gray-900` expecting it to react to Starlight's dark mode toggle.
**GOOD:** Install the Tailwind compatibility package and use `dark:` variants normally.

**Consequence:** Dark mode styles never activate.

### NEVER mix `--sl-*` variables with Tailwind theme customization arbitrarily

**WHY:** Starlight reads either its own CSS custom properties OR Tailwind theme values, depending on which integration is active. Mixing both without understanding the cascade causes unpredictable style conflicts.

**BAD:** Set `--sl-color-accent` in a CSS file while also setting `--color-accent-500` in `@theme`.
**GOOD:** Choose one approach (plain CSS or Tailwind) and apply all overrides consistently through that approach.

**Consequence:** Colors appear differently in light vs. dark mode or only partially apply.

## References

- [CSS & Styling Guide](https://starlight.astro.build/guides/css-and-tailwind/)
- [Customizing Starlight](https://starlight.astro.build/guides/customization/)
- [Starlight CSS variables (props.css)](https://github.com/withastro/starlight/blob/main/packages/starlight/style/props.css)
- [Color Theme Editor](https://starlight.astro.build/guides/css-and-tailwind/#theming)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs/theme)
- [Fontsource](https://fontsource.org/)
