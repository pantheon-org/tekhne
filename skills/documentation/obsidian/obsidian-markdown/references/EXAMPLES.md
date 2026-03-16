# Obsidian Markdown Examples

## Complete Note Example

A full note combining frontmatter properties, wikilinks, callouts, embeds, tasks, math, and tags.

````markdown
---
title: Project Alpha
date: 2024-01-15
tags:
  - project
  - active
status: in-progress
---

# Project Alpha

This project aims to [[improve workflow]] using modern techniques.

> [!important] Key Deadline
> The first milestone is due on ==January 30th==.

## Tasks

- [x] Initial planning
- [ ] Development phase
  - [ ] Backend implementation
  - [ ] Frontend design

## Notes

The algorithm uses $O(n \log n)$ sorting. See [[Algorithm Notes#Sorting]] for details.

![[Architecture Diagram.png|600]]

Reviewed in [[Meeting Notes 2024-01-10#Decisions]].
````

### What this example demonstrates

| Element | Syntax used |
|---------|-------------|
| Frontmatter properties | `---` YAML block with `title`, `date`, `tags`, `status` |
| Wikilink | `[[improve workflow]]` |
| Callout | `> [!important] Key Deadline` |
| Highlight | `==January 30th==` |
| Task list | `- [x]` and `- [ ]` |
| Wikilink to heading | `[[Algorithm Notes#Sorting]]` |
| Image embed with width | `![[Architecture Diagram.png\|600]]` |
| Wikilink to block in note | `[[Meeting Notes 2024-01-10#Decisions]]` |
| Inline math | `$O(n \log n)$` |
