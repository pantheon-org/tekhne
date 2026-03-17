# P01T06 — Create `SkillLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/SkillLayout.astro` as a three-column layout (`LeftNav` + content + right sidebar) for skill pages and their reference sub-pages, with an `entry` prop passed through to child components.

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
import SkillTabs from "../components/SkillTabs.astro";
import SkillSidebar from "../components/SkillSidebar.astro";

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;
---
<BaseLayout title={entry.data.title}>
  <Fragment slot="header">
    <nav class="site-header">
      <NavToggle />
      <a href={import.meta.env.BASE_URL} class="site-title">Tekhne</a>
      <ThemeToggle />
    </nav>
  </Fragment>

  <div class="skill-layout">
    <LeftNav currentPath={Astro.url.pathname} />
    <main class="skill-content" data-pagefind-body data-pagefind-meta="type:skill">
      <SkillPageTitle entry={entry} />
      <SkillTabs entry={entry}>
        <slot />
      </SkillTabs>
    </main>
    <aside class="skill-sidebar">
      <SkillSidebar entry={entry} />
    </aside>
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

  .skill-layout {
    display: grid;
    grid-template-columns: 260px 1fr 220px;
    min-height: calc(100vh - 3.5rem);
  }

  .skill-content {
    padding: 2rem;
    overflow-x: hidden;
  }

  .skill-sidebar {
    border-left: 1px solid var(--tk-border);
    padding: 1.5rem 1rem;
    overflow-y: auto;
  }

  @media (max-width: 1024px) {
    .skill-layout {
      grid-template-columns: 260px 1fr;
    }
    .skill-sidebar {
      display: none;
    }
  }

  @media (max-width: 768px) {
    .skill-layout {
      grid-template-columns: 1fr;
    }
  }
</style>
```

## Notes

- The `entry` prop flows through to `SkillPageTitle`, `SkillTabs`, and `SkillSidebar` — all three must accept `entry: CollectionEntry<"docs">` as a prop (done in Phase 2 P02T02).
- `data-pagefind-meta="type:skill"` on `<main>` enables Pagefind skill filtering (Phase 4).

## Verification

```sh
test -f docs/src/layouts/SkillLayout.astro
grep 'CollectionEntry' docs/src/layouts/SkillLayout.astro
```
