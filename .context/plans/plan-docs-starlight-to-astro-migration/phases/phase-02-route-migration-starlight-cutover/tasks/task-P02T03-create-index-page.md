# P02T03 — Create docs index page

## Goal

Create the top-level docs index route `src/pages/docs/index.astro` that lists
all documentation sections using the new `DocsLayout` from Phase 1.

## File

`src/pages/docs/index.astro`

## Context

Starlight generated this page automatically.  The bespoke replacement must use
`getCollection("docs")` to list content, group by domain, and render with
`DocsLayout`.

## Implementation

```astro
---
import { getCollection } from "astro:content";
import DocsLayout from "../../layouts/DocsLayout.astro";

const allDocs = await getCollection("docs");
const grouped = allDocs.reduce(
  (acc, entry) => {
    const [domain] = entry.slug.split("/");
    (acc[domain] ??= []).push(entry);
    return acc;
  },
  {} as Record<string, typeof allDocs>,
);
const domains = Object.keys(grouped).sort();
---

<DocsLayout title="Documentation">
  <h1>Documentation</h1>
  {domains.map((domain) => (
    <section>
      <h2>{domain}</h2>
      <ul>
        {grouped[domain]
          .sort((a, b) => (a.data.sidebar?.order ?? 99) - (b.data.sidebar?.order ?? 99))
          .map((entry) => (
            <li>
              <a href={`/tekhne/docs/${entry.slug}/`}>
                {entry.data.sidebar?.label ?? entry.data.title}
              </a>
            </li>
          ))}
      </ul>
    </section>
  ))}
</DocsLayout>
```

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f dist/tekhne/docs/index.html && echo "index page generated"
```
