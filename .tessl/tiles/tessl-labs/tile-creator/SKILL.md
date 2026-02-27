---
name: tile-creator
description: Create tessl tiles containing docs, rules, or skills. Use when a user wants to create a tile, package content for tessl, or needs help with tile.json configuration. Also triggers on "create tile from", "convert to tile", "turn into tile", "make tile from", or "tile this repo" for existing content.
---

# Tessl Tile Creator

Create tessl tiles from scratch or from existing content (repos, docs, packages).

## Choosing Content Types

| Type       | Purpose              | Agent Behavior                                | Example Content                                         |
| ---------- | -------------------- | --------------------------------------------- | ------------------------------------------------------- |
| **Docs**   | Facts and knowledge  | Agent _reaches_ for info on-demand (like RAG) | API references, schemas, architecture, config options   |
| **Skills** | Procedural workflows | Agent _does_ new complex tasks                | Deployment steps, migration procedures, build workflows |
| **Rules**  | Critical constraints | Loaded into agent's instruction prompt        | Security policies, naming conventions                   |

### Decision checklist

1. **Does it describe steps to _do_ something?** (deploy, migrate, build, test, release)
   → **Skill**. Workflows, runbooks, and step-by-step procedures are skills, even if the source calls them "documentation."
2. **Is it a constraint that must ALWAYS be enforced?** (MUST, NEVER, always)
   → **Rule** (use sparingly)
3. **Is it factual reference material?** (APIs, schemas, config, architecture)
   → **Docs**

**Common mistake**: Putting procedural/workflow content into docs. If the content has numbered steps, imperative verbs, or describes a process — it belongs in a skill, not docs.

Most content should be docs or skills. Rules are reserved for critical behavioral constraints only.

## Creating the Tile Structure

Two methods are available. Use whichever is available in your environment:

### Option A: MCP tool (preferred if available)

Use the `mcp__tessl__new_tile` tool. It has typed parameters so there's no syntax to get wrong, and it never triggers interactive prompts.

Parameters:

- `name`: "workspace/tile-name" (required)
- `summary`: brief description (required)
- `path`: **absolute** output directory path
- `docs`: true for documentation tiles
- `rules`: rule name for rules tiles
- `skill`: `{"name": "skill-name", "description": "..."}` for skill tiles
- `describes`: purl string for package docs
- `isPrivate`: boolean, defaults to true

### Option B: CLI

Use the `tessl tile new` command with ALL required flags:

```bash
tessl tile new \
  --name workspace/tile-name \
  --summary "Description" \
  --workspace workspace \
  --path /absolute/path/to/tile \
  --docs  # or --rules rule-name, or --skill-name name --skill-description "desc"
```

**Critical**: Always pass `--workspace` — omitting it triggers an interactive prompt that blocks execution. The `--workspace` value must match the prefix of `--name` (e.g., if `--name local/foo`, then `--workspace local`).

See [CLI commands](./references/cli-commands.md) for full flag reference.

---

**Path rule**: Always use absolute paths for tile creation and ALL subsequent file operations. Store the tile's absolute path and use it consistently — the working directory can change between tool calls.

After scaffolding, validate with: `tessl tile lint /absolute/path/to/tile`

## Workflows

Choose the most relevant workflow based on your task:

- **From scratch** (no existing content): [tile-from-scratch.md](./references/workflows/tile-from-scratch.md)
- **From existing source code** (repo, package, codebase): [tile-from-source.md](./references/workflows/tile-from-source.md)
- **From existing docs/context** (markdown, .mdc files, other tiles): [tile-from-docs.md](./references/workflows/tile-from-docs.md)

All workflows use the scaffolding method described above (MCP tool or CLI).

## Package Documentation

When documenting software packages, use the `{ .api }` marker for API signatures.

### API Marker Format

Mark all public API elements:

```markdown
### createClient(options) { .api }

Creates a new client instance.

**Parameters:**

- `options.apiKey` (string) - API key for authentication
- `options.timeout` (number, optional) - Request timeout in ms

**Returns:** `Client` - The client instance
```

### tile.json for Packages

Include the `describes` field with a purl:

```json
{
  "name": "tessl/npm-example",
  "version": "2.0.0",
  "docs": "docs/index.md",
  "describes": "pkg:npm/example-sdk@2.0.0",
  "summary": "TypeScript SDK for Example API"
}
```

Purl format: `pkg:<ecosystem>/<name>@<version>`

- npm: `pkg:npm/openai@6.9.1`
- pypi: `pkg:pypi/requests@2.31.0` (normalize names: lowercase, hyphens)

---

## References

- [CLI commands](./references/cli-commands.md) for full `tessl tile new` usage
- [docs format](./references/docs-format.md)
- [rules format](./references/rules-format.md)
- [skills format](./references/skills-format.md)
- [tile.json spec](./references/tile-json.md)
