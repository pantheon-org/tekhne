# P02T11 — fix-skill-sidebar-mobile

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Rewrite `docs/src/components/SkillSidebar.astro` to remove all `virtual:starlight/*` imports and replace the mobile-sidebar pattern with a native `<details>`/`<summary>` drawer that works without Starlight's JS runtime.

## File to create / modify

```
docs/src/components/SkillSidebar.astro
```

## Implementation

```astro
---
import type { CollectionEntry } from "astro:content";

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;

const { title, grade, tilePublishedUrl, tileVersion, description } = entry.data;
---
<aside class="skill-sidebar" aria-label="Skill details">
  <details class="sidebar-mobile-drawer" open>
    <summary class="sidebar-toggle">Skill Info</summary>
    <div class="sidebar-content">
      {grade && (
        <div class="sidebar-section">
          <span class="sidebar-label">Grade</span>
          <span class={`badge grade-${grade.toLowerCase().replace("+", "-plus")}`}>{grade}</span>
        </div>
      )}

      {tilePublishedUrl && (
        <div class="sidebar-section">
          <span class="sidebar-label">Registry</span>
          <a href={tilePublishedUrl} target="_blank" rel="noopener noreferrer" class="sidebar-link">
            View on Tessl
            {tileVersion && <span class="sidebar-version">v{tileVersion}</span>}
          </a>
        </div>
      )}

      {description && (
        <div class="sidebar-section">
          <span class="sidebar-label">About</span>
          <p class="sidebar-description">{description}</p>
        </div>
      )}
    </div>
  </details>
</aside>
```

## Notes

- `virtual:starlight/components` and `virtual:starlight/user-data` are Starlight-internal virtual modules that will not exist after P02T07; any remaining imports will cause a Vite build error.
- The `<details open>` pattern provides a native mobile drawer with zero JS: open by default on desktop (overridden via CSS `@media`), collapsible on mobile.
- CSS for `.skill-sidebar` responsive behaviour is added in Phase 4 (polish); this task only removes the Starlight dependency and establishes the markup skeleton.
- `data-pagefind-ignore` should be added to `<aside>` if Pagefind indexes sidebar content in false-positive search hits (evaluate in Phase 4).

## Verification

```sh
cd docs
grep -r "virtual:starlight" src/components/SkillSidebar.astro && echo "FAIL: virtual:starlight import remains" || echo "ok"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
```
