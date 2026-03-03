# Markdownlint Rules Reference

## Core Rule Categories

### Document Structure Rules

**MD001 - heading-increment**
- Heading levels should increment by one level at a time
- Don't skip from H1 to H3

**MD022 - blanks-around-headings**
- Headings should be surrounded by blank lines

**MD025 - single-title/single-h1**
- Multiple top-level headings in the same document

**MD041 - first-line-heading/first-line-h1**
- First line in a file should be a top-level heading

### Heading Style Rules

**MD003 - heading-style**
```json
{
  "MD003": {
    "style": "atx"  // or "atx_closed", "setext", "setext_with_atx"
  }
}
```

**MD043 - required-headings**
- Required heading structure

### List Rules

**MD004 - ul-style**
```json
{
  "MD004": {
    "style": "dash"  // or "asterisk", "plus", "consistent"
  }
}
```

**MD005 - list-indent**
- Inconsistent indentation for list items at the same level

**MD007 - ul-indent**
```json
{
  "MD007": {
    "indent": 2,
    "start_indented": false,
    "start_indent": 2
  }
}
```

**MD029 - ol-prefix**
```json
{
  "MD029": {
    "style": "ordered"  // or "one", "zero", "one_or_ordered"
  }
}
```

**MD030 - list-marker-space**
- Spaces after list markers

**MD032 - blanks-around-lists**
- Lists should be surrounded by blank lines

### Code Block Rules

**MD031 - blanks-around-fences**
- Fenced code blocks should be surrounded by blank lines

**MD040 - fenced-code-language**
- Fenced code blocks should have a language specified

**MD046 - code-block-style**
```json
{
  "MD046": {
    "style": "fenced"  // or "indented", "consistent"
  }
}
```

### Line Length Rules

**MD013 - line-length**
```json
{
  "MD013": {
    "line_length": 120,
    "heading_line_length": 120,
    "code_blocks": false,
    "tables": false,
    "headings": true,
    "strict": false,
    "stern": false
  }
}
```

### Whitespace Rules

**MD009 - no-trailing-spaces**
```json
{
  "MD009": {
    "br_spaces": 2,
    "list_item_empty_lines": false,
    "strict": false
  }
}
```

**MD010 - no-hard-tabs**
- Hard tabs

**MD012 - no-multiple-blanks**
- Multiple consecutive blank lines

**MD027 - no-multiple-space-blockquote**
- Multiple spaces after blockquote symbol

**MD028 - no-blanks-blockquote**
- Blank line inside blockquote

### HTML Rules

**MD033 - no-inline-html**
```json
{
  "MD033": {
    "allowed_elements": ["br", "img", "a"]
  }
}
```

### Link and Image Rules

**MD034 - no-bare-urls**
- Bare URL used

**MD042 - no-empty-links**
- No empty links

**MD045 - no-alt-text**
- Images should have alternate text (alt text)

**MD052 - reference-links-images**
- Reference links and images should use a label that is defined

### Emphasis Rules

**MD036 - no-emphasis-as-heading**
- Emphasis used instead of a heading

**MD037 - no-space-in-emphasis**
- Spaces inside emphasis markers

**MD049 - emphasis-style**
```json
{
  "MD049": {
    "style": "underscore"  // or "asterisk", "consistent"
  }
}
```

**MD050 - strong-style**
```json
{
  "MD050": {
    "style": "asterisk"  // or "underscore", "consistent"
  }
}
```

### Other Rules

**MD014 - commands-show-output**
- Dollar signs used before commands without showing output

**MD019 - no-multiple-space-atx**
- Multiple spaces after hash on atx style heading

**MD023 - heading-start-left**
- Headings must start at the beginning of the line

**MD026 - no-trailing-punctuation**
```json
{
  "MD026": {
    "punctuation": ".,;:!。，；：！"
  }
}
```

**MD035 - hr-style**
```json
{
  "MD035": {
    "style": "---"  // or "***", "___", "consistent"
  }
}
```

**MD044 - proper-names**
```json
{
  "MD044": {
    "names": ["JavaScript", "TypeScript", "GitHub"],
    "code_blocks": false,
    "html_elements": false
  }
}
```

**MD047 - single-trailing-newline**
- Files should end with a single newline character

**MD048 - code-fence-style**
```json
{
  "MD048": {
    "style": "backtick"  // or "tilde", "consistent"
  }
}
```

## Common Rule Configurations

### Strict Documentation
```json
{
  "default": true,
  "MD013": { "line_length": 80, "code_blocks": false },
  "MD033": false,
  "MD041": true
}
```

### Relaxed Blog Style
```json
{
  "default": true,
  "MD013": false,
  "MD033": { "allowed_elements": ["br", "img", "iframe"] },
  "MD041": false
}
```

### Technical Documentation
```json
{
  "default": true,
  "MD013": { "line_length": 120, "code_blocks": false, "tables": false },
  "MD033": { "allowed_elements": ["br", "details", "summary"] },
  "MD040": true
}
```
