## Creating Tiles from Scratch

For greenfield tiles with no existing content. Applicable when the user has asked for a new tile without pointing to existing context.

### Workflow

1. **Determine content type**: docs, skills, or rules (usually just one). Use the decision checklist in SKILL.md to classify correctly. Decide on an **absolute output path** (e.g., `/Users/name/tiles/my-tile`) and use it for all subsequent operations.

2. **Create structure**: Use the scaffolding method from SKILL.md (MCP tool or CLI) with your absolute output path.

3. **Read the relevant format references**: Familiarise yourself with the relevant format(s):
   - **Docs format**: See ../docs-format.md
   - **Rules format**: See ../rules-format.md
   - **Skills format**: See ../skills-format.md

4. **Add content**: Write markdown files in the appropriate folders

5. **Validate and interpret lint output**:
   ```bash
   tessl tile lint /absolute/path/to/tile
   ```

   **Interpreting results:**
   - **Errors** (must fix): missing tile.json fields, unreachable markdown files, invalid frontmatter
   - **Warnings** (evaluate case-by-case): broken links to source code files (e.g., `src/lib/foo.ts`) are expected in documentation tiles — they reference code in the documented repo, not the tile itself. Safely ignored.
   - **Token counts**: informational, no action needed unless unusually high

   Fix errors, re-run lint until errors are resolved. Report remaining warnings to the user with explanation.

### Tile Structure

```
my-tile/
├── tile.json
├── docs/           # Documentation (optional)
│   └── index.md
├── rules/          # Rules/steering (optional)
│   └── my-rule.md
└── skills/         # Skills (optional)
    └── my-skill/
        └── SKILL.md
```
