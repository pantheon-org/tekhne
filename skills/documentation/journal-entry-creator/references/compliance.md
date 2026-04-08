# Compliance

Purpose

- Ensure every journal entry follows the project's structure, formatting, and quality standards so entries are discoverable, consistent, and machine-validated.

Quick automation (recommended)

- Format: `npx prettier --write path/to/entry.md`
- Lint: `npx markdownlint-cli2 path/to/entry.md`
- Validator: `bash scripts/validate-journal-entry.sh path/to/entry.md` (accepts multiple files)

Required tools

- Node.js + `npx` (for Prettier and markdownlint-cli2)
- `bash`, `perl`, `rg` (ripgrep), and GNU awk (or `gawk`) for the repository validator

YAML frontmatter (standardized)

All templates now include a standard YAML frontmatter that journal entries should adopt when appropriate. Fields:

- `title` (string): descriptive title
- `date` (YYYY-MM-DD): entry date — must match filename prefix
- `authors` (array): list of author names
- `tags` (array): lowercase, hyphen-separated tags
- `source` (string): optional URL or origin
- `status` (string): `draft`, `published`, or other workflow states

Example frontmatter:

```yaml
---
title: "Example — Investigating Service Crash"
date: 2025-09-01
authors:
  - Thomas Roche
tags:
  - troubleshooting
  - systemd
source: ""
status: draft
---
```

Checklist (run for every new or updated entry)

- [ ] Filename: `YYYY-MM-DD-slug.md` (ISO 8601 date prefix must match entry date)
- [ ] Location: placed under `YYYY/MM/` where YYYY and MM match the filename
- [ ] Single H1 with full date: `# Title - Month D, YYYY` (exactly one H1)
- [ ] `## Session Overview` (or an equivalent short context section) present
- [ ] Required sections for the entry type present:
  - Troubleshooting: Problem Description, Investigation Process, Resolution Steps, Session Outcome
  - Learning: Context, Key Learning, Solution/Process, Use Cases
- [ ] All fenced code blocks have language specifiers (e.g., ```bash)
- [ ] No duplicate headings (MD024) and no multiple consecutive blank lines (MD012)
- [ ] Tags present and correctly formatted (lowercase, hyphen-separated, nested allowed)
- [ ] Images include alt text where present
- [ ] No trailing whitespace; prefer line length ≤ 120 characters when practical
- [ ] Prettier-format and markdownlint report zero errors
- [ ] Commit message follows convention: `Add journal entry: [Brief Description] (YYYY-MM-DD)`

Common validator failures & fixes

- `Single H1` errors: remove extra H1s or convert them to H2 (`##`) and ensure date is present in the H1.
- `Code block language missing`: add the language after the opening fence, e.g., ```bash
- `Duplicate headings` or `multiple blank lines`: remove the duplicate section or extra blank lines.
- `Filename/location mismatch`: move or rename the file so the path and filename date match the H1 date.

CI and pre-commit behavior

- Pre-commit (lefthook) runs Prettier, markdownlint, then the validator for staged files under `202*/**/*.md`.
- CI should treat validator failures as blocking; run the same commands used locally.
- If Prettier fixes files during pre-commit, re-run linters/validator to ensure a clean state before committing.

Notes for AI agents

- Run the validator and linter automatically on any files you create or update, fix issues programmatically when safe (formatting, adding code fence languages), and prompt the user for ambiguous fixes
  (renaming files, content deletions).

Reference

- This checklist is canonical: `.opencode/checklist/compliance.md` — link to this file from `AGENT.md` and other contributing docs.
