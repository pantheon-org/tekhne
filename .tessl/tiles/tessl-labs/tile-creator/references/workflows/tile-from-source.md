## Creating Tiles from Existing Source

For converting existing repos, docs folders, or packages into tiles.

### Workflow

1. **Gather info**:
   - Source path (local folder, repo URL, or package name)
   - **Output absolute path** for the tile (e.g., `/Users/name/tiles/output-tile`). Use this path for ALL subsequent operations.
   - Desired workspace/tile name

2. **Analyze source**:
   - For **packages**: Read metadata (`package.json`, `pyproject.toml`, etc.), identify language, map public API
   - For **docs/repos**: Inventory existing markdown, assess structure, identify content types
   - Flag potential agent context using the decision checklist in SKILL.md: docs (facts), rules (behavioral - MUST/NEVER/always), skills (procedural workflows)
   - Use parallel agents to deeply analyze the code and document additional useful APIs (for external use), architectural specs and code patterns (for development use).

3. **Create tile structure**: Use the scaffolding method from SKILL.md (MCP tool or CLI) with your absolute output path.

   For packages, include the `describes` parameter/flag (e.g., `"pkg:ecosystem/name@version"`).

4. **Copy existing content into tile structure**: Copy and rename existing files into the scaffolded tile directories. Do NOT rewrite content yet — move files first, then edit in place.
   - `cp` or move existing markdown files into `docs/`, `skills/`, or `rules/` directories
   - Rename files to match tile naming conventions (lowercase, hyphens)
   - Preserve original content; restructuring happens in the edit step

5. **Read the relevant format references**: Familiarise yourself with the relevant format(s):
   - **Docs format**: See ../docs-format.md
   - **Rules format**: See ../rules-format.md
   - **Skills format**: See ../skills-format.md

6. **Edit copied files in place**: Modify the already-copied files to meet tile format requirements. Do NOT rewrite from scratch — edit what you copied in step 4.
   - Restructure for **progressive disclosure**: ensure index.md has overview + links, details in sub-pages
   - Apply **token efficiency**: trim verbose explanations, prefer examples and tables
   - Maintain **comprehensive coverage**: do not delete content, reorganize it
   - For packages: add `{ .api }` markers (see Package Documentation in SKILL.md)
   - If behavioral content detected: suggest to user before creating rules (keep minimal)
   - If procedural content detected: convert to skills (see decision checklist in SKILL.md)

7. **Configure tile.json**:
   - For packages: include `describes` field with purl
   - For non-packages: omit `describes`

8. **Validate and interpret lint output**:

   ```bash
   tessl tile lint /absolute/path/to/tile
   ```

   **Interpreting results:**
   - **Errors** (must fix): missing tile.json fields, unreachable markdown files, invalid frontmatter
   - **Warnings** (evaluate case-by-case): broken links to source code files (e.g., `src/lib/foo.ts`) are expected in documentation tiles — they reference code in the documented repo, not the tile itself. Safely ignored.
   - **Token counts**: informational, no action needed unless unusually high

   Fix errors, re-run lint until errors are resolved. Report remaining warnings to the user with explanation.

9. **Report**: Summarize what was created, note any gaps or assumptions for user review

### Index.md Structure

Flexible by source type:

**For packages:**

```markdown
# Package Name

Brief description.

## Installation

[install command]

## Quick Start

[basic usage example]

## API Reference

- [Module A](./module-a.md)
- [Module B](./module-b.md)
```

**For general docs:**

```markdown
# Title

Overview of what this documentation covers.

## Contents

- [Topic A](./topic-a.md) - Brief description
- [Topic B](./topic-b.md) - Brief description
```

---
