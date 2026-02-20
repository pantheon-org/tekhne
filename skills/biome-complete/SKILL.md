---
name: biome-complete
description: |-
  Complete Biome toolchain guidance covering configuration, linting rules, formatting, and migration from ESLint/Prettier. Use when: configuring biome.json, applying linting rules, formatting code, or migrating from ESLint and Prettier to Biome.
  
  Keywords: Biome, biome.json, linting, formatting, ESLint, Prettier, biome check, biome format, biome lint, rule categories, VCS integration, migration
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# Biome Complete Toolchain

Comprehensive Biome guidance covering configuration, linting, formatting, and migration from ESLint/Prettier.

## When to Apply

Use this skill when:
- Configuring biome.json for projects
- Applying linting rules and severity levels
- Formatting JavaScript/TypeScript code
- Migrating from ESLint and Prettier
- Setting up VCS integration
- Debugging Biome errors

## Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Configuration | CRITICAL | `config-` |
| 2 | Linting Rules | HIGH | `linting-` |
| 3 | Formatting | HIGH | `formatter-` |
| 4 | Migration | MEDIUM | `migration-` |

## How to Use

Read individual reference files for detailed guidance:

```
references/config-biome-json.md
references/linting-rule-categories.md
references/formatter-options.md
```

Each reference file contains:
- Configuration file examples
- Rule explanations and examples
- CLI command usage
- Best practices and patterns
- Migration strategies

## References

- https://biomejs.dev/
- https://biomejs.dev/reference/configuration/
- https://biomejs.dev/linter/rules/
- https://biomejs.dev/formatter/
