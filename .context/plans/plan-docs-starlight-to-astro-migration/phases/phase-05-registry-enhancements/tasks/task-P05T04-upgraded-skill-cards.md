# P05T04 — upgraded-skill-cards

## Phase

Phase 05 — Registry Enhancements

## Goal

Enhance the skill cards on `tiles.astro` to display grade badge, domain tag, description excerpt, and registry link in a visually consistent card layout, replacing the plain `<li>` list from P02T04.

## File to create / modify

```
docs/src/components/SkillCard.astro   (CREATE)
docs/src/pages/tiles.astro            (use SkillCard)
docs/src/styles/base.css              (card styles)
```

## Implementation

Create `docs/src/components/SkillCard.astro`:

```astro
---
import type { CollectionEntry } from "astro:content";

interface Props {
  entry: CollectionEntry<"docs">;
  href: string;
}
const { entry, href } = Astro.props;
const { title, description, grade, domain, tilePublishedUrl, tileVersion } = entry.data;
const gradeClass = grade
  ? `grade-${grade.toLowerCase().replace("+", "-plus")}`
  : "grade-unknown";
---
<article class="skill-card" data-grade={grade} data-domain={domain}>
  <header class="card-header">
    <a href={href} class="card-title">{title}</a>
    {grade && <span class={`badge ${gradeClass}`}>{grade}</span>}
  </header>
  {description && <p class="card-description">{description}</p>}
  <footer class="card-footer">
    {domain && <span class="card-domain">{domain}</span>}
    {tilePublishedUrl && (
      <a href={tilePublishedUrl} target="_blank" rel="noopener noreferrer" class="card-registry">
        Tessl registry{tileVersion ? ` v${tileVersion}` : ""}
      </a>
    )}
  </footer>
</article>
```

Update `tiles.astro` to use `<SkillCard>` instead of the plain `<li>`.

Add to `base.css`:

```css
.skill-card {
  border: 1px solid var(--tk-border);
  border-radius: 6px;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.skill-card:hover {
  border-color: var(--tk-accent);
}
.card-title {
  font-weight: 600;
  text-decoration: none;
  color: var(--tk-text);
}
.card-description {
  font-size: 0.875rem;
  color: var(--tk-text-muted);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}
.card-footer {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
  margin-top: auto;
}
```

## Notes

- `--tk-border` and `--tk-text-muted` must be defined in `tokens.css` if not already present (add in Phase 3 or here).
- `-webkit-line-clamp` is widely supported; no fallback required for modern browsers.
- The `data-grade` and `data-domain` attributes on `<article>` feed into the P05T02 client-side filter script.

## Verification

```sh
cd docs
[ -f src/components/SkillCard.astro ] && echo "SkillCard.astro exists" || echo "FAIL"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep "SkillCard" src/pages/tiles.astro && echo "SkillCard used in tiles.astro" || echo "FAIL"
```
