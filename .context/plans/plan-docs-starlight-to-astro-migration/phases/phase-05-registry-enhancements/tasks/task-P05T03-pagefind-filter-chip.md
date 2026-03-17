# P05T03 — Pagefind filter chip

## Phase

Phase 05 — Registry Enhancements

## Goal

Add a filter chip to `SearchBar.astro` that toggles the Pagefind UI between
showing all results and showing only skill pages (`type:skill`).

## File to create / modify

```
src/components/SearchBar.astro
```

## Implementation

Extend `SearchBar.astro` to add a filter chip and pass the active filter to the
Pagefind UI `filters` option:

```astro
---
---
<div class="search-wrapper">
  <div id="search" class="search-bar"></div>
  <div class="search-filter">
    <button id="filter-all" class="chip active">All</button>
    <button id="filter-skills" class="chip">Skills only</button>
  </div>
</div>

<link rel="stylesheet" href="/pagefind/pagefind-ui.css" />
<script src="/pagefind/pagefind-ui.js" is:inline></script>
<script is:inline>
  let pf;
  window.addEventListener("DOMContentLoaded", () => {
    pf = new PagefindUI({ element: "#search", showSubResults: true });

    document.getElementById("filter-all").addEventListener("click", () => {
      pf.triggerSearch("", {});
      setActive("all");
    });

    document.getElementById("filter-skills").addEventListener("click", () => {
      pf.triggerSearch("", { filters: { type: "skill" } });
      setActive("skills");
    });

    function setActive(id) {
      document.querySelectorAll(".search-filter .chip").forEach((c) =>
        c.classList.toggle("active", c.id === `filter-${id}`)
      );
    }
  });
</script>
```

## Notes

- Pagefind filters require the `data-pagefind-meta` attributes added in P04T04.
- The Pagefind JS bundle is only present after `bun run build` — the filter
  chip is hidden via CSS in dev mode if `pagefind-ui.js` does not load.

## Verification

```sh
bunx astro build && npx pagefind --source dist 2>&1 | tail -3
test -f dist/pagefind/pagefind.js && echo "index present"
grep "filter-skills\|type.*skill" src/components/SearchBar.astro && echo "filter chip present"
```
