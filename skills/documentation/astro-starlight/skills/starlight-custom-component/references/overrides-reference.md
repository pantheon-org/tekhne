# Starlight Component Overrides Reference

Quick-reference for overridable components and common `starlightRoute` data properties.

## Overridable Components

Full list: [Overrides Reference](https://starlight.astro.build/reference/overrides/)

| Key | Default component | Location in UI |
|-----|------------------|----------------|
| `Banner` | Banner.astro | Announcement banner above header |
| `Header` | Header.astro | Top navigation bar |
| `Footer` | Footer.astro | Page footer |
| `PageTitle` | PageTitle.astro | The `<h1>` page title |
| `Hero` | Hero.astro | Landing page hero (splash pages) |
| `SocialIcons` | SocialIcons.astro | Social links in header |
| `ThemeSelect` | ThemeSelect.astro | Light/dark toggle |
| `Search` | Search.astro | Search component |
| `Sidebar` | Sidebar.astro | Left sidebar |
| `TableOfContents` | TableOfContents.astro | Right table of contents |
| `Pagination` | Pagination.astro | Previous/next page links |
| `LastUpdated` | LastUpdated.astro | Last-updated date |
| `EditLink` | EditLink.astro | Edit page link |
| `PageFrame` | PageFrame.astro | Overall page layout |
| `TwoColumnContent` | TwoColumnContent.astro | Content + TOC layout |

## `starlightRoute` Properties

Access via `Astro.locals.starlightRoute`.

| Property | Type | Description |
|----------|------|-------------|
| `id` | `string` | Page slug (empty for homepage) |
| `slug` | `string` | Alias for `id` |
| `entry.data.title` | `string` | Page title from frontmatter |
| `entry.data.description` | `string \| undefined` | Page description |
| `entry.data.template` | `'doc' \| 'splash'` | Page layout |
| `entry.data.draft` | `boolean \| undefined` | Draft status |
| `entry.data.sidebar.hidden` | `boolean \| undefined` | Hidden from sidebar |
| `sidebar` | `SidebarEntry[]` | Full sidebar tree |
| `toc` | `TocEntry[] \| undefined` | TOC entries |
| `pagination.prev` | `Link \| undefined` | Previous page link |
| `pagination.next` | `Link \| undefined` | Next page link |
| `editUrl` | `URL \| undefined` | Edit page URL |
| `lastUpdated` | `Date \| undefined` | Last updated date |

Full reference: [Route Data Reference](https://starlight.astro.build/reference/route-data/)

## Named Slots in Layout Components

Components that use named slots — must be transferred when wrapping:

### `PageFrame`

```astro
<Default>
  <slot name="header" slot="header" />
  <slot />
  <slot name="footer" slot="footer" />
</Default>
```

### `TwoColumnContent`

```astro
<Default>
  <slot />
  <slot name="right-sidebar" slot="right-sidebar" />
</Default>
```

## Import Path

Default components are importable from:

```js
import Default from '@astrojs/starlight/components/<ComponentName>.astro';
```
