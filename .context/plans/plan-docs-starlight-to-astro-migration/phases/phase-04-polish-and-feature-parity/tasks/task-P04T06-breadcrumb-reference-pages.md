# P04T06 — Breadcrumb for reference sub-pages

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Render a breadcrumb trail (skill name → reference title) on reference sub-pages
inside `SkillLayout` so users can navigate up without the browser back button.

## File to create / modify

```
src/layouts/SkillLayout.astro
src/components/Breadcrumb.astro   (new)
```

## Implementation

### 1. Create Breadcrumb.astro

```astro
---
interface Props {
  crumbs: Array<{ label: string; href?: string }>;
}
const { crumbs } = Astro.props;
---

<nav aria-label="Breadcrumb" class="breadcrumb">
  <ol>
    {crumbs.map((crumb, i) => (
      <li>
        {crumb.href && i < crumbs.length - 1
          ? <a href={crumb.href}>{crumb.label}</a>
          : <span aria-current="page">{crumb.label}</span>}
      </li>
    ))}
  </ol>
</nav>
```

### 2. Wire into SkillLayout

Add a `breadcrumbs` prop to `SkillLayout.astro`:

```astro
---
interface Props {
  title: string;
  breadcrumbs?: Array<{ label: string; href?: string }>;
  entryId?: string;
}
const { title, breadcrumbs, entryId } = Astro.props;
---

{breadcrumbs && breadcrumbs.length > 1 && (
  <Breadcrumb crumbs={breadcrumbs} />
)}
```

### 3. Populate from slug route

In the reference sub-page route, construct the crumbs from the slug segments:

```astro
const crumbs = [
  { label: "Docs", href: "/tekhne/docs/" },
  { label: skillName, href: `/tekhne/docs/${skillSlug}/` },
  { label: entry.data.title },
];
```

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f src/components/Breadcrumb.astro && echo "Breadcrumb component exists"
grep "Breadcrumb" src/layouts/SkillLayout.astro && echo "wired in SkillLayout"
```
