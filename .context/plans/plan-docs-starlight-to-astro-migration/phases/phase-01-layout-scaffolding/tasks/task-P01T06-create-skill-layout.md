# P01T06 — Create `src/layouts/SkillLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/SkillLayout.astro` — a three-column layout (LeftNav +
content + right sidebar) for skill pages and reference sub-pages.

## File to create / modify

```
docs/src/layouts/SkillLayout.astro
```

## Implementation

```astro
---
import type { CollectionEntry } from "astro:content";
import BaseLayout from "./BaseLayout.astro";
import LeftNav from "../components/LeftNav.astro";
import ThemeToggle from "../components/ThemeToggle.astro";
import NavToggle from "../components/NavToggle.astro";
import SkillPageTitle from "../components/SkillPageTitle.astro";
import SkillSidebar from "../components/SkillSidebar.astro";

interface Props {
  entry: CollectionEntry<"docs">;
}

const { entry } = Astro.props;
---

<BaseLayout title={entry.data.title}>
  <header slot="header">
    <NavToggle />
    <a href={`${import.meta.env.BASE_URL}/`}>Tekhne</a>
    <ThemeToggle />
  </header>
  <div class="layout">
    <LeftNav />
    <main data-pagefind-body data-pagefind-meta="type:skill">
      <SkillPageTitle entry={entry} />
      <slot />
    </main>
    <aside class="right-sidebar">
      <SkillSidebar entry={entry} />
    </aside>
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
    grid-template-columns: 16rem 1fr 14rem;
    min-height: calc(100vh - 3.5rem);
  }

  main {
    padding: 2rem;
    overflow: auto;
  }

  .right-sidebar {
    border-left: 1px solid var(--tk-border);
    padding: 1.5rem 1rem;
    overflow: auto;
  }

  @media (max-width: 1024px) {
    .layout {
      grid-template-columns: 16rem 1fr;
    }
    .right-sidebar {
      display: none;
    }
  }

  @media (max-width: 768px) {
    .layout {
      grid-template-columns: 1fr;
    }
  }
</style>
```

## Notes

- `SkillPageTitle` and `SkillSidebar` still consume `Astro.locals.starlightRoute` until Phase 2 task P02T02 refactors them. This layout passes `entry` as a prop — the components will be updated in Phase 2 to accept it.
- `data-pagefind-meta="type:skill"` added here; Phase 4 (P04T04) wires the actual Pagefind integration.

## Verification

```sh
test -f docs/src/layouts/SkillLayout.astro && echo ok
grep "SkillSidebar" docs/src/layouts/SkillLayout.astro
grep "three-column\|grid-template-columns: 16rem" docs/src/layouts/SkillLayout.astro
```
