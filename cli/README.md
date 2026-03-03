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

### Tessl Commands

Tessl registry management and publishing.

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
│   └── tessl.ts              # Tessl subcommands
└── lib/
    ├── audit/
    │   ├── audit-skill.ts    # Single skill audit
    │   ├── audit-all.ts      # Batch audit
    │   ├── audit-status.ts   # Compliance check
    │   └── audit-summary.ts  # Summary report
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
