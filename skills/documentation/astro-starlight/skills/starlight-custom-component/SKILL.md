---
name: starlight-custom-component
description: Create and override Astro Starlight UI components. Use when replacing or extending built-in Starlight components (header, footer, sidebar, page title, etc.), adding custom UI alongside existing components, accessing page route data inside a component, or applying overrides conditionally on specific pages.
---

# Starlight Custom Components

Starlight's built-in UI can be extended or completely replaced by registering your own Astro components. This is the right approach when CSS customization is not enough, or when you need behavioral changes.

## When to Use

- Changing the appearance of a Starlight UI element that cannot be adjusted with CSS alone
- Adding new UI alongside an existing Starlight element (e.g., a banner above the header)
- Replacing a built-in component with entirely different markup or behavior
- Conditionally rendering different UI on specific pages

## When Not to Use

- Simple color or font changes — use `starlight-theme` instead
- Adding documentation content (pages, sidebars, asides) — use `starlight-base` instead

## Mental Model

Starlight's UI is divided into named component slots. You provide a replacement `.astro` file for any slot, and Starlight uses it instead of its default. You can also import and render the original component inside your replacement to wrap (rather than fully replace) it.

The current page's data is always available via `Astro.locals.starlightRoute`.

## Step-by-Step: Override a Component

### Step 1 — Identify the component to override

Find the component name in the [Overrides Reference](https://starlight.astro.build/reference/overrides/). Common ones:

| Component | Renders |
|-----------|---------|
| `Header` | Top navigation bar |
| `Footer` | Page footer |
| `PageTitle` | The `<h1>` page title |
| `SocialIcons` | Social link icons in the header |
| `Sidebar` | Left sidebar navigation |
| `TableOfContents` | Right-side TOC |
| `Hero` | Landing page hero section |
| `Pagination` | Previous/Next page links |

Use the [Starlight Overrides Map](https://starlight-overrides-map.netlify.app/) to visually identify which component covers a given area of the UI.

### Step 2 — Create the Astro component

```astro
---
// src/components/CustomFooter.astro
---
<footer>
  <p>Built with Starlight</p>
</footer>
```

### Step 3 — Register the override in `astro.config.mjs`

```js
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'My Docs',
      components: {
        Footer: './src/components/CustomFooter.astro',
      },
    }),
  ],
});
```

The key must match the exact component name from the Overrides Reference. The value is the path to your `.astro` file relative to the project root.

## Wrapping a Built-in Component

Import the default component and render it alongside your additions. Always include a `<slot />` so child content is passed through correctly.

```astro
---
// src/components/CustomSocialIcons.astro
import Default from '@astrojs/starlight/components/SocialIcons.astro';
---
<a href="mailto:hello@example.com">Contact</a>
<Default><slot /></Default>
```

### Transferring named slots

Some components like `PageFrame` and `TwoColumnContent` use named slots. Transfer them explicitly:

```astro
---
// src/components/CustomContent.astro
import Default from '@astrojs/starlight/components/TwoColumnContent.astro';
---
<Default>
  <slot />
  <slot name="right-sidebar" slot="right-sidebar" />
</Default>
```

## Accessing Page Data

All page metadata is available via `Astro.locals.starlightRoute`.

```astro
---
// src/components/CustomTitle.astro
const { title, description } = Astro.locals.starlightRoute.entry.data;
---
<h1 id="_top">{title}</h1>
{description && <p class="subtitle">{description}</p>}
```

Common properties on `starlightRoute`:

| Property | Type | Description |
|----------|------|-------------|
| `id` | `string` | Page slug (empty string for homepage) |
| `entry.data.title` | `string` | Page title from frontmatter |
| `entry.data.description` | `string \| undefined` | Page description |
| `entry.data.template` | `'doc' \| 'splash'` | Page layout template |
| `sidebar` | `SidebarEntry[]` | Full sidebar tree |
| `toc` | `TocEntry[] \| undefined` | Table of contents entries |

See the full [Route Data Reference](https://starlight.astro.build/reference/route-data/) for all available properties.

## Conditional Overrides

Apply custom UI on specific pages by checking `starlightRoute`:

```astro
---
// src/components/ConditionalBanner.astro
import Default from '@astrojs/starlight/components/Header.astro';

const isHomepage = Astro.locals.starlightRoute.id === '';
---
{isHomepage && (
  <div class="announcement-banner">
    New version available!
  </div>
)}
<Default><slot /></Default>
```

## Complete Example: Custom Header with Banner

```astro
---
// src/components/HeaderWithBanner.astro
import Default from '@astrojs/starlight/components/Header.astro';

const { id } = Astro.locals.starlightRoute;
const showBanner = id.startsWith('guides/');
---
{showBanner && (
  <div class="guide-banner">
    These guides are for version 2.x
  </div>
)}
<Default><slot /></Default>

<style>
  .guide-banner {
    background: var(--sl-color-accent);
    color: var(--sl-color-white);
    padding: 0.5rem 1rem;
    text-align: center;
    font-size: var(--sl-text-sm);
  }
</style>
```

Register in config:

```js
components: {
  Header: './src/components/HeaderWithBanner.astro',
},
```

## Anti-Patterns

### NEVER register a component path with a leading slash

**WHY:** Paths in the `components` config are relative to the project root, not absolute.

**BAD:** `Footer: '/src/components/CustomFooter.astro'`
**GOOD:** `Footer: './src/components/CustomFooter.astro'`

**Consequence:** Starlight fails to find the component and throws a build error.

### NEVER omit `<slot />` when wrapping a built-in component

**WHY:** Without `<slot />`, child content passed to the built-in component is silently discarded.

**BAD:** `<Default />` (no slot)
**GOOD:** `<Default><slot /></Default>`

**Consequence:** Content nested inside the component (e.g., sidebar links) disappears from the rendered output.

### NEVER access page data via props — use `Astro.locals.starlightRoute`

**WHY:** Override components do not receive page data as props. The data is only available via `Astro.locals`.

**BAD:** `const { title } = Astro.props;`
**GOOD:** `const { title } = Astro.locals.starlightRoute.entry.data;`

**Consequence:** `title` (and all other page data) is `undefined`.

### NEVER use the component `name` from Overrides Reference as the file name

**WHY:** The file name of your `.astro` component is irrelevant; only the key in `components: {}` determines which slot is overridden.

**BAD:** Assume that naming your file `Footer.astro` automatically overrides the Footer slot.
**GOOD:** Explicitly set `Footer: './src/components/MyFooter.astro'` in the config.

**Consequence:** Default component renders unchanged.

### NEVER forget to transfer named slots on layout components

**WHY:** `PageFrame` and `TwoColumnContent` expose named slots. If you wrap these and omit the named slot transfers, the content of those named slots (like the right sidebar) is lost.

**BAD:** Wrap `TwoColumnContent` with only a default `<slot />`.
**GOOD:** Transfer all named slots: `<slot name="right-sidebar" slot="right-sidebar" />`.

**Consequence:** The right sidebar disappears from all pages using that layout.

## References

- [Overriding Components Guide](https://starlight.astro.build/guides/overriding-components/)
- [Overrides Reference](https://starlight.astro.build/reference/overrides/)
- [Route Data Reference](https://starlight.astro.build/reference/route-data/)
- [Starlight Overrides Map (interactive)](https://starlight-overrides-map.netlify.app/)
- [Astro Components — Slots](https://docs.astro.build/en/basics/astro-components/#slots)
