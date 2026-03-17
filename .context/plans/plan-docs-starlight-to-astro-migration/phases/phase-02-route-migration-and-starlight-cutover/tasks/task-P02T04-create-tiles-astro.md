# P02T04 — create-tiles-astro

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Create `docs/src/pages/tiles.astro` as the skill catalogue page, replacing `src/content/docs/tiles.md` with a dynamic Astro page that groups skills by domain, shows grade badges, and links to `tilePublishedUrl` when available.

## File to create / modify

```
docs/src/pages/tiles.astro
```

## Implementation

```astro
---
import { getCollection } from "astro:content";
import BaseLayout from "../layouts/BaseLayout.astro";

const BASE = import.meta.env.BASE_URL.replace(/\/$/, "");

const allDocs = await getCollection("docs");
const skills = allDocs.filter((e) => e.id.startsWith("skills/"));

const GRADE_CLASS: Record<string, string> = {
  A: "grade-a",
  "B+": "grade-b-plus",
  B: "grade-b",
  C: "grade-c",
};

const byDomain: Record<string, typeof skills> = {};
for (const skill of skills) {
  const parts = skill.id.replace(/^skills\//, "").split("/");
  const domain = parts[0] ?? "other";
  (byDomain[domain] ??= []).push(skill);
}
const domains = Object.keys(byDomain).sort();
---
<BaseLayout title="Skill Catalogue — Tekhne" description="Browse all Tekhne skills by domain.">
  <main class="tiles-page">
    <h1>Skill Catalogue</h1>
    {domains.map((domain) => (
      <section class="domain-group">
        <h2>{domain}</h2>
        <ul class="skill-list">
          {byDomain[domain].map((skill) => {
            const grade = skill.data.grade;
            const slug = skill.id.replace(/^skills\//, "").replace(/\/index$/, "");
            return (
              <li class="skill-card">
                <a href={`${BASE}/skills/${slug}`}>{skill.data.title}</a>
                {grade && <span class={`badge ${GRADE_CLASS[grade] ?? "grade-unknown"}`}>{grade}</span>}
                {skill.data.tilePublishedUrl && (
                  <a class="registry-link" href={skill.data.tilePublishedUrl} target="_blank" rel="noopener">
                    registry
                  </a>
                )}
              </li>
            );
          })}
        </ul>
      </section>
    ))}
  </main>
</BaseLayout>
```

## Notes

- This page replaces `src/content/docs/tiles.md` (deleted in P02T10).
- `tilePublishedUrl` is injected by `prelink.mjs` (P02T05); it is optional and may be absent for unpublished skills.
- Domain grouping mirrors the `LeftNav.astro` domain structure for visual consistency.
- Grade badge classes (`grade-a`, `grade-b-plus`, etc.) are defined in `tokens.css` or `base.css` in Phase 3.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep -r "starlight" src/pages/tiles.astro && echo "FAIL: starlight import found" || echo "ok"
```
