# P04T06 — add-breadcrumb

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Create `docs/src/components/Breadcrumb.astro` and wire it into `DocsLayout.astro` and `SkillLayout.astro` to display the skill's path hierarchy (e.g. `Home > ci-cd > github-actions > generator`) above the page title, matching Starlight's breadcrumb behaviour.

## File to create / modify

```
docs/src/components/Breadcrumb.astro   (CREATE)
docs/src/layouts/DocsLayout.astro      (add Breadcrumb)
docs/src/layouts/SkillLayout.astro     (add Breadcrumb)
```

## Implementation

```astro
---
import type { CollectionEntry } from "astro:content";

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;
const BASE = import.meta.env.BASE_URL.replace(/\/$/, "");

// Build breadcrumb segments from entry.id path
// e.g. "skills/ci-cd/github-actions/generator/index" -> ["skills","ci-cd","github-actions","generator"]
const parts = entry.id
  .replace(/\/index$/, "")
  .replace(/\.mdx?$/, "")
  .split("/")
  .filter(Boolean);

// Build cumulative hrefs
const crumbs = parts.map((part, i) => ({
  label: part.replace(/-/g, " "),
  href: `${BASE}/${parts.slice(0, i + 1).join("/")}`,
}));
---
<nav aria-label="breadcrumb" class="breadcrumb">
  <ol>
    <li><a href={`${BASE}/`}>Home</a></li>
    {crumbs.map((crumb, i) => (
      <li aria-current={i === crumbs.length - 1 ? "page" : undefined}>
        {i === crumbs.length - 1
          ? <span>{crumb.label}</span>
          : <a href={crumb.href}>{crumb.label}</a>
        }
      </li>
    ))}
  </ol>
</nav>
```

Add `<Breadcrumb entry={entry} />` above the `<slot />` in both layouts.

## Notes

- Breadcrumb intermediate paths may not resolve to actual pages (e.g. domain-level pages don't exist yet). Use `<span>` instead of `<a>` for segments with no corresponding page, or link to the tiles page filtered by domain.
- `aria-current="page"` on the last crumb satisfies WCAG 2.1 SC 2.4.8.
- CSS for the `ol` separator (e.g. ` > `) should use `::before` on `li:not(:first-child)` in `base.css`.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
[ -f src/components/Breadcrumb.astro ] && echo "Breadcrumb.astro exists" || echo "FAIL"
grep "Breadcrumb" src/layouts/DocsLayout.astro && echo "wired in DocsLayout" || echo "FAIL"
grep "Breadcrumb" src/layouts/SkillLayout.astro && echo "wired in SkillLayout" || echo "FAIL"
```
