# P05T02 — tiles-page-filters

## Phase

Phase 05 — Registry Enhancements

## Goal

Add client-side domain and grade filter controls to `tiles.astro` so users can narrow the skill catalogue without a page reload, with filter state persisted in the URL query string.

## File to create / modify

```
docs/src/pages/tiles.astro
```

## Implementation

Add a filter bar above the domain groups, and a client-side script that shows/hides `.skill-card` elements based on selected filters:

```astro
---
// (existing getCollection / grouping logic from P02T04)
const grades = ["A", "B+", "B", "C"];
---
<!-- Filter bar -->
<div class="filter-bar" id="filter-bar">
  <label for="filter-domain">Domain</label>
  <select id="filter-domain">
    <option value="">All domains</option>
    {domains.map((d) => <option value={d}>{d}</option>)}
  </select>

  <fieldset class="grade-filters">
    <legend>Grade</legend>
    {grades.map((g) => (
      <label>
        <input type="checkbox" name="grade" value={g} checked />
        {g}
      </label>
    ))}
  </fieldset>
</div>

<script>
const domainSelect = document.getElementById("filter-domain") as HTMLSelectElement;
const gradeInputs = document.querySelectorAll<HTMLInputElement>("input[name=grade]");
const cards = document.querySelectorAll<HTMLElement>(".skill-card");
const sections = document.querySelectorAll<HTMLElement>(".domain-group");

const applyFilters = () => {
  const domain = domainSelect.value;
  const grades = new Set(
    Array.from(gradeInputs).filter((i) => i.checked).map((i) => i.value)
  );
  sections.forEach((section) => {
    const sectionDomain = section.dataset.domain ?? "";
    let visible = 0;
    section.querySelectorAll<HTMLElement>(".skill-card").forEach((card) => {
      const show = (!domain || sectionDomain === domain) &&
                   (!card.dataset.grade || grades.has(card.dataset.grade));
      card.hidden = !show;
      if (show) visible++;
    });
    section.hidden = visible === 0;
  });
  const params = new URLSearchParams();
  if (domain) params.set("domain", domain);
  const activeGrades = Array.from(grades).sort();
  if (activeGrades.length < 4) params.set("grades", activeGrades.join(","));
  history.replaceState(null, "", `?${params.toString()}` || location.pathname);
};

// Restore from URL
const params = new URLSearchParams(location.search);
if (params.has("domain")) domainSelect.value = params.get("domain")!;
if (params.has("grades")) {
  const active = new Set(params.get("grades")!.split(","));
  gradeInputs.forEach((i) => { i.checked = active.has(i.value); });
}

domainSelect.addEventListener("change", applyFilters);
gradeInputs.forEach((i) => i.addEventListener("change", applyFilters));
applyFilters();
</script>
```

Add `data-domain` to each `.domain-group` and `data-grade` to each `.skill-card` in the Astro template.

## Notes

- This is purely client-side filtering with no server round-trip — all skill cards are rendered in the initial HTML and toggled via `hidden`.
- URL state uses `history.replaceState` for shareability without polluting history.
- Grade checkbox defaults to all-checked; the filter only hides cards when specific grades are deselected.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep "filter-bar\|filter-domain\|applyFilters" src/pages/tiles.astro && echo "filters present" || echo "FAIL"
```
