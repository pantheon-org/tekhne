# P05T01 — url-driven-tab-state

## Phase

Phase 05 — Registry Enhancements

## Goal

Add URL-driven tab/section state to skill pages so that the active tab (e.g. Overview, References, Examples) is reflected in the URL hash and is bookmarkable and shareable.

## File to create / modify

```
docs/src/components/TabGroup.astro   (CREATE)
docs/src/layouts/SkillLayout.astro   (integrate TabGroup if applicable)
```

## Implementation

Create a `TabGroup.astro` component that reads the initial tab from `location.hash` and updates the hash on tab change:

```astro
---
interface Props {
  tabs: Array<{ id: string; label: string }>;
}
const { tabs } = Astro.props;
---
<div class="tab-group">
  <ul role="tablist" class="tab-list">
    {tabs.map((tab) => (
      <li role="presentation">
        <button
          id={`tab-${tab.id}`}
          role="tab"
          aria-controls={`panel-${tab.id}`}
          class="tab-btn"
          data-tab={tab.id}
        >
          {tab.label}
        </button>
      </li>
    ))}
  </ul>
  <slot />
</div>

<script>
const tabs = document.querySelectorAll<HTMLButtonElement>("[role=tab]");
const panels = document.querySelectorAll<HTMLElement>("[role=tabpanel]");

const activate = (id: string) => {
  tabs.forEach((t) => t.setAttribute("aria-selected", t.dataset.tab === id ? "true" : "false"));
  panels.forEach((p) => { p.hidden = p.id !== `panel-${id}`; });
  history.replaceState(null, "", `#${id}`);
};

tabs.forEach((t) => t.addEventListener("click", () => activate(t.dataset.tab!)));

const initial = location.hash.slice(1);
const validIds = Array.from(tabs).map((t) => t.dataset.tab!);
activate(validIds.includes(initial) ? initial : validIds[0]);
</script>
```

## Notes

- `history.replaceState` (not `pushState`) avoids polluting browser history with tab changes.
- The initial hash read must happen client-side only; Astro SSG renders with no hash awareness.
- If skill pages do not currently use tabs, this component can be deferred until tab-based content is introduced. The task still creates the component for future use.
- Keyboard navigation: arrow keys should move focus between tabs per ARIA authoring practices (add `keydown` handler for ArrowLeft/ArrowRight).

## Verification

```sh
cd docs
[ -f src/components/TabGroup.astro ] && echo "TabGroup.astro exists" || echo "FAIL"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
```
