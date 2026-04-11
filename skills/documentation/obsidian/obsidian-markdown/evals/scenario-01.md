# Scenario 01: Create an Obsidian Note for Project Alpha

## User Prompt

The team is starting a new initiative called "Project Alpha". Create an Obsidian note for it.

The note should include:

- Metadata capturing the project title, one or more relevant labels, and a status field set to "in-progress"
- A prose introduction that references another note called "Meeting Notes"
- An embedded section from a separate note called "Architecture Diagram" — specifically its "Overview" section
- A callout that draws attention to an upcoming deadline on March 30th, treating it as something that warrants a warning

Write the complete note content to a file called `project-alpha.md`.

## Expected Behavior

1. Begin the file with a YAML frontmatter block (`---`) before any headings or body content
2. Reference "Meeting Notes" using `[[Meeting Notes]]` wikilink syntax, not a standard Markdown link
3. Embed the "Overview" section of "Architecture Diagram" using `![[Architecture Diagram#Overview]]` or similar section-scoped embed, not a full note embed
4. Use `> [!warning]` callout syntax (with the bracketed type on the blockquote line) for the deadline callout
5. Ensure any tags in frontmatter do not contain spaces (e.g., `#project-alpha` not `#project alpha`)

## Success Criteria

- **Frontmatter is first**: The file begins with a YAML frontmatter block (`---`) before any headings or body content
- **Wikilink used for internal note reference**: The note references Meeting Notes using `[[Meeting Notes]]` wikilink syntax, not `[Meeting Notes](meeting-notes.md)` or similar Markdown link
- **Section embed used for Architecture Diagram**: The embed uses `![[Architecture Diagram#Overview]]` or similar section-scoped embed form, not `![[Architecture Diagram]]` (full note embed)
- **Warning callout syntax correct**: The deadline callout uses `> [!warning]` syntax (with the bracketed type on the blockquote line)
- **Tags are valid (no spaces)**: Any tags in frontmatter or inline do not contain spaces (e.g., `#project-alpha` not `#project alpha`)

## Failure Conditions

- YAML frontmatter block appears after a heading or body content
- "Meeting Notes" is referenced using `[Meeting Notes](meeting-notes.md)` instead of `[[Meeting Notes]]`
- "Architecture Diagram" is embedded using `![[Architecture Diagram]]` (full note) instead of a section-scoped embed like `![[Architecture Diagram#Overview]]`
- Deadline callout uses a syntax other than `> [!warning]` (e.g., `> **Warning:**` or a generic blockquote)
- Any tag in frontmatter contains a space (e.g., `#project alpha`)
