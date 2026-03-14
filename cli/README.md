# Tekhne CLI

TypeScript CLI tool for managing Tekhne skills using Commander.js and Bun runtime.

## Installation

```bash
bun install
chmod +x cli/index.ts
```

## Usage

Run directly with bun:

```bash
bun cli/index.ts <command> [options]
```

Or install globally:

```bash
bun link
tekhne <command> [options]
```

## Commands

### Audit Commands

Quality audit and compliance checking for skills.

```bash
tekhne audit skill <path>
```

Audit a single skill at the specified path.

```bash
tekhne audit all [options]
```

Audit all skills in the repository.

Options:
- `-s, --skill <path>` - Audit a single skill
- `-f, --force` - Force re-audit even if current audit exists

```bash
tekhne audit status
```

Check audit status and compliance for all skills. Shows:
- ✓ Compliant skills (audited within 30 days)
- ⚠ Outdated skills (>30 days old)
- ✗ Missing audits

```bash
tekhne audit summary
```

Generate comprehensive audit summary report with:
- Overall statistics (avg, median, min, max scores)
- Grade distribution
- Dimensional analysis (D1-D8)
- Top/bottom 10 skills

### README Commands

Maintain README.md skill tables organized by 12 domains.

```bash
tekhne readme update [options]
```

Options:
- `--dry-run` - Show changes without applying

Examples:

```bash
# Update README.md with current audit data
tekhne readme update

# Preview changes
tekhne readme update --dry-run
```

The command automatically:
- Discovers all skills across 12 domains
- Parses SKILL.md frontmatter for descriptions
- Finds latest audit data from `.context/audits/`
- Checks Tessl publication status from `tile.json`
- Generates markdown tables organized by domain
- Updates README.md in place

Used by lefthook pre-commit hook to keep README in sync with audits.

### Install Commands

Install skills to local agent directories via symlinks.

```bash
tekhne install [options]
```

Options:
- `-a, --agent <agents...>` - Target agents (opencode, cursor, gemini, claude, codex) (default: opencode)
- `-g, --global` - Install to global agent directories (~/.config/)
- `--dry-run` - Preview changes without modifying filesystem

Examples:

```bash
# Install to local .agents/skills (default)
tekhne install

# Install to multiple agents
tekhne install -a opencode -a cursor -a gemini

# Install globally for all projects
tekhne install --global

# Preview what would be installed
tekhne install --dry-run
```

Skills are symlinked with namespaced names to avoid collisions:
- Format: `domain--category--[subcategory--]skill-name`
- Example: `infrastructure--terraform--generator`

### Sync Commands

Sync OpenCode configuration: clean broken symlinks, sync MCP config, and link Tessl tiles.

```bash
tekhne sync opencode [options]
```

Options:

- `--opencode-skills` - Use `.opencode/skills` instead of `.agents/skills`
- `--dry-run` - Preview changes without modifying filesystem

Examples:

```bash
# Sync OpenCode configuration
tekhne sync opencode

# Preview changes
tekhne sync opencode --dry-run

# Use .opencode/skills directory
tekhne sync opencode --opencode-skills
```

The command performs three operations in order:

1. **Clean broken symlinks** — removes dead symlinks from the skills directory
2. **Sync MCP config** — reads `.mcp.json` and updates `opencode.json` with the `mcp` section in OpenCode format
3. **Link Tessl tiles** — creates `tessl__<skill_name>` symlinks for all skills defined in `.tessl/tiles/*/*/tile.json`

Replaces `scripts/sync-opencode.sh`.

### Tessl Commands

```bash
tekhne tessl manage [skill] [options]
```

Manage skill lifecycle (import, lint, review, publish).

Options:
- `-w, --workspace <name>` - Tessl workspace name (default: pantheon-ai)

Behavior:
- If `tile.json` exists: lint → review → publish
- If no `tile.json`: import skill

```bash
tekhne tessl publish-check <tiles...>
```

Validate tiles before publishing. Checks:
- tile.json existence
- SKILL.md files for all skills
- Runs `tessl skill lint`

## Architecture

```
cli/
├── index.ts                   # Entry point
├── commands/
│   ├── audit.ts              # Audit subcommands
│   ├── sync.ts               # Sync subcommands
│   └── tessl.ts              # Tessl subcommands
└── lib/
    ├── audit/
    │   ├── audit-skill.ts    # Single skill audit
    │   ├── audit-all.ts      # Batch audit
    │   ├── audit-status.ts   # Compliance check
    │   └── audit-summary.ts  # Summary report
    ├── sync/
    │   └── sync-opencode.ts  # OpenCode sync logic
    ├── tessl/
    │   ├── manage.ts         # Lifecycle management
    │   └── publish-check.ts  # Pre-publish validation
    └── utils/
        ├── logger.ts         # Color logging
        └── shell.ts          # Shell command execution
```

## Migration from Shell Scripts

This CLI replaces the following shell scripts:

| Shell Script | CLI Command |
|-------------|-------------|
| `audit-skill.sh` | `tekhne audit skill <path>` |
| `audit-all-skills.sh` | `tekhne audit all` |
| `check-skill-audit-status.sh` | `tekhne audit status` |
| `generate-audit-summary.sh` | `tekhne audit summary` |
| `manage-skills.sh` | `tekhne tessl manage` |
| `tessl-publish-check.sh` | `tekhne tessl publish-check` |
| `update-readme-ratings.sh` | `tekhne readme update` |
| `npx skills add ./skills` | `tekhne install` |
| `sync-opencode.sh` | `tekhne sync opencode` |

## Examples

Audit a single skill:

```bash
tekhne audit skill skills/infrastructure/terraform/generator
```

Audit all skills (skip today's audits):

```bash
tekhne audit all
```

Force re-audit all skills:

```bash
tekhne audit all --force
```

Check compliance status:

```bash
tekhne audit status
```

Generate summary report:

```bash
tekhne audit summary
```

Manage all skills in Tessl:

```bash
tekhne tessl manage
```

Manage specific skill:

```bash
tekhne tessl manage skills/infrastructure/terraform
```

Pre-publish validation:

```bash
tekhne tessl publish-check skills/infrastructure/terraform
```

## Development

Type checking:

```bash
bun run tsc --noEmit
```

Linting:

```bash
bunx @biomejs/biome check cli/
```
