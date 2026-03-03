# markdown-authoring

## Overview

Complete markdown authoring and documentation guidance covering syntax, documentation structure, markdownlint configuration, and CI/CD integration.

## Structure

```
markdown-authoring/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - complete reference guide
  references/    # Detailed reference files by category
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
| 2 | Documentation Structure | HIGH | `documentation-` |
| 3 | Linting Configuration | HIGH | `linting-config-` |
| 4 | Linting Integration | MEDIUM | `linting-integration-` |

## Available References

**Markdown Syntax** (`syntax-`):
- `references/syntax-fundamentals.md` - Headings, paragraphs, emphasis
- `references/syntax-lists.md` - Ordered, unordered, nested lists
- `references/syntax-links.md` - Inline, reference, autolinks
- `references/syntax-images.md` - Image syntax and alt text
- `references/syntax-code.md` - Inline code and code blocks
- `references/syntax-tables.md` - Table syntax and alignment
- `references/syntax-blockquotes.md` - Blockquote syntax

**Documentation Structure** (`documentation-`):
- `references/documentation-readme.md` - README structure and sections
- `references/documentation-api.md` - API documentation patterns
- `references/documentation-changelogs.md` - Changelog formatting
- `references/documentation-contributing.md` - Contributing guide structure

**Linting Configuration** (`linting-config-`):
- `references/linting-config-rules.md` - markdownlint rule categories
- `references/linting-config-file.md` - .markdownlint.json configuration
- `references/linting-config-inline.md` - Inline comments to disable rules
- `references/linting-config-inheritance.md` - Configuration file hierarchy

**Linting Integration** (`linting-integration-`):
- `references/linting-integration-cli.md` - markdownlint-cli usage
- `references/linting-integration-ci.md` - GitHub Actions, GitLab CI
- `references/linting-integration-pre-commit.md` - Pre-commit hooks
- `references/linting-integration-api.md` - Programmatic usage

---

*Consolidates 4 original skills: markdown-documentation, markdown-syntax-fundamentals, markdownlint-configuration, markdownlint-integration*

*19 reference files across 4 categories*
