# markdown-authoring

## Overview

Complete markdown authoring and documentation guidance covering syntax, documentation structure, markdownlint configuration, CI/CD integration, and writing style.

## Structure

```
markdown-authoring/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - navigation reference
  references/    # Detailed reference files by category
  evals/         # Evaluation scenarios
```

## Usage

1. Read `SKILL.md` to understand when to apply markdown best practices
2. Navigate to categories based on your task
3. Load reference files on-demand
4. Start with syntax, then move to documentation structure, then linting

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Markdown Syntax | CRITICAL | `syntax-` |
| 2 | Documentation Structure | HIGH | `docs-` |
| 3 | Linting Configuration | HIGH | `lint-` |
| 4 | Linting Integration | MEDIUM | `lint-` |

## Available References

**Markdown Syntax** (`syntax-`):
- `references/syntax-headings.md` - Heading levels, ATX vs setext, best practices
- `references/syntax-lists.md` - Ordered, unordered, nested, and task lists
- `references/syntax-code-blocks.md` - Fenced code blocks, language tags, inline code
- `references/syntax-links-images.md` - Inline, reference, automatic links, images, alt text
- `references/syntax-formatting.md` - Bold, italic, strikethrough, inline code formatting
- `references/syntax-other-elements.md` - Blockquotes, tables, horizontal rules, HTML, escaping

**Documentation Structure** (`docs-`):
- `references/docs-readme.md` - README structure, templates, and best practices
- `references/docs-api.md` - REST, function, GraphQL, and CLI documentation patterns
- `references/docs-changelog.md` - Keep a Changelog format and writing good entries
- `references/docs-organization.md` - File naming, directory structure, navigation, cross-references
- `references/docs-writing-style.md` - Active voice, conciseness, tone, audience considerations

**Linting Configuration** (`lint-`):
- `references/lint-config.md` - Configuration files, hierarchy, inline comments, common patterns
- `references/lint-rules.md` - All markdownlint rule categories and configuration examples

**Linting Integration** (`lint-`):
- `references/lint-cli.md` - markdownlint-cli2 usage, npm scripts, monorepo setup
- `references/lint-ci.md` - GitHub Actions, GitLab CI, CircleCI, Jenkins, Azure Pipelines, pre-commit hooks
- `references/lint-api.md` - Programmatic API, custom rules, build tool integration

---

Consolidated from 4 original skills: markdown-documentation, markdown-syntax-fundamentals, markdownlint-configuration, markdownlint-integration. 16 reference files across 4 categories.
