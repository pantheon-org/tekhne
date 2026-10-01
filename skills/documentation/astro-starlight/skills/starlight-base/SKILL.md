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

## Philosophy

**Starlight = Astro integration + file-based routing + content collections.**

- **One integration, one config object.** All options live inside `starlight({})` in `astro.config.mjs`. No separate config file.
- **Files are pages.** Every `.md` / `.mdx` under `src/content/docs/` becomes a URL. File path = URL route.
- **Sidebar and routing are independent.** You can have a page with no sidebar entry, or a sidebar entry for any slug. They are not coupled unless you use `autogenerate`.
- **Configure, do not build.** Prefer Starlight's built-in features over hand-rolled replacements.

## Quick Start

```bash
npm create astro@latest -- --template starlight
npm run dev
```

To add Starlight to an existing Astro project:

```bash
npx astro add starlight
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

Use `template: splash` for landing pages, `draft: true` to exclude from builds:

```md
---
title: Home
template: splash
hero:
  tagline: Welcome to my docs
---
```

## Anti-Patterns

### NEVER add content outside `src/content/docs/`

**WHY:** Starlight's routing only picks up files inside `src/content/docs/`. **Consequence:** Pages silently won't appear.

**BAD:** Create `.md` in `src/pages/`.

**GOOD:** Create `.md` in `src/content/docs/`.

### NEVER use `src` inside `logo` alongside `light`/`dark`

**WHY:** Mutually exclusive. `src` is for a single logo; `light`/`dark` are for variants. **Consequence:** Config error.

**BAD:** `logo: { src: './logo.svg', light: './light.svg' }`

**GOOD:**

```js
logo: { light: './src/assets/light-logo.svg', dark: './src/assets/dark-logo.svg' }
```

### NEVER hard-code sidebar slugs with leading slashes or file extensions

**WHY:** Slugs map to paths under `src/content/docs/` with no leading slash and no `.md` extension. **Consequence:** Sidebar links 404.

**BAD:** `slug: '/guides/setup.md'`

**GOOD:** `slug: 'guides/setup'`

### NEVER set `site` inside `starlight({})` for sitemap

**WHY:** Sitemap generation requires `site` at the `defineConfig` level. **Consequence:** Sitemap not generated.

**BAD:** `starlight({ site: 'https://...' })`

**GOOD:** `defineConfig({ site: 'https://...' })`

### NEVER mix `autogenerate` with `items` in the same sidebar group

**WHY:** A group uses either `items` or `autogenerate`, not both. **Consequence:** Build error.

**BAD:** `{ label: 'Guides', items: [...], autogenerate: { directory: 'guides' } }`

**GOOD:** Choose one approach per group.

### NEVER put Starlight options in a separate config file

**WHY:** All options live inside `starlight({})` in `astro.config.mjs`. **Consequence:** Options in another file are never read.

**BAD:** Create `starlight.config.mjs` with `title` and `sidebar`.

**GOOD:** Put `title` and `sidebar` inside `starlight({})`.

### NEVER assume a page appears in the sidebar just because the file exists

**WHY:** Sidebar and routing are independent unless you use `autogenerate`. **Consequence:** Reachable pages missing from navigation.

**BAD:** Add `guides/setup.md` and expect a sidebar entry.

**GOOD:** Add a `slug: 'guides/setup'` entry, or use `autogenerate` for the directory.

### NEVER hand-roll the integration when `astro add starlight` exists

**WHY:** For an existing Astro project, `npx astro add starlight` installs the package and updates `astro.config.mjs` for you. **Consequence:** Manual edits risk a half-wired integration.

**BAD:** `npm install @astrojs/starlight` followed by manual config edits.

**GOOD:** `npx astro add starlight`

### NEVER publish a work-in-progress page without `draft: true`

**WHY:** `draft: true` excludes the page from production builds. **Consequence:** Unfinished pages go live.

**BAD:** Commit an unfinished page with only `title` in frontmatter.

**GOOD:** Add `draft: true` to its frontmatter until it is ready.

### NEVER use this skill for theming or component overrides

**WHY:** Those are separate concerns with their own skills. **Consequence:** Mixed guidance and wrong patterns.

**BAD:** Override `Header` here.

**GOOD:** Use `starlight-theme` for CSS variables and Tailwind and `starlight-custom-component` for component overrides.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Configuration options | [Configuration Reference](./references/configuration-reference.md) | Looking up the full option table |
| Getting started | [Starlight Getting Started](https://starlight.astro.build/getting-started/) | Scaffolding a project |
| Configuration | [Configuration Options](https://starlight.astro.build/reference/configuration/) | Checking option semantics |
| Frontmatter | [Frontmatter Reference](https://starlight.astro.build/reference/frontmatter/) | Setting page fields such as `template` and `draft` |
| Sidebar | [Sidebar Navigation Guide](https://starlight.astro.build/guides/sidebar/) | Building navigation |
| Pages | [Pages Guide](https://starlight.astro.build/guides/pages/) | Adding content pages |
