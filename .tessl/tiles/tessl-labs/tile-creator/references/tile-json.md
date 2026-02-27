# tile.json Configuration

The manifest file that defines a tile's metadata and content locations.

## Required Fields

| Field     | Type   | Description                            |
| --------- | ------ | -------------------------------------- |
| `name`    | string | Tile identifier: `workspace/tile-name` |
| `version` | string | Semantic version (e.g., `1.0.0`)       |
| `summary` | string | Brief description for discovery        |

## Optional Fields

| Field       | Type    | Description                                         |
| ----------- | ------- | --------------------------------------------------- |
| `private`   | boolean | `true` (default) = workspace-only, `false` = public |
| `docs`      | string  | Path to docs entry point (e.g., `docs/index.md`)    |
| `describes` | string  | PURL of external package this documents             |
| `steering`  | object  | Map of rule names to rule files                     |
| `skills`    | object  | Map of skill names to SKILL.md paths                |

## Validation Rules

- At least one of `docs`, `steering`, or `skills` required
- If `describes` is set, `docs` is required
- Names must be lowercase with hyphens only

## PURL Format (describes field)

When documenting a software package, use the Package URL (PURL) format:

```
pkg:<ecosystem>/<name>@<version>
```

| Ecosystem | Example                                             |
| --------- | --------------------------------------------------- |
| npm       | `pkg:npm/openai@6.9.1`                              |
| pypi      | `pkg:pypi/requests@2.31.0`                          |
| maven     | `pkg:maven/org.apache.commons/commons-lang3@3.14.0` |
| nuget     | `pkg:nuget/Newtonsoft.Json@13.0.3`                  |

**Note**: Normalize package names (e.g., PyPI uses lowercase with hyphens: `emoji_logger` becomes `pkg:pypi/emoji-logger@1.0.0`)

## Examples

**Documentation tile for npm package:**

```json
{
  "name": "tessl/npm-openai",
  "version": "6.9.1",
  "docs": "docs/index.md",
  "describes": "pkg:npm/openai@6.9.1",
  "summary": "OpenAI TypeScript library",
  "private": false
}
```

**Private rules tile:**

```json
{
  "name": "myorg/code-standards",
  "version": "0.1.0",
  "summary": "Team coding standards",
  "private": true,
  "steering": {
    "naming": { "rules": "rules/naming.md" },
    "error-handling": { "rules": "rules/error-handling.md" }
  }
}
```

**Skills tile:**

```json
{
  "name": "myorg/deploy-tools",
  "version": "1.0.0",
  "summary": "Deployment automation skills",
  "skills": {
    "deploy-staging": { "path": "skills/deploy-staging/SKILL.md" },
    "deploy-prod": { "path": "skills/deploy-prod/SKILL.md" }
  }
}
```

**Mixed tile (docs + rules + skills):**

```json
{
  "name": "local/project-context",
  "version": "0.1.0",
  "summary": "Project documentation and tools",
  "private": true,
  "docs": "docs/index.md",
  "steering": {
    "code-style": { "rules": "rules/code-style.md" }
  },
  "skills": {
    "test-runner": { "path": "skills/test-runner/SKILL.md" }
  }
}
```
