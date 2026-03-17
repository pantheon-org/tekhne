# P05T01 — URL-driven tab state

## Phase

Phase 05 — Registry Enhancements

## Goal

On skill page tab clicks, update the URL with `?tab=skill|audit|evals|refs`
via `history.replaceState`, and on page load read the `?tab` param to activate
the correct tab and dispatch a `sk-tab-change` event.

## File to create / modify

```
src/components/SkillTabs.astro
```

## Implementation

Add to the `<script>` block in `SkillTabs.astro`:

```js
const TABS = ["skill", "audit", "evals", "refs"];

function activateTab(name) {
  TABS.forEach((t) => {
    const btn = document.querySelector(`[data-tab="${t}"]`);
    const panel = document.getElementById(`tab-panel-${t}`);
    if (!btn || !panel) return;
    const active = t === name;
    btn.setAttribute("aria-selected", String(active));
    panel.hidden = !active;
  });
  document.dispatchEvent(new CustomEvent("sk-tab-change", { detail: { tab: name } }));
}

// Activate from URL on load
const initialTab = new URLSearchParams(location.search).get("tab");
if (initialTab && TABS.includes(initialTab)) {
  activateTab(initialTab);
} else {
  activateTab(TABS[0]);
}

// On click, update URL and activate
document.querySelectorAll("[data-tab]").forEach((btn) => {
  btn.addEventListener("click", () => {
    const tab = btn.dataset.tab;
    history.replaceState(null, "", `?tab=${tab}`);
    activateTab(tab);
  });
});
```

## Notes

- Use `replaceState` (not `pushState`) to avoid polluting the browser history
  for tab switches on the same page.
- Tabs must use `role="tab"`, `aria-selected`, and `aria-controls` for
  accessibility.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
grep "replaceState\|sk-tab-change" src/components/SkillTabs.astro && echo "URL state present"
```
