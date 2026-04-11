# Scenario 02: Fix a Broken Obsidian Note

## User Prompt

A colleague shared the following Obsidian note, but it has several problems that are causing broken links and missing metadata. Fix all the issues and write the corrected note to `fixed-note.md`.

Here is the note as-is:

```markdown
# Weekly Sync

This week we reviewed progress on the main project.

---
title: Weekly Sync
tags:
  - project alpha
date: 2024-03-10
---

Action items from [Meeting Notes](meeting-notes.md):

- Confirm budget with finance
- Send update to [[https://company-portal.example.com]]
- Review designs with [Design Team](design-team.md)

Next sync is scheduled. See [Architecture Overview](architecture-overview.md) for context.
```

## Expected Behavior

1. Move the YAML frontmatter block so it appears before the `# Weekly Sync` heading and before any body content
2. Replace the tag `project alpha` with a valid tag that has no space, such as `project-alpha`
3. Convert all references to internal notes (Meeting Notes, Design Team, Architecture Overview) to `[[...]]` wikilink syntax
4. Convert the external company portal URL from `[[https://...]]` wikilink syntax to `[text](https://...)` standard Markdown link syntax

## Success Criteria

- **Frontmatter moved to top**: The YAML frontmatter block (`---`) appears before the `# Weekly Sync` heading and before any body content
- **Tag space removed**: The tag `project alpha` is replaced with a valid tag that has no space, such as `#project-alpha` or `project-alpha` in the frontmatter list
- **Internal note links converted to wikilinks**: All references to internal notes (Meeting Notes, Design Team, Architecture Overview) use `[[...]]` wikilink syntax, not `[text](file.md)` Markdown links
- **External URL uses Markdown link syntax**: The company portal URL uses `[text](https://...)` syntax, not `[[https://...]]` wikilink syntax

## Failure Conditions

- YAML frontmatter block still appears after the `# Weekly Sync` heading or body content
- Tag still contains a space (e.g., `project alpha`)
- Any internal note reference (Meeting Notes, Design Team, Architecture Overview) still uses `[text](file.md)` instead of `[[...]]`
- The external company portal URL still uses `[[https://...]]` wikilink syntax instead of standard Markdown link syntax
