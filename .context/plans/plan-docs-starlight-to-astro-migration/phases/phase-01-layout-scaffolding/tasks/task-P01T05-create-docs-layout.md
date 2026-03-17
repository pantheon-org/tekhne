# P01T05 — Create `src/layouts/DocsLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/DocsLayout.astro` — a two-column layout (LeftNav + content
area) for non-skill pages, the index page, and the tiles page.

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
  <header slot="header">
    <NavToggle />
    <a href={`${import.meta.env.BASE_URL}/`}>Tekhne</a>
    <ThemeToggle />
  </header>
  <div class="layout">
    <LeftNav />
    <main>
      <slot />
    </main>
  </div>
</BaseLayout>

<style>
  header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--tk-border);
    background: var(--tk-bg-surface);
  }

  .layout {
    display: grid;
    grid-template-columns: 16rem 1fr;
    min-height: calc(100vh - 3.5rem);
  }

  main {
    padding: 2rem;
    overflow: auto;
  }

  @media (max-width: 768px) {
    .layout {
      grid-template-columns: 1fr;
    }
  }
</style>
```

## Notes

- `import.meta.env.BASE_URL` resolves to `/tekhne/` at build time, respecting the `base` config.
- `LeftNav` is imported but not yet wired to a real collection query until Phase 2 routes exist — during Phase 1 it renders the nav tree independently of any route.

## Verification

```sh
test -f docs/src/layouts/DocsLayout.astro && echo ok
grep "LeftNav" docs/src/layouts/DocsLayout.astro
```
