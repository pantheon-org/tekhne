# P05T04 — Upgraded skill cards

## Phase

Phase 05 — Registry Enhancements

## Goal

Enrich each skill card on the tiles index page with a grade badge (colour-coded),
eval count, latest audit date, and domain tag so reviewers can assess skill
quality at a glance.

## File to create / modify

```
src/components/SkillCard.astro   (new or update if exists)
src/pages/docs/tiles/index.astro
```

## Implementation

### 1. SkillCard.astro

```astro
---
interface Props {
  title: string;
  description?: string;
  href: string;
  domain?: string;
  grade?: string;        // "A" | "B+" | "B" | "C"
  evalCount?: number;
  auditDate?: string;    // ISO date string
}
const { title, description, href, domain, grade, evalCount, auditDate } = Astro.props;
const gradeClass = grade?.replace("+", "plus").toLowerCase() ?? "";
---

<article
  class="skill-card"
  data-tile-card
  data-domain={domain ?? ""}
  data-grade={grade ?? ""}
>
  <a href={href} class="skill-card__title">{title}</a>
  {domain && <span class="skill-card__domain">{domain}</span>}
  {grade && <span class={`skill-card__grade grade-${gradeClass}`}>{grade}</span>}
  {description && <p class="skill-card__desc">{description}</p>}
  <footer class="skill-card__meta">
    {evalCount != null && <span>{evalCount} evals</span>}
    {auditDate && <time datetime={auditDate}>{auditDate.slice(0, 10)}</time>}
  </footer>
</article>

<style>
  .grade-a    { background: var(--tk-color-grade-a,    #22c55e); }
  .grade-bplus{ background: var(--tk-color-grade-bplus, #84cc16); }
  .grade-b    { background: var(--tk-color-grade-b,    #eab308); }
  .grade-c    { background: var(--tk-color-grade-c,    #ef4444); }
</style>
```

### 2. Update tiles index page

Replace the existing `<li>` list items with `<SkillCard>` components, passing
`grade`, `evalCount`, and `auditDate` from the tile's frontmatter (or omit if
data is not yet available).

## Notes

- Grade token colours should be added to `tokens.css` under both `:root` and
  `[data-theme="dark"]` if not already present.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f src/components/SkillCard.astro && echo "SkillCard exists"
grep "SkillCard" src/pages/docs/tiles/index.astro && echo "SkillCard used in tiles page"
```
