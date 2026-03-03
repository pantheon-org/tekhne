# Blockquotes and Other Elements

## Blockquotes

Use `>` for blockquotes:

```markdown
> This is a blockquote.
```

### Multi-Line Blockquotes

```markdown
> This is a blockquote
> that spans multiple lines.
```

Or use a single `>`:

```markdown
> This is a blockquote
that spans multiple lines.
```

### Nested Blockquotes

```markdown
> First level
>> Second level
>>> Third level
```

### Blockquotes with Other Elements

```markdown
> ## Heading in Blockquote
>
> This blockquote contains:
>
> - A list item
> - Another item
>
> And a code block:
>
> ```javascript
> console.log('Hello');
> ```
```

## Horizontal Rules

Create with three or more `-`, `*`, or `_`:

```markdown
---

***

___
```

## Line Breaks

### Hard Break

End line with two spaces:

```markdown
Line one with two spaces at end  
Line two
```

### Paragraph Break

Use blank line:

```markdown
First paragraph

Second paragraph
```

## Escaping Characters

Use backslash to escape special characters:

```markdown
\* Not italic \*
\# Not a heading
\[Not a link\](url)
```

Characters that can be escaped:
```
\ ` * _ { } [ ] ( ) # + - . ! |
```

## Tables

Basic table syntax:

```markdown
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
```

### Column Alignment

```markdown
| Left aligned | Center aligned | Right aligned |
|:-------------|:--------------:|--------------:|
| Left         | Center         | Right         |
```

### Complex Tables

```markdown
| Feature      | Description                    | Status |
|--------------|--------------------------------|--------|
| Authentication | OAuth 2.0 with JWT        | ✅     |
| Authorization  | Role-based access control | ✅     |
| Logging        | Structured JSON logs      | 🚧     |
```

## HTML in Markdown

### When to Use HTML

- Complex layouts not supported by Markdown
- Specific styling requirements
- Image sizing
- Embedded content

### Common HTML Elements

```markdown
<div align="center">
  <img src="logo.png" alt="Logo" width="200">
</div>

<details>
<summary>Click to expand</summary>

Hidden content here.

</details>

<kbd>Ctrl</kbd> + <kbd>C</kbd>

<sup>Superscript</sup> and <sub>subscript</sub>
```

## Best Practices

### Blockquotes

- Use for quotes, callouts, or highlighting important information
- Keep nested quotes to 2-3 levels maximum
- Add attribution for actual quotes

### Horizontal Rules

- Use sparingly to separate major sections
- Be consistent with style (prefer `---`)
- Surround with blank lines

### Line Breaks

- Use paragraph breaks (blank lines) for most cases
- Hard breaks for poetry, addresses, or specific formatting needs
- Avoid trailing spaces (can be invisible and cause issues)

### Tables

- Keep tables simple (use HTML or images for complex data)
- Align columns for readability in source
- Use column alignment for numeric data
- Consider accessibility (screen readers may struggle with complex tables)

### HTML

- Prefer Markdown over HTML when possible
- Validate HTML syntax
- Consider that some Markdown renderers may strip HTML
- Document when HTML is necessary

## Examples

### Good Blockquote

```markdown
> **Note:** This feature requires Node.js 18 or higher.

> **Warning:** Deleting this resource cannot be undone.
```

### Good Table

```markdown
| Command | Description              |
|---------|--------------------------|
| `start` | Starts the server        |
| `stop`  | Stops the server         |
| `test`  | Runs the test suite      |
```

### Good HTML Usage

```markdown
<details>
<summary>View detailed error log</summary>

```bash
Error: Connection timeout
  at connect (db.js:45)
  at async main (index.js:12)
```

</details>
```

## Linting Rules

- **MD009** (no-trailing-spaces): Trailing spaces
- **MD010** (no-hard-tabs): Hard tabs
- **MD026** (no-trailing-punctuation): Trailing punctuation in heading
- **MD027** (no-multiple-space-blockquote): Multiple spaces after blockquote symbol
- **MD028** (no-blanks-blockquote): Blank line inside blockquote
- **MD033** (no-inline-html): Inline HTML
- **MD035** (hr-style): Horizontal rule style
- **MD047** (single-trailing-newline): Files should end with a single newline character

## Configuration Example

```json
{
  "MD009": {
    "br_spaces": 2,
    "list_item_empty_lines": false,
    "strict": false
  },
  "MD010": {
    "code_blocks": true,
    "spaces_per_tab": 4
  },
  "MD026": {
    "punctuation": ".,;:!?。，；：！？"
  },
  "MD033": {
    "allowed_elements": ["details", "summary", "kbd", "sup", "sub", "br"]
  },
  "MD035": {
    "style": "consistent"
  }
}
```
