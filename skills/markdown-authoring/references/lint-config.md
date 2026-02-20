# Markdownlint Configuration

## Installation

```bash
# npm
npm install -D markdownlint-cli2

# pnpm
pnpm add -D markdownlint-cli2

# yarn
yarn add -D markdownlint-cli2
```

## Configuration Files

### .markdownlint.json

Basic JSON configuration:

```json
{
  "$schema": "https://raw.githubusercontent.com/DavidAnson/markdownlint/main/schema/markdownlint-config-schema.json",
  "default": true,
  "MD003": { "style": "atx" },
  "MD004": { "style": "dash" },
  "MD007": { "indent": 2 },
  "MD013": {
    "line_length": 120,
    "code_blocks": false,
    "tables": false
  },
  "MD033": {
    "allowed_elements": ["br", "img", "details", "summary"]
  },
  "MD041": true
}
```

### .markdownlint.jsonc

JSONC allows comments:

```jsonc
{
  "$schema": "https://raw.githubusercontent.com/DavidAnson/markdownlint/main/schema/markdownlint-config-schema.json",
  // Enable all rules by default
  "default": true,
  
  // Use ATX-style headings (# Heading)
  "MD003": { "style": "atx" },
  
  // Use dashes for unordered lists
  "MD004": { "style": "dash" },
  
  // Allow longer lines for code blocks
  "MD013": {
    "line_length": 120,
    "code_blocks": false
  }
}
```

### .markdownlint.yaml

YAML configuration:

```yaml
# Enable all rules by default
default: true

# Heading style
MD003:
  style: atx

# Unordered list style
MD004:
  style: dash

# Line length
MD013:
  line_length: 120
  code_blocks: false
  tables: false

# Allowed HTML elements
MD033:
  allowed_elements:
    - br
    - img
    - details
    - summary
```

### .markdownlint.cjs (JavaScript)

```javascript
module.exports = {
  default: true,
  MD003: { style: 'atx' },
  MD004: { style: 'dash' },
  MD013: {
    line_length: 120,
    code_blocks: false,
    tables: false
  },
  MD033: {
    allowed_elements: ['br', 'img', 'details', 'summary']
  }
};
```

## Configuration Hierarchy

Markdownlint searches for configuration in this order:

1. `.markdownlint-cli2.jsonc` (CLI-specific)
2. `.markdownlint-cli2.yaml`
3. `.markdownlint-cli2.cjs`
4. `.markdownlint.jsonc` (markdownlint rules)
5. `.markdownlint.json`
6. `.markdownlint.yaml`
7. `.markdownlint.yml`
8. `.markdownlint.cjs`

## Configuration Inheritance

### Extending Base Config

```json
{
  "extends": "../.markdownlint.json",
  "MD013": false
}
```

### Package-based Extends

```json
{
  "extends": "markdownlint-config-standard"
}
```

## CLI-Specific Configuration

### .markdownlint-cli2.jsonc

```jsonc
{
  "config": {
    // Markdownlint rules configuration
    "default": true,
    "MD013": { "line_length": 120 }
  },
  "globs": [
    "**/*.md"
  ],
  "ignores": [
    "**/node_modules/**",
    "**/dist/**",
    "CHANGELOG.md"
  ],
  "fix": false,
  "outputFormatters": [
    [
      "markdownlint-cli2-formatter-default"
    ]
  ]
}
```

## Inline Configuration

### Disable All Rules for File

```markdown
<!-- markdownlint-disable-file -->
# This file is not linted
```

### Disable Specific Rules

```markdown
<!-- markdownlint-disable MD013 MD033 -->
This line can be very long and contain <span>HTML</span>.
<!-- markdownlint-enable MD013 MD033 -->
```

### Disable for Single Line

```markdown
This is a normal line.
<!-- markdownlint-disable-next-line MD013 -->
This is a very long line that would normally trigger MD013 but is explicitly allowed.
```

Or:

```markdown
This is a very long line that would normally trigger MD013 but is explicitly allowed. <!-- markdownlint-disable-line MD013 -->
```

### Capture and Restore

```markdown
<!-- markdownlint-capture -->
<!-- markdownlint-disable MD013 -->
Long content here...
<!-- markdownlint-restore -->
Normal linting resumes.
```

### Configure Inline

```markdown
<!-- markdownlint-configure-file { "MD013": { "line_length": 150 } } -->
# Rest of file uses 150 character line length
```

## Common Configuration Patterns

### Strict Documentation

```json
{
  "default": true,
  "MD013": {
    "line_length": 80,
    "code_blocks": false
  },
  "MD033": false,
  "MD041": true,
  "MD046": { "style": "fenced" },
  "MD048": { "style": "backtick" }
}
```

### Relaxed Blog Style

```json
{
  "default": true,
  "MD013": false,
  "MD033": {
    "allowed_elements": ["br", "img", "iframe", "div", "span"]
  },
  "MD041": false
}
```

### Technical README

```json
{
  "default": true,
  "MD013": {
    "line_length": 120,
    "code_blocks": false,
    "tables": false
  },
  "MD033": {
    "allowed_elements": ["br", "details", "summary", "img"]
  },
  "MD024": { "siblings_only": true },
  "MD041": true
}
```

### Monorepo Configuration

**Root .markdownlint.json:**
```json
{
  "default": true,
  "MD013": { "line_length": 100 }
}
```

**packages/docs/.markdownlint.json:**
```json
{
  "extends": "../../.markdownlint.json",
  "MD013": { "line_length": 120 }
}
```

## Per-Directory Configuration

```
project/
├── .markdownlint.json          # Base config
├── docs/
│   ├── .markdownlint.json      # Overrides for docs
│   └── api.md
└── blog/
    ├── .markdownlint.json      # Overrides for blog
    └── post.md
```

## Schema Validation

Always include the schema for IDE support:

```json
{
  "$schema": "https://raw.githubusercontent.com/DavidAnson/markdownlint/main/schema/markdownlint-config-schema.json",
  "default": true
}
```

This enables:
- Autocomplete in VS Code and other editors
- Validation of rule names and parameters
- Documentation on hover

## Best Practices

1. **Start with Default** - Enable `"default": true` and disable specific rules
2. **Use Schema** - Always include `$schema` for IDE support
3. **Document Exceptions** - Comment why rules are disabled
4. **Version Control** - Commit configuration files to repo
5. **Extend Base Config** - Use `extends` for shared configurations
6. **Per-Project Tuning** - Adjust line length and HTML rules per project type
7. **Inline Sparingly** - Prefer fixing issues over disabling rules inline

## Common Pitfalls

1. **JSON Syntax Errors** - Use JSONC for comments
2. **Wrong Rule Names** - Use schema validation to catch typos
3. **Conflicting Rules** - Some rules conflict (e.g., MD046 and MD031)
4. **Missing Schema** - Without schema, no autocomplete or validation
5. **Incorrect Paths** - In monorepos, verify `extends` paths are correct
6. **Over-configuration** - Don't disable rules without good reason
