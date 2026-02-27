# Tessl CLI Commands

Reference for tile and skill CLI commands.

## tessl tile new

Create a new tile structure.

```bash
tessl tile new --name <workspace/tile-name> --summary "<description>" --workspace <workspace> --path <output-dir> [content-flags]
```

**Required flags:**

| Flag          | Description                                 | Example                     |
| ------------- | ------------------------------------------- | --------------------------- |
| `--name`      | Tile name (workspace/tile-name format)      | `local/my-tile`             |
| `--summary`   | Brief description                           | `"API documentation for X"` |
| `--workspace` | Workspace (must match first part of --name) | `local`                     |
| `--path`      | Output directory                            | `./my-tile`                 |

**Content type flags (at least one required):**

| Flag                                               | Description                           |
| -------------------------------------------------- | ------------------------------------- |
| `--docs`                                           | Include docs/ folder                  |
| `--rules <name>`                                   | Include rules/ folder with named rule |
| `--skill-name <name> --skill-description "<desc>"` | Include skills/ folder                |

**Optional flags:**

| Flag                 | Description                                       |
| -------------------- | ------------------------------------------------- |
| `--describes <purl>` | PURL of package (for software package tiles only) |
| `--install`          | Auto-install in current project                   |
| `--public`           | Make tile public (default: private)               |

**Examples:**

```bash
# Docs tile
tessl tile new \
  --name local/my-docs \
  --summary "Documentation for My Library" \
  --workspace local \
  --path ./my-docs-tile \
  --docs

# Software package tile (with --describes)
tessl tile new \
  --name tessl/npm-openai \
  --summary "OpenAI TypeScript SDK" \
  --workspace tessl \
  --path ./openai-tile \
  --docs \
  --describes "pkg:npm/openai@6.9.1"

# Rules tile
tessl tile new \
  --name myorg/code-standards \
  --summary "Team coding standards" \
  --workspace myorg \
  --path ./standards-tile \
  --rules code-style

# Skill tile
tessl tile new \
  --name local/deploy \
  --summary "Deployment automation" \
  --workspace local \
  --path ./deploy-tile \
  --skill-name deploy \
  --skill-description "Deploy to staging or production. Use when deploying the application."
```

**Important:** The `--workspace` value must match the prefix of `--name`. If `--name` is `local/foo`, then `--workspace` must be `local`.

## tessl tile lint

Validate a tile structure and contents.

```bash
tessl tile lint <path-to-tile>
```

Reports validation errors and token costs.

## tessl skill new

Create a new standalone skill.

```bash
tessl skill new --name <skill-name> --description "<description>" --path <output-dir>
```

## tessl skill lint

Validate a skill structure.

```bash
tessl skill lint <path-to-skill>
```

## tessl skill review

Get quality feedback on a skill.

```bash
tessl skill review <path-to-skill>
```

Returns scores for description quality, content quality, and suggestions.
