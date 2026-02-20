# Markdown Headings

## Syntax

```markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6
```

## Rules

- Use only one H1 (`#`) per document (typically the title)
- Don't skip heading levels (don't go from H2 to H4)
- Add blank lines before and after headings
- Use ATX-style headings (`#`) not Setext-style (`===` or `---`)

## Best Practices

- Keep headings concise and descriptive
- Use sentence case (capitalize first word only)
- Don't end headings with punctuation
- Make headings scannable for quick navigation

## Examples

### Good

```markdown
# Project Title

## Installation

### Prerequisites

### Quick Start

## Usage
```

### Bad

```markdown
# Project Title
##### Prerequisites
## Installation
```

## Linting Rules

- **MD001** (heading-increment): Heading levels should only increment by one level at a time
- **MD003** (heading-style): Heading style should be consistent (`atx`, `atx_closed`, or `setext`)
- **MD018** (no-missing-space-atx): No space after hash on atx style heading
- **MD019** (no-multiple-space-atx): Multiple spaces after hash on atx style heading
- **MD020** (no-missing-space-closed-atx): No space inside hashes on closed atx style heading
- **MD021** (no-multiple-space-closed-atx): Multiple spaces inside hashes on closed atx style heading
- **MD022** (blanks-around-headings): Headings should be surrounded by blank lines
- **MD023** (heading-start-left): Headings must start at the beginning of the line
- **MD024** (no-duplicate-heading): Multiple headings with the same content
- **MD025** (single-h1): Multiple top-level headings in the same document
- **MD041** (first-line-heading/first-line-h1): First line in a file should be a top-level heading
- **MD043** (required-headings): Required heading structure

## Configuration Example

```json
{
  "MD003": { "style": "atx" },
  "MD022": { "lines_above": 1, "lines_below": 1 },
  "MD024": { "siblings_only": true },
  "MD025": { "front_matter_title": "" },
  "MD041": { "level": 1 }
}
```
