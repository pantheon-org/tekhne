# P01T05 — Create `DocsLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/DocsLayout.astro` as a two-column layout (`LeftNav` + content area) for non-skill pages (index, tiles, reference pages).

## File to create / modify

```
docs/src/layouts/DocsLayout.astro
```

## Implementation

```astro
---
import BaseLayout from "./BaseLayout.astro";
import LeftNav from "../components/LeftNav.astro";
import ThemeToggle from "../components/ThemeToggle.astro";
import NavToggle from "../components/NavToggle.astro";

interface Props {
  title: string;
  description?: string;
}
const { title, description } = Astro.props;
---
<BaseLayout title={title} description={description}>
  <Fragment slot="header">
    <nav class="site-header">
      <NavToggle />
      <a href={import.meta.env.BASE_URL} class="site-title">Tekhne</a>
      <ThemeToggle />
    </nav>
  </Fragment>

  <div class="docs-layout">
    <LeftNav currentPath={Astro.url.pathname} />
    <main class="docs-content">
      <slot />
    </main>
  </div>
</BaseLayout>

<style>
  .site-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1.5rem;
    background: var(--tk-bg-surface);
    border-bottom: 1px solid var(--tk-border);
  }

  .site-title {
    font-weight: 600;
    text-decoration: none;
    color: var(--tk-text);
    margin-right: auto;
  }

  .docs-layout {
    display: grid;
    grid-template-columns: 260px 1fr;
    min-height: calc(100vh - 3.5rem);
  }

  .docs-content {
    padding: 2rem;
    max-width: 860px;
    overflow-x: hidden;
  }

  @media (max-width: 768px) {
    .docs-layout {
      grid-template-columns: 1fr;
    }
  }
</style>
```

## Notes

- `LeftNav` receives `currentPath` so it can highlight the active link server-side.
- `import.meta.env.BASE_URL` resolves to `/tekhne/` in production — never hardcode.

## Verification

```sh
test -f docs/src/layouts/DocsLayout.astro
grep 'BASE_URL' docs/src/layouts/DocsLayout.astro
```
