# P02T06 — create-slug-route

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Create `docs/src/pages/skills/[...slug].astro` as the dynamic catch-all route that renders every skill and docs page using either `SkillLayout` or `DocsLayout` based on the collection entry's `id` prefix.

## File to create / modify

```
docs/src/pages/skills/[...slug].astro
```

## Implementation

```astro
---
import { getCollection, render } from "astro:content";
import type { CollectionEntry } from "astro:content";
import SkillLayout from "../../layouts/SkillLayout.astro";
import DocsLayout from "../../layouts/DocsLayout.astro";

export async function getStaticPaths() {
  const allDocs = await getCollection("docs");
  return allDocs.map((entry) => {
    const slug = entry.id
      .replace(/^skills\//, "")
      .replace(/(\/index)?\.mdx?$/, "");
    return {
      params: { slug },
      props: { entry },
    };
  });
}

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;
const { Content } = await render(entry);

const isSkill = entry.id.startsWith("skills/");
const Layout = isSkill ? SkillLayout : DocsLayout;
---
<Layout entry={entry}>
  <Content />
</Layout>
```

## Notes

- The `slug` param strips the `skills/` prefix and `.md`/`.mdx`/`/index` suffix so URLs match the existing Starlight routing scheme (e.g. `skills/ci-cd/gitlab-ci/generator` maps to `/tekhne/skills/ci-cd/gitlab-ci/generator`).
- `render(entry)` replaces the deprecated `entry.render()` API available in Astro v4+.
- Pages not under `skills/` in the collection (e.g. standalone docs pages) use `DocsLayout`.
- This route handles all 537+ pages; `getStaticPaths` is called once at build time.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -20
# Count generated static paths matches collection size
node --input-type=module <<'EOF'
import { getCollection } from "astro:content";
const all = await getCollection("docs");
console.log("entries:", all.length);
EOF
```
