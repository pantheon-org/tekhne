# Text Formatting

## Basic Formatting

### Italic

Use single asterisks or underscores:

```markdown
*italic text*
_italic text_
```

### Bold

Use double asterisks or underscores:

```markdown
**bold text**
__bold text__
```

### Bold and Italic

Combine them:

```markdown
***bold and italic***
___bold and italic___
**_bold and italic_**
```

### Strikethrough

Use double tildes:

```markdown
~~strikethrough text~~
```

## Inline Code

Use single backticks:

```markdown
Use the `console.log()` function.
```

For code with backticks inside:

```markdown
Use ``code with `backticks` inside``.
```

## Best Practices

- Be consistent with italic/bold markers throughout a document
- Use `*` for emphasis (italic) and `**` for strong emphasis (bold)
- Use underscores `_` only when marking up text within words
- Always use inline code for:
  - Function names, variable names, method names
  - File paths and filenames
  - Command-line commands
  - Package names
  - Configuration keys

## Examples

### Good

```markdown
The `getUserById()` function accepts an `id` parameter and returns a **User** object.

Install with `npm install express`.

Edit the `package.json` file in your project root.
```

### Bad

```markdown
The getUserById() function accepts an id parameter and returns a User object.

Install with npm install express.

Edit the package.json file in your project root.
```

## Linting Rules

- **MD036** (no-emphasis-as-heading): Emphasis used instead of a heading
- **MD037** (no-space-in-emphasis): Spaces inside emphasis markers
- **MD038** (no-space-in-code): Spaces inside code span elements
- **MD042** (no-empty-links): No empty links

## Configuration Example

```json
{
  "MD036": { "punctuation": ".,;:!?。，；：！？" },
  "MD037": true,
  "MD038": true
}
```
