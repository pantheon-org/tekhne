---
name: starlight-base
description: Set up a new Astro Starlight documentation site from scratch. Use when creating a new Starlight project, adding Starlight to an existing Astro project, or configuring core options like sidebar, logo, social links, and page layout.
---

# Starlight Base Setup

Starlight is a full-featured documentation theme built on [Astro](https://astro.build). It ships with routing, search, dark mode, i18n, and accessibility — configure features rather than build them.

## When to Use

- Creating a new documentation site with Astro and Starlight
- Adding Starlight to an existing Astro project
- Configuring core options (logo, sidebar, social links, etc.)
- Setting up content pages and navigation

## When Not to Use

- Custom theming with CSS variables or Tailwind — use `starlight-theme` instead
- Overriding built-in Starlight components — use `starlight-custom-component` instead

## Mental Model

**Starlight = Astro integration + file-based routing + content collections.**

1. **One integration, one config object.** All options live inside `starlight({})` in `astro.config.mjs`. No separate config file.
2. **Files are pages.** Every `.md` / `.mdx` under `src/content/docs/` becomes a URL. File path = URL route.
3. **Sidebar and routing are independent.** You can have a page with no sidebar entry, or a sidebar entry for any slug. They are not coupled unless you use `autogenerate`.

## Quick Start

```bash
npm create astro@latest -- --template starlight
npm run dev
```

To add Starlight to an existing Astro project:

```bash
npx astro add starlight
```

## Project Structure

```
my-docs/
├── astro.config.mjs       # Starlight integration config
├── src/
│   ├── assets/            # Images, logos
│   ├── content/docs/      # All documentation pages (.md / .mdx)
│   │   ├── index.md       # Homepage (route: /)
│   │   └── guides/example.md   # Route: /guides/example/
│   └── styles/            # Optional custom CSS
└── public/                # Static assets served as-is
```

## Core Configuration

All options live inside `starlight({})`. See [configuration-reference.md](./references/configuration-reference.md) for the full option table.

```js
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://mydocs.example.com', // Required for sitemap
  integrations: [
    starlight({
      title: 'My Docs',
      logo: { src: './src/assets/logo.svg' },
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/my-org/repo' },
      ],
      sidebar: [
        {
          label: 'Guides',
          items: [
            { label: 'Getting Started', slug: 'guides/getting-started' },
          ],
        },
        { label: 'Reference', autogenerate: { directory: 'reference' } },
      ],
      editLink: { baseUrl: 'https://github.com/my-org/repo/edit/main/docs/' },
      customCss: ['./src/styles/custom.css'],
    }),
  ],
});
```

## Content Pages

Create `.md` or `.mdx` files under `src/content/docs/`:

```md
---
title: My Page Title
description: A short description for SEO.
---

Content goes here.
```

Use `template: splash` for full-width landing pages. Use `draft: true` to exclude a page from production builds.

## Keeping Up to Date

```bash
npx @astrojs/upgrade
```

## Anti-Patterns

### NEVER add content outside `src/content/docs/`

**WHY:** Starlight's routing only picks up files inside `src/content/docs/`. Files elsewhere will not become pages.

**BAD:** Create `.md` in `src/pages/`.
**GOOD:** Create `.md` in `src/content/docs/`.

**Consequence:** Pages silently won't appear.

### NEVER use `src` inside `logo` alongside `light`/`dark`

**WHY:** Mutually exclusive options. `src` is for a single logo; `light`/`dark` are for variants.

**BAD:** `logo: { src: './logo.svg', light: './light.svg', dark: './dark.svg' }`
**GOOD:** Use either `src` alone or `light` + `dark` together.

**Consequence:** Configuration error or unexpected rendering.

### NEVER hard-code sidebar slugs with leading slashes or file extensions

**WHY:** Slugs map to paths under `src/content/docs/` — no leading slash, no `.md` extension.

**BAD:** `slug: '/guides/setup.md'`
**GOOD:** `slug: 'guides/setup'`

**Consequence:** Sidebar links 404.

### NEVER set `site` inside `starlight({})` for sitemap

**WHY:** Sitemap generation requires `site` at the `defineConfig` level.

**BAD:** `starlight({ site: 'https://...' })`
**GOOD:** `defineConfig({ site: 'https://...' })`

**Consequence:** Sitemap not generated.

### NEVER mix `autogenerate` with `items` in the same sidebar group

**WHY:** A group uses either `items` or `autogenerate`, not both.

**BAD:** `{ label: 'Guides', items: [...], autogenerate: { directory: 'guides' } }`
**GOOD:** Choose one approach per group.

**Consequence:** Build error.

## References

- [Configuration Reference](./references/configuration-reference.md)
- [Starlight Getting Started](https://starlight.astro.build/getting-started/)
- [Configuration Options](https://starlight.astro.build/reference/configuration/)
- [Frontmatter Reference](https://starlight.astro.build/reference/frontmatter/)
- [Sidebar Navigation Guide](https://starlight.astro.build/guides/sidebar/)
- [Pages Guide](https://starlight.astro.build/guides/pages/)
