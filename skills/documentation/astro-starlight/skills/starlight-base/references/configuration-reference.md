# Starlight Configuration Reference

Quick-reference for the most commonly used `starlight({})` options in `astro.config.mjs`.

## Top-Level Options

| Option | Type | Description |
|--------|------|-------------|
| `title` | `string` | **Required.** Site title shown in header and page titles. |
| `description` | `string` | Default meta description for all pages. |
| `logo` | `LogoConfig` | Header logo. See Logo section below. |
| `social` | `SocialLink[]` | Social media links in the header. |
| `sidebar` | `SidebarItem[]` | Sidebar navigation configuration. |
| `editLink` | `EditLinkConfig` | "Edit page" link configuration. |
| `customCss` | `string[]` | CSS files to inject. Paths relative to project root. |
| `components` | `ComponentsConfig` | Override built-in UI components. |
| `head` | `HeadConfig[]` | Additional HTML elements to inject into `<head>`. |
| `favicon` | `string` | Path to favicon (relative to `public/`). |
| `tableOfContents` | `TableOfContentsConfig \| false` | Global TOC configuration. |
| `lastUpdated` | `boolean` | Show last-updated date on pages. |
| `pagination` | `boolean` | Show previous/next page links. |
| `defaultLocale` | `string` | Default locale for i18n. |

## Logo Options

```ts
type LogoConfig =
  | { src: string; replacesTitle?: boolean; alt?: string }
  | { light: string; dark: string; replacesTitle?: boolean; alt?: string }
```

`src` and `light`/`dark` are mutually exclusive.

## Sidebar Item Types

```ts
type SidebarItem =
  | { label: string; slug: string }                              // Single link
  | { label: string; items: SidebarItem[]; collapsed?: boolean } // Group with explicit items
  | { label: string; autogenerate: { directory: string }; collapsed?: boolean } // Auto group
```

A group cannot have both `items` and `autogenerate`.

## Social Link Icons

Full list from [Starlight Icons Reference](https://starlight.astro.build/reference/icons/):
`github`, `gitlab`, `discord`, `twitter`, `mastodon`, `linkedin`, `youtube`, `rss`, `email`, `external`, `seti`, `x.com`

## defineConfig Options (not in starlight({}))

| Option | Description |
|--------|-------------|
| `site` | Canonical URL — required for sitemap generation |
| `base` | Base path for deployment to a subdirectory |

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

## Upgrading

```bash
npx @astrojs/upgrade
```
