# P05T03 — pagefind-filter-chip

## Phase

Phase 05 — Registry Enhancements

## Goal

Add Pagefind filter metadata (`data-pagefind-filter`) to skill pages so search results can be filtered by domain and grade, then expose those filters as chips in the `Search.astro` component UI.

## File to create / modify

```
docs/src/layouts/SkillLayout.astro   (add data-pagefind-filter attributes)
docs/src/components/Search.astro     (add filter chip UI)
```

## Implementation

**SkillLayout.astro** — add filter metadata to the content wrapper:

```astro
<article
  data-pagefind-body
  data-pagefind-filter={`domain:${entry.data.domain ?? "other"}`}
  data-pagefind-meta={`grade:${entry.data.grade ?? ""}`}
>
  <slot />
</article>
```

**Search.astro** — extend the Pagefind UI to show domain filter chips. The Pagefind JS API allows filtering:

```astro
<div id="search-container">
  <div id="search-filters" class="filter-chips">
    <!-- populated by JS after pagefind initialises -->
  </div>
  <div id="pagefind-ui"></div>
</div>

<script>
import("/pagefind/pagefind-ui.js").then(({ PagefindUI }) => {
  new PagefindUI({
    element: "#pagefind-ui",
    showSubResults: true,
    filters: true,
  });
});
</script>
```

## Notes

- `data-pagefind-filter` is a Pagefind v1 attribute that enables faceted filtering. The format is `"key:value"`.
- Multiple filters on the same element are supported by using separate `data-pagefind-filter` attributes or a JSON array value.
- Pagefind filter UI is built into `PagefindUI` when `filters: true` is passed; custom chip rendering is optional.
- Test with `bunx astro build && bunx pagefind --site dist` — the index must be rebuilt to reflect new filter metadata.

## Verification

```sh
cd docs
bunx astro build 2>&1 | tail -5
grep "data-pagefind-filter" src/layouts/SkillLayout.astro && echo "filter metadata present" || echo "FAIL"
[ -d dist/pagefind ] && echo "pagefind index present" || echo "FAIL: run build first"
```
