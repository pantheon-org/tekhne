---
name: starlight-custom-component
description: Create and override Astro Starlight UI components. Use when replacing or extending built-in Starlight components (header, footer, sidebar, page title, etc.), adding custom UI alongside existing components, accessing page route data inside a component, or applying overrides conditionally on specific pages.
---

# Starlight Custom Components

Starlight's built-in UI can be extended or replaced by registering your own Astro components. Use this when CSS is not enough or you need behavioral changes.

## When to Use

- Changing a Starlight UI element that CSS alone cannot address
- Adding new UI alongside an existing Starlight element (e.g., a banner above the header)
- Replacing a built-in component with different markup or behavior
- Conditionally rendering different UI on specific pages

## When Not to Use

- Simple color or font changes — use `starlight-theme` instead
- Adding documentation content (pages, sidebars) — use `starlight-base` instead

## Mental Model

**Starlight's UI is a set of named slots. You swap slot implementations, not DOM nodes.**

1. **Named slots, not selectors.** You register a replacement `.astro` file for a named component slot in `astro.config.mjs`. No CSS selector is involved — you replace the entire component rendering.
2. **Wrap, don't replace.** Import the default component and render it inside your override to add behavior without duplicating Starlight's markup. This keeps you compatible with future Starlight updates.
3. **Page data lives in `Astro.locals`, not props.** All route metadata is available via `Astro.locals.starlightRoute`. See [overrides-reference.md](./references/overrides-reference.md) for all available properties.

## Step-by-Step: Override a Component

**Step 1 — Identify the slot name** from the [Overrides Reference](https://starlight.astro.build/reference/overrides/). Common ones: `Header`, `Footer`, `PageTitle`, `Sidebar`, `TableOfContents`, `Hero`, `Pagination`. Use the [Overrides Map](https://starlight-overrides-map.netlify.app/) to visually locate UI areas.

**Step 2 — Create the component:**

```astro
---
// src/components/CustomFooter.astro
---
<footer>
  <p>Built with Starlight</p>
</footer>
```

**Step 3 — Register in `astro.config.mjs`:**

```js
starlight({
  components: {
    Footer: './src/components/CustomFooter.astro',
  },
})
```

## Wrapping a Built-in Component

```astro
---
// src/components/CustomSocialIcons.astro
import Default from '@astrojs/starlight/components/SocialIcons.astro';
---
<a href="mailto:hello@example.com">Contact</a>
<Default><slot /></Default>
```

For layout components with named slots (`PageFrame`, `TwoColumnContent`), transfer them explicitly:

```astro
<Default>
  <slot />
  <slot name="right-sidebar" slot="right-sidebar" />
</Default>
```

## Accessing Page Data

```astro
---
const { title, description } = Astro.locals.starlightRoute.entry.data;
const isHomepage = Astro.locals.starlightRoute.id === '';
---
```

See [overrides-reference.md](./references/overrides-reference.md) for the full `starlightRoute` property table.

## Complete Example: Conditional Banner

```astro
---
// src/components/HeaderWithBanner.astro
import Default from '@astrojs/starlight/components/Header.astro';
const showBanner = Astro.locals.starlightRoute.id.startsWith('guides/');
---
{showBanner && (
  <div class="guide-banner">These guides are for version 2.x</div>
)}
<Default><slot /></Default>

<style>
  .guide-banner {
    background: var(--sl-color-accent);
    color: var(--sl-color-white);
    padding: 0.5rem 1rem;
    text-align: center;
  }
</style>
```

## Anti-Patterns

### NEVER register a component path with a leading slash

**WHY:** Paths in `components: {}` are relative to the project root, not absolute.

**BAD:** `Footer: '/src/components/CustomFooter.astro'`
**GOOD:** `Footer: './src/components/CustomFooter.astro'`

**Consequence:** Build error — component not found.

### NEVER omit `<slot />` when wrapping a built-in component

**WHY:** Without `<slot />`, child content is silently discarded.

**BAD:** `<Default />` (no slot)
**GOOD:** `<Default><slot /></Default>`

**Consequence:** Sidebar links and other nested content disappear.

### NEVER access page data via `Astro.props` in override components

**WHY:** Override components do not receive page data as props — only via `Astro.locals`.

**BAD:** `const { title } = Astro.props;` — always `undefined`
**GOOD:** `const { title } = Astro.locals.starlightRoute.entry.data;`

**Consequence:** All page metadata is `undefined`.

### NEVER rely on file name to determine which slot is overridden

**WHY:** Only the key in `components: {}` determines the slot, not the file name.

**BAD:** Naming a file `Footer.astro` without registering it in config.
**GOOD:** `Footer: './src/components/MyFooter.astro'` in config.

**Consequence:** Default component renders unchanged.

### NEVER forget to transfer named slots on layout components

**WHY:** `PageFrame` and `TwoColumnContent` expose named slots. Omitting them silently drops content.

**BAD:** Wrap `TwoColumnContent` with only a default `<slot />`.
**GOOD:** Add `<slot name="right-sidebar" slot="right-sidebar" />` explicitly.

**Consequence:** Right sidebar disappears from all pages.

### NEVER import component internals not listed in the Overrides Reference

**WHY:** Only listed components are stable public API. Others may change in minor releases.

**BAD:** Import an internal component not in the official reference.
**GOOD:** Override only components from [the official list](https://starlight.astro.build/reference/overrides/).

**Consequence:** Imports break on Starlight minor version bumps.

## References

- [Overrides Reference](./references/overrides-reference.md)
- [Overriding Components Guide](https://starlight.astro.build/guides/overriding-components/)
- [Route Data Reference](https://starlight.astro.build/reference/route-data/)
- [Starlight Overrides Map (interactive)](https://starlight-overrides-map.netlify.app/)
- [Astro Components — Slots](https://docs.astro.build/en/basics/astro-components/#slots)
