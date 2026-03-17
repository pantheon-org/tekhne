# P04T05 — Edit-on-GitHub link

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Add a per-page "Edit on GitHub" link to `DocsLayout` and `SkillLayout` that
constructs a direct GitHub edit URL from the content entry's `id`.

## File to create / modify

```
src/layouts/DocsLayout.astro
src/layouts/SkillLayout.astro
```

## Implementation

In each layout, accept an `entryId` prop and render the link in the page footer
or bottom-of-content area:

```astro
---
interface Props {
  title: string;
  entryId?: string;   // e.g. "ci-cd/github-actions/SKILL.md"
}
const { title, entryId } = Astro.props;
const GITHUB_BASE =
  "https://github.com/pantheon-org/tekhne/edit/main/";
const editUrl = entryId ? `${GITHUB_BASE}${entryId}` : undefined;
---

<!-- inside the layout template -->
{editUrl && (
  <a
    href={editUrl}
    class="edit-link"
    target="_blank"
    rel="noopener noreferrer"
  >
    Edit this page on GitHub
  </a>
)}
```

In `[...slug].astro`, pass `entry.id` as the `entryId` prop:

```astro
<DocsLayout title={entry.data.title} entryId={entry.id}>
```

## Notes

- `entry.id` is the collection-relative path (e.g.
  `ci-cd/github-actions/SKILL.md`), not the full filesystem path.
- The GitHub edit link uses `main` branch — adjust if the default branch differs.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
grep "edit-link\|Edit this page" src/layouts/DocsLayout.astro && echo "edit link present"
```
