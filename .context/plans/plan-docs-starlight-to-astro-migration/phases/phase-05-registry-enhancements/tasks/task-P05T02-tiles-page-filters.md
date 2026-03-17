# P05T02 — Tiles page filters

## Phase

Phase 05 — Registry Enhancements

## Goal

Add client-side domain filter chips and grade filter chips to the tiles index
page, with a skill count badge per domain group that updates as filters change.

## File to create / modify

```
src/pages/docs/tiles/index.astro
src/components/FilterChips.astro   (new)
```

## Implementation

### 1. FilterChips.astro

```astro
---
interface Props {
  label: string;
  values: string[];
  filterAttr: string;
}
const { label, values, filterAttr } = Astro.props;
---

<div class="filter-group" data-filter-attr={filterAttr}>
  <span class="filter-label">{label}</span>
  <button class="chip active" data-value="all">All</button>
  {values.map((v) => (
    <button class="chip" data-value={v}>{v}</button>
  ))}
</div>

<script>
  document.querySelectorAll(".filter-group").forEach((group) => {
    const attr = group.dataset.filterAttr;
    group.querySelectorAll(".chip").forEach((chip) => {
      chip.addEventListener("click", () => {
        group.querySelectorAll(".chip").forEach((c) => c.classList.remove("active"));
        chip.classList.add("active");
        const value = chip.dataset.value;
        document.querySelectorAll("[data-tile-card]").forEach((card) => {
          const match = value === "all" || card.dataset[attr] === value;
          card.hidden = !match;
        });
        // update counts
        document.querySelectorAll("[data-domain-count]").forEach((badge) => {
          const domain = badge.dataset.domainCount;
          const visible = document.querySelectorAll(
            `[data-tile-card][data-domain="${domain}"]:not([hidden])`
          ).length;
          badge.textContent = String(visible);
        });
      });
    });
  });
</script>
```

### 2. Wire into tiles index page

In `src/pages/docs/tiles/index.astro`:
- Derive unique `domains` and `grades` from the tiles collection.
- Render `<FilterChips label="Domain" values={domains} filterAttr="domain" />`.
- Render `<FilterChips label="Grade" values={grades} filterAttr="grade" />`.
- Add `data-tile-card`, `data-domain`, and `data-grade` attributes to each card.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f src/components/FilterChips.astro && echo "FilterChips component exists"
grep "data-tile-card" src/pages/docs/tiles/index.astro && echo "card attrs present"
```
