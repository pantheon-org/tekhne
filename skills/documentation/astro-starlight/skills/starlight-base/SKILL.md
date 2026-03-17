---
name: starlight-base
description: Set up a new Astro Starlight documentation site from scratch. Use when creating a new Starlight project, adding Starlight to an existing Astro project, or configuring core options like sidebar, logo, social links, and page layout.
---

# Starlight Base Setup

Starlight is a full-featured documentation theme built on the [Astro](https://astro.build) framework. It ships with routing, search, dark mode, i18n, and accessibility out of the box — so you configure features rather than build them.

## When to Use

- Creating a new documentation site with Astro and Starlight
- Adding Starlight to an existing Astro project
- Configuring core Starlight options (logo, sidebar, social links, etc.)
- Setting up content pages and navigation

## When Not to Use

- Custom theming with CSS variables or Tailwind — use `starlight-theme` instead
- Overriding built-in Starlight components — use `starlight-custom-component` instead

## Mental Model

**Starlight = Astro integration + file-based routing + content collections.**

Three things to internalize:

1. **One integration, one config object.** All Starlight options live inside `starlight({})` in `astro.config.mjs`. There is no separate config file.

2. **Files are pages.** Every `.md` or `.mdx` file under `src/content/docs/` becomes a page. The file path maps 1:1 to the URL route. Moving or renaming a file changes its URL.

3. **Sidebar and routing are independent.** You can have a page with no sidebar entry (it's accessible by URL but unlisted), or a sidebar entry pointing to any slug. They are not coupled automatically unless you use `autogenerate`.

## Quick Start

### New project

```bash
npm create astro@latest -- --template starlight
# or
pnpm create astro --template starlight
# or
yarn create astro --template starlight
```

This scaffolds a project with all required files and configuration.

```bash
npm run dev
```

Open the URL printed in your terminal. Changes are reflected instantly.

### Add to an existing Astro project

```bash
npx astro add starlight
```

This installs `@astrojs/starlight` and updates `astro.config.mjs` automatically.

## Project Structure

```
my-docs/
├── astro.config.mjs       # Starlight integration config
├── src/
│   ├── assets/            # Images, logos
│   ├── content/
│   │   └── docs/          # All documentation pages (.md / .mdx)
│   │       ├── index.md   # Homepage (route: /)
│   │       └── guides/
│   │           └── example.md   # Route: /guides/example/
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

Use a single image for all modes:

```js
logo: {
  src: './src/assets/logo.svg',
},
```

Use separate light/dark variants:

```js
logo: {
  light: './src/assets/logo-light.svg',
  dark: './src/assets/logo-dark.svg',
},
```

Set `replacesTitle: true` when the logo already includes the site name. This hides the visible title text while keeping it in the DOM for screen readers.

```js
logo: {
  src: './src/assets/logo-with-text.svg',
  replacesTitle: true,
},
```

### Social links

```js
social: [
  { icon: 'github', label: 'GitHub', href: 'https://github.com/my-org/repo' },
  { icon: 'discord', label: 'Discord', href: 'https://discord.gg/example' },
],
```

Available icons: `github`, `gitlab`, `discord`, `twitter`, `mastodon`, `linkedin`, `youtube`, `rss`, and more. See the [Icons Reference](https://starlight.astro.build/reference/icons/).

### Sidebar navigation

Starlight supports two sidebar entry types — explicit items and auto-generated directories:

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

- `items`: explicit list. Use when you need custom ordering or labeling.
- `autogenerate`: builds entries automatically from a directory. Use when you want the sidebar to stay in sync with the file system.
- `slug` values are file paths under `src/content/docs/` — no leading slash, no extension.

### Collapsed groups

```js
{
  label: 'Advanced',
  collapsed: true,
  items: [ /* ... */ ],
},
```

### Edit links

```js
editLink: {
  baseUrl: 'https://github.com/my-org/repo/edit/main/docs/',
},
```

Appends an "Edit page" link to every page footer, pointing to the source file on GitHub.

### Sitemap

The `site` option belongs on `defineConfig`, not inside `starlight({})`:

```js
export default defineConfig({
  site: 'https://mydocs.example.com',
  integrations: [starlight({ title: 'My Docs' })],
});
```

Starlight generates `sitemap-index.xml` and `sitemap-0.xml` automatically when `site` is set.

## Content Pages

Create `.md` or `.mdx` files under `src/content/docs/`. Each file becomes a page.

### Minimal frontmatter

```md
---
title: My Page Title
description: A short description for SEO and social sharing.
---

Content goes here.
```

`title` is required. `description` is used in `<meta>` tags and OpenGraph cards.

### Landing page layout

Use `template: splash` for a full-width hero page without sidebars:

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
    - text: GitHub
      link: https://github.com/my-org/repo
      icon: external
      variant: minimal
---
```

### Table of contents control

Disable TOC on a specific page:

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

### Draft pages

Mark a page as a draft to exclude it from production builds:

```md
---
title: Unreleased Feature
draft: true
---
```

Draft pages are still accessible in development but are omitted from the production build and sitemap.

## Advanced Configuration

### Custom 404 page

Create `src/content/docs/404.md` with `template: splash` to add a branded 404 page:

```md
---
title: Page Not Found
template: splash
---

The page you were looking for doesn't exist.
```

### Head injection

Add global `<meta>`, `<link>`, or `<script>` tags:

```js
starlight({
  head: [
    {
      tag: 'meta',
      attrs: { name: 'google-site-verification', content: 'TOKEN' },
    },
    {
      tag: 'script',
      attrs: { src: 'https://analytics.example.com/script.js', defer: true },
    },
  ],
});
```

### Favicon

```js
starlight({
  favicon: '/favicon.ico',
});
```

Place `favicon.ico` in the `public/` directory.

## Keeping Starlight Up to Date

```bash
npx @astrojs/upgrade
```

Run this regularly. Starlight is actively developed with frequent releases.

## Anti-Patterns

### NEVER add content outside `src/content/docs/`

**WHY:** Starlight's file-based routing only picks up files inside `src/content/docs/`. Files elsewhere will not become pages.

**BAD:** Create `.md` files in `src/pages/` expecting them to appear in the docs.
**GOOD:** Create `.md` files in `src/content/docs/`.

**Consequence:** Pages silently won't appear in the site or sidebar.

### NEVER use `src` inside `logo` alongside `light`/`dark`

**WHY:** The two options are mutually exclusive. `light` and `dark` are for variant logos; `src` is for a single logo. Providing both causes a configuration error.

**BAD:** `logo: { src: './logo.svg', light: './light.svg', dark: './dark.svg' }`
**GOOD:** Use either `src` alone or `light` + `dark` together.

**Consequence:** Configuration errors or unexpected rendering.

### NEVER hard-code sidebar slugs with leading slashes or file extensions

**WHY:** Sidebar `slug` values must match the file path relative to `src/content/docs/` without a leading slash or extension.

**BAD:** `slug: '/guides/setup.md'`
**GOOD:** `slug: 'guides/setup'`

**Consequence:** Links in the sidebar break or point to 404 pages.

### NEVER skip setting `site` when you need a sitemap

**WHY:** Starlight's sitemap generation requires the canonical `site` URL set at the `defineConfig` level, not inside `starlight({})`.

**BAD:** `starlight({ site: 'https://...' })`
**GOOD:** `defineConfig({ site: 'https://...' })`

**Consequence:** Sitemap is not generated.

### NEVER mix `autogenerate` with manual `items` in the same group

**WHY:** A sidebar group can use either `items` or `autogenerate`, not both. Providing both causes Starlight to throw a configuration error.

**BAD:** `{ label: 'Guides', items: [...], autogenerate: { directory: 'guides' } }`
**GOOD:** Choose one approach per group.

**Consequence:** Build error and no sidebar rendered.

## References

- [Starlight Getting Started](https://starlight.astro.build/getting-started/)
- [Configuration Reference](https://starlight.astro.build/reference/configuration/)
- [Frontmatter Reference](https://starlight.astro.build/reference/frontmatter/)
- [Project Structure Guide](https://starlight.astro.build/guides/project-structure/)
- [Sidebar Navigation Guide](https://starlight.astro.build/guides/sidebar/)
- [Pages Guide](https://starlight.astro.build/guides/pages/)
