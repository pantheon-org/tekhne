# biome-complete

## Overview

Complete Biome toolchain guidance covering configuration, linting rules, formatting, and migration from ESLint/Prettier.

## Structure

```
biome-complete/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - complete reference guide
  references/    # Detailed reference files by category
```

## Usage

1. Read `SKILL.md` to understand when to use Biome
2. Navigate to categories based on your task
3. Load reference files on-demand
4. Start with config, then linting, then formatting

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Configuration | CRITICAL | `config-` |
| 2 | Linting Rules | HIGH | `linting-` |
| 3 | Formatting | HIGH | `formatter-` |
| 4 | Migration | MEDIUM | `migration-` |

## Available References

**Configuration** (`config-`):
- `references/config-biome-json.md` - biome.json structure and schema
- `references/config-vcs.md` - VCS integration and ignored files
- `references/config-project.md` - Project organization and extends

**Linting Rules** (`linting-`):
- `references/linting-rule-categories.md` - Rule groups (correctness, style, etc.)
- `references/linting-severity.md` - Error, warning, info severity levels
- `references/linting-ignoring.md` - Disabling rules inline and in config
- `references/linting-recommended.md` - Recommended rule sets

**Formatting** (`formatter-`):
- `references/formatter-options.md` - Indentation, quotes, line width
- `references/formatter-line-endings.md` - CRLF vs LF configuration
- `references/formatter-trailing-commas.md` - Trailing comma options
- `references/formatter-ignore.md` - Ignoring files from formatting

**Migration** (`migration-`):
- `references/migration-from-eslint.md` - ESLint to Biome migration
- `references/migration-from-prettier.md` - Prettier to Biome migration
- `references/migration-gradual.md` - Gradual adoption strategies
- `references/migration-config-mapping.md` - Rule mapping reference

---

*Consolidates 2 original skills: biome-configuration, biome-linting*

*15 reference files across 4 categories*
