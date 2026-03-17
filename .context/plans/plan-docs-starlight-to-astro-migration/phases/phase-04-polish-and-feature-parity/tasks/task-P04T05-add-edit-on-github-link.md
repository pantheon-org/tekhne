# P04T05 — add-edit-on-github-link

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Add an "Edit on GitHub" link to `DocsLayout.astro` and `SkillLayout.astro` that points to the source markdown file on the `main` branch, matching the feature Starlight provided automatically.

## File to create / modify

```
docs/src/layouts/DocsLayout.astro
docs/src/layouts/SkillLayout.astro
```

## Implementation

Derive the GitHub edit URL from `entry.id` (the collection file path):

```astro
---
const GITHUB_EDIT_BASE =
  "https://github.com/pantheon-org/tekhne/edit/main/";

// entry.id is relative to docs/src/content/docs/
// The source file lives at skills/<path>/SKILL.md in the repo root
const sourceRelPath = entry.id.startsWith("skills/")
  ? entry.id.replace(/\/index$/, "") + ".md"
  : `docs/src/content/docs/${entry.id}`;

const editUrl = `${GITHUB_EDIT_BASE}${sourceRelPath}`;
---
```

Add to the layout template:

```astro
<footer class="page-footer">
  <a href={editUrl} target="_blank" rel="noopener noreferrer" class="edit-link">
    Edit this page on GitHub
  </a>
</footer>
```

## Notes

- The `entry.id` path within the collection corresponds to the file path relative to `src/content/docs/`. For skill pages the actual source is in `skills/<domain>/...` at the repo root — the path mapping must account for this.
- If `prelink.mjs` copies skill files into `src/content/docs/skills/` during build, the source URL should point to the original `skills/` location, not the copy.
- `rel="noopener noreferrer"` is required for cross-origin links that open in a new tab.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep "edit-link\|editUrl\|github.com" src/layouts/DocsLayout.astro && echo "edit link present in DocsLayout" || echo "FAIL"
grep "edit-link\|editUrl\|github.com" src/layouts/SkillLayout.astro && echo "edit link present in SkillLayout" || echo "FAIL"
```
