# Lists

## Unordered Lists

Use `-`, `*`, or `+` for unordered list items:

```markdown
- First item
- Second item
- Third item
```

### Nested Lists

Indent by 2 or 4 spaces:

```markdown
- First level
  - Second level
    - Third level
  - Back to second level
- Back to first level
```

## Ordered Lists

Use numbers followed by a period:

```markdown
1. First item
2. Second item
3. Third item
```

### Nested Ordered Lists

```markdown
1. First level
   1. Second level
   2. Another second level
2. Back to first level
```

## Task Lists

Use `- [ ]` for unchecked and `- [x]` for checked:

```markdown
- [x] Completed task
- [ ] Incomplete task
- [ ] Another incomplete task
```

## Multi-Paragraph List Items

Add blank lines and indent:

```markdown
- First item

  This is a continuation of the first item.

  Another paragraph in the first item.

- Second item
```

## Code Blocks in Lists

Indent the code block:

```markdown
- Install dependencies:

  ```bash
  npm install
  ```

- Run the server:

  ```bash
  npm start
  ```
```

## Best Practices

- Be consistent with list markers (`-` recommended for unordered lists)
- Use 2-space or 4-space indentation consistently
- Add blank lines between list items containing multiple paragraphs
- Align nested lists properly
- Use meaningful numbering (don't use `1.` for every item)

## Examples

### Good

```markdown
- Prerequisites:
  - Node.js 18+
  - PostgreSQL 14+
  - Redis 7+

- Installation steps:

  1. Clone the repository
  2. Install dependencies
  3. Configure environment variables
```

### Bad

```markdown
* Prerequisites:
- Node.js 18+
+ PostgreSQL 14+
  * Redis 7+

- Installation steps:
1. Clone the repository
2. Install dependencies
3. Configure environment variables
```

## Linting Rules

- **MD004** (ul-style): Unordered list style should be consistent
- **MD005** (list-indent): Inconsistent indentation for list items at the same level
- **MD006** (ul-start-left): Consider starting bulleted lists at the beginning of the line
- **MD007** (ul-indent): Unordered list indentation (default: 2 spaces)
- **MD029** (ol-prefix): Ordered list item prefix (one, ordered, one_or_ordered, zero)
- **MD030** (list-marker-space): Spaces after list markers (ul_single, ol_single, ul_multi, ol_multi)
- **MD032** (blanks-around-lists): Lists should be surrounded by blank lines

## Configuration Example

```json
{
  "MD004": { "style": "dash" },
  "MD007": { "indent": 2, "start_indented": false },
  "MD029": { "style": "ordered" },
  "MD030": {
    "ul_single": 1,
    "ol_single": 1,
    "ul_multi": 1,
    "ol_multi": 1
  },
  "MD032": { "lines_above": 1, "lines_below": 1 }
}
```
