---
name: obsidian-markdown
description: Create and edit Obsidian Flavored Markdown with wikilinks, embeds, callouts, properties, and other Obsidian-specific syntax. Use when working with .md files in Obsidian, or when the user mentions wikilinks, callouts, frontmatter, tags, embeds, or Obsidian notes.
---

# Obsidian Flavored Markdown Skill

Create and edit valid Obsidian Flavored Markdown. Obsidian extends CommonMark and GFM with wikilinks, embeds, callouts, properties, comments, and other syntax. This skill covers only Obsidian-specific extensions -- standard Markdown (headings, bold, italic, lists, quotes, code blocks, tables) is assumed knowledge.

## Workflow: Creating an Obsidian Note

1. **Add frontmatter** with properties (title, tags, aliases) at the top of the file. See [PROPERTIES.md](references/PROPERTIES.md) for all property types.
2. **Write content** using standard Markdown for structure, plus Obsidian-specific syntax below.
3. **Link related notes** using wikilinks (`[[Note]]`) for internal vault connections, or standard Markdown links for external URLs.
4. **Embed content** from other notes, images, or PDFs using the `![[embed]]` syntax. See [EMBEDS.md](references/EMBEDS.md) for all embed types.
5. **Add callouts** for highlighted information using `> [!type]` syntax. See [CALLOUTS.md](references/CALLOUTS.md) for all callout types.
6. **Verify** the note renders correctly in Obsidian's reading view.

> When choosing between wikilinks and Markdown links: use `[[wikilinks]]` for notes within the vault (Obsidian tracks renames automatically) and `[text](url)` for external URLs only.

## Internal Links (Wikilinks)

```markdown
[[Note Name]]                          Link to note
[[Note Name|Display Text]]             Custom display text
[[Note Name#Heading]]                  Link to heading
[[Note Name#^block-id]]                Link to block
[[#Heading in same note]]              Same-note heading link
```

Define a block ID by appending `^block-id` to any paragraph:

```markdown
This paragraph can be linked to. ^my-block-id
```

For lists and quotes, place the block ID on a separate line after the block:

```markdown
> A quote block

^quote-id
```

## Embeds

Prefix any wikilink with `!` to embed its content inline:

```markdown
![[Note Name]]                         Embed full note
![[Note Name#Heading]]                 Embed section
![[image.png]]                         Embed image
![[image.png|300]]                     Embed image with width
![[document.pdf#page=3]]               Embed PDF page
```

See [EMBEDS.md](references/EMBEDS.md) for audio, video, search embeds, and external images.

## Callouts

```markdown
> [!note]
> Basic callout.

> [!warning] Custom Title
> Callout with a custom title.

> [!faq]- Collapsed by default
> Foldable callout (- collapsed, + expanded).
```

Common types: `note`, `tip`, `warning`, `info`, `example`, `quote`, `bug`, `danger`, `success`, `failure`, `question`, `abstract`, `todo`.

See [CALLOUTS.md](references/CALLOUTS.md) for the full list with aliases, nesting, and custom CSS callouts.

## Properties (Frontmatter)

```yaml
---
title: My Note
date: 2024-01-15
tags:
  - project
  - active
aliases:
  - Alternative Name
cssclasses:
  - custom-class
---
```

Default properties: `tags` (searchable labels), `aliases` (alternative note names for link suggestions), `cssclasses` (CSS classes for styling).

See [PROPERTIES.md](references/PROPERTIES.md) for all property types, tag syntax rules, and advanced usage.

## Tags

```markdown
#tag                    Inline tag
#nested/tag             Nested tag with hierarchy
```

Tags can contain letters, numbers (not first character), underscores, hyphens, and forward slashes. Tags can also be defined in frontmatter under the `tags` property.

## Comments

```markdown
This is visible %%but this is hidden%% text.

%%
This entire block is hidden in reading view.
%%
```

## Obsidian-Specific Formatting

```markdown
==Highlighted text==                   Highlight syntax
```

## Math (LaTeX)

```markdown
Inline: $e^{i\pi} + 1 = 0$

Block:
$$
\frac{a}{b} = c
$$
```

## Diagrams (Mermaid)

````markdown
```mermaid
graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Do this]
    B -->|No| D[Do that]
```
````

To link Mermaid nodes to Obsidian notes, add `class NodeName internal-link;`.

## Footnotes

```markdown
Text with a footnote[^1].

[^1]: Footnote content.

Inline footnote.^[This is inline.]
```

## Complete Example

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

## Common Mistakes

### 1. Using Markdown links for internal vault notes

**NEVER** use standard Markdown links for notes inside the vault. Obsidian will not update them on rename, and they may break if the file moves.

**WHY:** Markdown links bypass Obsidian's link graph entirely, so renamed or moved notes silently break without any warning in the UI.

```markdown
<!-- BAD -->
[Meeting Notes](meeting-notes.md)

<!-- GOOD -->
[[Meeting Notes]]
```

Use `[[wikilinks]]` for all notes within the vault. Reserve `[text](url)` for external URLs only.

### 2. Using wikilink syntax for external URLs

**NEVER** wrap an external URL in `[[...]]` wikilink syntax. Obsidian resolves wikilinks against vault notes and will create a broken internal link instead of a hyperlink.

**WHY:** The URL becomes an unresolvable note reference in the graph, producing a dead link that cannot be opened in a browser.

```markdown
<!-- BAD -->
[[https://obsidian.md]]

<!-- GOOD -->
[Obsidian](https://obsidian.md)
```

### 3. Placing block IDs inline on list items

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

### 4. Embedding an entire note when only a section is needed

**NEVER** embed a full note with `![[Note]]` when only a specific section is relevant. This pulls in every heading and paragraph of the target note, flooding the reading view with unwanted content.

**WHY:** Embedding an entire large note makes the host note hard to read and causes unnecessary re-rendering whenever any part of the source note changes.

```markdown
<!-- BAD -->
![[Architecture Diagram]]

<!-- GOOD -->
![[Architecture Diagram#Overview]]
```

### 5. Using spaces in tags

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

### 6. Placing frontmatter after content

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

## References

- [Obsidian Flavored Markdown](https://help.obsidian.md/obsidian-flavored-markdown)
- [Internal links](https://help.obsidian.md/links)
- [Embed files](https://help.obsidian.md/embeds)
- [Callouts](https://help.obsidian.md/callouts)
- [Properties](https://help.obsidian.md/properties)
