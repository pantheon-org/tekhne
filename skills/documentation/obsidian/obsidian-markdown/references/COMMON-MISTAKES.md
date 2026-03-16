# Obsidian Markdown Common Mistakes

## 1. Using Markdown links for internal vault notes

**NEVER** use standard Markdown links for notes inside the vault. Obsidian will not update them on rename, and they break if the file moves.

**WHY:** Markdown links bypass Obsidian's link graph entirely, so renamed or moved notes silently break without any warning in the UI.

```markdown
<!-- BAD -->
[Meeting Notes](meeting-notes.md)

<!-- GOOD -->
[[Meeting Notes]]
```

Use `[[wikilinks]]` for all notes within the vault. Reserve `[text](url)` for external URLs only.

## 2. Using wikilink syntax for external URLs

**NEVER** wrap an external URL in `[[...]]` wikilink syntax. Obsidian resolves wikilinks against vault notes and will create a broken internal link instead of a hyperlink.

**WHY:** The URL becomes an unresolvable note reference in the graph, producing a dead link that cannot be opened in a browser.

```markdown
<!-- BAD -->
[[https://obsidian.md]]

<!-- GOOD -->
[Obsidian](https://obsidian.md)
```

## 3. Placing block IDs inline on list items

**NEVER** place a block ID on the same line as a list item or quote block. Obsidian requires block IDs for lists and quote blocks to appear on a **separate line** after the block, or the ID will not be recognised.

**WHY:** An inline block ID on a list item is parsed as plain text, so any `[[Note#^block-id]]` link targeting it will silently fail to resolve.

```markdown
<!-- BAD -->
- First item ^item-one
- Second item ^item-two

<!-- GOOD -->
- First item
- Second item

^list-block-id
```

## 4. Embedding an entire note when only a section is needed

**NEVER** embed a full note with `![[Note]]` when only a specific section is relevant. This pulls in every heading and paragraph of the target note, flooding the reading view with unwanted content.

**WHY:** Embedding an entire large note makes the host note hard to read and causes unnecessary re-rendering whenever any part of the source note changes.

```markdown
<!-- BAD -->
![[Architecture Diagram]]

<!-- GOOD -->
![[Architecture Diagram#Overview]]
```

## 5. Using spaces in tags

**NEVER** include spaces in tag names. A space terminates the tag; the rest of the text becomes plain content and is not indexed.

**WHY:** The partial tag appears in search results under the wrong name, and the trailing words are indexed as free text rather than as part of the tag hierarchy.

```markdown
<!-- BAD -->
#project alpha
#my tag

<!-- GOOD -->
#project-alpha
#my-tag
```

## 6. Placing frontmatter after content

**NEVER** place YAML frontmatter anywhere other than the very start of the file. It must appear before any headings, text, or blank lines. Frontmatter placed anywhere else is treated as a fenced code block and the properties are ignored.

**WHY:** Obsidian only parses the opening `---` block as properties; a misplaced block renders as a visible code fence and all defined properties are lost.

```markdown
<!-- BAD -->
# My Note

---
title: My Note
tags:
  - project
---

<!-- GOOD -->
---
title: My Note
tags:
  - project
---

# My Note
```
