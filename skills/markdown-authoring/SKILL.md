---
name: markdown-authoring
description: "Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style."
---

# Markdown Authoring

## When to Use

Use this skill when the task involves Markdown content, linting, or documentation standards.

## When Not to Use

Do not use this skill for non-Markdown document formats (for example `.adoc` or `.rst`) unless conversion is part of the task.

## Core Principles

1. Structure first, wording second.
2. Keep Markdown lint-clean and deterministic.
3. Use code fences with explicit language tags.
4. Keep file-level conventions consistent across the repository.

## Deterministic Workflow

1. Identify document type (README, guide, API doc, changelog).
2. Apply template and section order from references.
3. Write content with concise headings and actionable examples.
4. Run markdownlint locally and fix violations. Re-run until zero errors are reported. If a violation persists: identify the specific rule, fix the source content or apply a narrow, justified inline exception, then re-lint. Do not proceed until lint is clean.
   - Lint-fix loop: `lint fails → identify rule → fix source or add justified exception → re-lint → proceed only when clean`
5. Validate CI/pre-commit integration for regression prevention.

## Quick Commands

### Lint all Markdown files

```bash
bunx markdownlint-cli2 "**/*.md"
```

Expected result: no errors for staged or target files.

### Lint one skill folder

```bash
bunx markdownlint-cli2 "skills/markdown-authoring/**/*.md"
```

Expected result: folder-specific Markdown issues reported or zero errors.

### Run repository checks

```bash
bunx @biomejs/biome check .
```

Expected result: non-Markdown formatting/lint issues also caught.

### Evaluate this skill quality

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh markdown-authoring --json
```

Expected result: updated dimension scores and grade.

## Anti-Patterns

### NEVER ship Markdown with missing fence language tags

**WHY:** Untyped code blocks reduce readability and tooling support.

**BAD:** Use plain triple backticks for code samples.
**GOOD:** Use typed fences like ` ```bash ` or ` ```ts `.

**Consequence:** Syntax highlighting and lint checks become inconsistent.

### NEVER ignore markdownlint violations by disabling broad rules

**WHY:** Global suppression hides real quality and maintainability issues.

**BAD:** Turn off multiple rules to make CI green quickly.
**GOOD:** Fix source content or apply narrow, justified exceptions.

**Consequence:** Documentation quality degrades release-over-release.

### NEVER use heading levels out of sequence

**WHY:** Skipped heading levels break document hierarchy.

**BAD:** Jump from `##` to `####` with no `###`.
**GOOD:** Increase heading levels one step at a time.

**Consequence:** Navigation and accessibility suffer.

### NEVER mix inconsistent list and table styles in one document

**WHY:** Style drift makes docs harder to scan and review.

**BAD:** Alternate list markers and inconsistent table pipe spacing.
**GOOD:** Keep one list style and normalized table formatting.

**Consequence:** Diff noise increases and review confidence drops.

## References

- `references/syntax-headings.md`
- `references/syntax-lists.md`
- `references/syntax-code-blocks.md`
- `references/syntax-links-images.md`
- `references/docs-readme.md`
- `references/docs-organization.md`
- `references/docs-writing-style.md`
- `references/lint-config.md`
- `references/lint-rules.md`
- `references/lint-cli.md`
- `references/lint-ci.md`
