---
name: starlight-base
description: Set up a new Astro Starlight documentation site from scratch. Use when creating a new Starlight project, adding Starlight to an existing Astro project, or configuring core options like sidebar, logo, social links, and page layout.
---

# Starlight Base Setup

Starlight is a full-featured documentation theme built on the [Astro](https://astro.build) framework. This skill guides agents through creating and configuring a Starlight documentation site.

## When to Use

- Creating a new documentation site with Astro and Starlight
- Adding Starlight to an existing Astro project
- Configuring core Starlight options (logo, sidebar, social links, etc.)
- Setting up content pages and navigation

## When Not to Use

- Custom theming with CSS variables or Tailwind — use `starlight-theme` instead
- Overriding built-in Starlight components — use `starlight-custom-component` instead

## Mental Model

Starlight wraps Astro as an integration. The configuration lives in `astro.config.mjs` inside the `starlight({})` call. Content pages are Markdown or MDX files under `src/content/docs/`. File paths map directly to URL routes.

## Quick Start

### New project (recommended)

```bash
npm create astro@latest -- --template starlight
```

```bash
pnpm create astro --template starlight
```

```bash
yarn create astro --template starlight
```

This scaffolds a project with all required files and configuration.

### Start the development server

```bash
npm run dev
```

Open the URL printed in your terminal to preview the site. Changes are reflected instantly.

### Add Starlight to an existing Astro project

```bash
npx astro add starlight
```

This installs `@astrojs/starlight` and adds it to `astro.config.mjs`.

## Project Structure

```
my-docs/
├── astro.config.mjs       # Starlight integration config
├── src/
│   ├── assets/            # Images, logos
│   ├── content/
│   │   └── docs/          # All documentation pages (.md / .mdx)
│   │       ├── index.md   # Homepage
│   │       └── guides/
│   │           └── example.md
│   └── styles/            # Optional custom CSS
└── public/                # Static assets served as-is
```

## Core Configuration

All Starlight options live inside `starlight({})` in `astro.config.mjs`.

### Minimal configuration

```js
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'My Docs',
    }),
  ],
});
```

### Logo

```js
starlight({
  title: 'My Docs',
  logo: {
    src: './src/assets/logo.svg',
  },
});
```

For separate light/dark variants:

```js
logo: {
  light: './src/assets/logo-light.svg',
  dark: './src/assets/logo-dark.svg',
},
```

Set `replacesTitle: true` if the logo already includes the site name (hides the title text while keeping it accessible).

### Social links

```js
social: [
  { icon: 'github', label: 'GitHub', href: 'https://github.com/my-org/repo' },
  { icon: 'discord', label: 'Discord', href: 'https://discord.gg/example' },
],
```

Available icons: `github`, `gitlab`, `discord`, `twitter`, `mastodon`, `linkedin`, `youtube`, `rss`, and more — see the [Icons Reference](https://starlight.astro.build/reference/icons/).

### Sidebar navigation

```js
sidebar: [
  {
    label: 'Guides',
    items: [
      { label: 'Getting Started', slug: 'guides/getting-started' },
      { label: 'Configuration', slug: 'guides/configuration' },
    ],
  },
  {
    label: 'Reference',
    autogenerate: { directory: 'reference' },
  },
],
```

- `items`: explicit list of links.
- `autogenerate`: automatically builds entries from a directory.
- `slug` maps to the file path under `src/content/docs/` (without extension).

### Edit links

```js
editLink: {
  baseUrl: 'https://github.com/my-org/repo/edit/main/docs/',
},
```

Appends an "Edit page" link to every page footer.

### Sitemap

```js
// In defineConfig (not inside starlight):
site: 'https://mydocs.example.com',
```

Starlight generates a sitemap automatically when `site` is set.

## Adding Content Pages

Create `.md` or `.mdx` files under `src/content/docs/`. Each file becomes a page.

### Page frontmatter

```md
---
title: My Page Title
description: A short description for SEO and social sharing.
---

# My Page Title

Content goes here.
```

### Splash (landing) layout

Use `template: splash` for a full-width landing page without sidebars:

```md
---
title: Welcome
template: splash
hero:
  title: My Documentation
  tagline: Fast, beautiful, accessible docs.
  actions:
    - text: Get Started
      link: /guides/getting-started/
      icon: right-arrow
---
```

### Table of contents control

Disable per-page:

```md
---
title: Example
tableOfContents: false
---
```

Control heading levels globally:

```js
starlight({
  tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
});
```

## Keeping Starlight Up to Date

```bash
npx @astrojs/upgrade
```

Run this regularly — Starlight is actively developed and releases frequently.

## Anti-Patterns

### NEVER add content outside `src/content/docs/`

**WHY:** Starlight's file-based routing only picks up files inside `src/content/docs/`. Files elsewhere will not become pages.

**BAD:** Create `.md` files in `src/pages/` expecting them to appear in the docs.
**GOOD:** Create `.md` files in `src/content/docs/`.

**Consequence:** Pages silently won't appear in the site or sidebar.

### NEVER use `src` inside `logo` alongside `light`/`dark`

**WHY:** The two options are mutually exclusive. `light` and `dark` are for variant logos; `src` is for a single logo.

**BAD:** Set both `src` and `light`/`dark` on the `logo` option.
**GOOD:** Use either `src` alone or `light` + `dark` together.

**Consequence:** Configuration errors or unexpected rendering.

### NEVER hard-code sidebar slugs with leading slashes or file extensions

**WHY:** Sidebar `slug` values must match the file path relative to `src/content/docs/` — no leading slash, no `.md` extension.

**BAD:** `slug: '/guides/setup.md'`
**GOOD:** `slug: 'guides/setup'`

**Consequence:** Links in the sidebar break or point to 404 pages.

### NEVER skip setting `site` when you need a sitemap

**WHY:** Starlight's sitemap generation requires the canonical `site` URL set at the `defineConfig` level.

**BAD:** Set `site` inside `starlight({})`.
**GOOD:** Set `site` directly inside `defineConfig({})`.

**Consequence:** Sitemap is not generated.

## References

- [Starlight Getting Started](https://starlight.astro.build/getting-started/)
- [Configuration Reference](https://starlight.astro.build/reference/configuration/)
- [Frontmatter Reference](https://starlight.astro.build/reference/frontmatter/)
- [Project Structure Guide](https://starlight.astro.build/guides/project-structure/)
- [Sidebar Navigation Guide](https://starlight.astro.build/guides/sidebar/)
- [Pages Guide](https://starlight.astro.build/guides/pages/)
