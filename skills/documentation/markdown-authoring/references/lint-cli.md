# Markdownlint CLI Usage

## Installation

```bash
# markdownlint-cli2 (recommended)
npm install -D markdownlint-cli2

# Original markdownlint-cli
npm install -D markdownlint-cli
```

## Basic Usage

### Lint Files

```bash
# Lint all markdown files
markdownlint-cli2 "**/*.md"

# Lint specific files
markdownlint-cli2 README.md CONTRIBUTING.md

# Lint with pattern
markdownlint-cli2 "docs/**/*.md"
```

### Fix Issues Automatically

```bash
# Fix all auto-fixable issues
markdownlint-cli2 --fix "**/*.md"

# Fix specific files
markdownlint-cli2 --fix README.md
```

### Ignore Patterns

```bash
# Ignore node_modules and dist
markdownlint-cli2 "**/*.md" "#node_modules" "#dist"

# Multiple ignore patterns
markdownlint-cli2 "**/*.md" "#node_modules" "#**/vendor/**" "#CHANGELOG.md"
```

## Configuration

### Using Config Files

```bash
# Uses .markdownlint-cli2.jsonc if present
markdownlint-cli2 "**/*.md"

# Specify config file
markdownlint-cli2 --config custom-config.json "**/*.md"
```

### CLI Options

```bash
# Show help
markdownlint-cli2 --help

# Show version
markdownlint-cli2 --version

# Fix issues
markdownlint-cli2 --fix "**/*.md"

# Custom config
markdownlint-cli2 --config .markdownlint.json "**/*.md"

# No config (use defaults)
markdownlint-cli2 --no-config "**/*.md"
```

## Output Formats

### Default Output

```bash
markdownlint-cli2 "**/*.md"
# Output:
# README.md:23 MD013/line-length Line length [Expected: 80; Actual: 125]
# docs/api.md:45 MD040/fenced-code-language Fenced code blocks should have a language specified
```

### JSON Output

```bash
markdownlint-cli2 --config .markdownlint-cli2.jsonc "**/*.md"
```

`.markdownlint-cli2.jsonc`:
```jsonc
{
  "outputFormatters": [
    ["markdownlint-cli2-formatter-json"]
  ]
}
```

### Summary Output

```bash
# Summary at end
markdownlint-cli2 "**/*.md"
# Output includes:
# Found 15 error(s) in 5 file(s)
```

## Exit Codes

- `0` - No errors found
- `1` - Errors found
- `2` - Invalid command-line arguments

Use in CI/CD:

```bash
#!/bin/bash
markdownlint-cli2 "**/*.md"
exit_code=$?
if [ $exit_code -ne 0 ]; then
  echo "Markdown linting failed!"
  exit 1
fi
```

## NPM Scripts Integration

### package.json

```json
{
  "scripts": {
    "lint:md": "markdownlint-cli2 \"**/*.md\" \"#node_modules\"",
    "lint:md:fix": "markdownlint-cli2 --fix \"**/*.md\" \"#node_modules\"",
    "lint": "npm run lint:md && npm run lint:js",
    "format:md": "markdownlint-cli2 --fix \"**/*.md\" \"#node_modules\""
  }
}
```

Usage:

```bash
npm run lint:md
npm run lint:md:fix
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Lint Markdown

on: [push, pull_request]

jobs:
  markdown-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Lint markdown
        run: npm run lint:md
```

### GitLab CI

```yaml
markdown-lint:
  image: node:20
  stage: test
  script:
    - npm ci
    - npm run lint:md
  only:
    - merge_requests
    - main
```

### Pre-commit Hook

Using Husky + lint-staged:

**package.json:**
```json
{
  "lint-staged": {
    "*.md": "markdownlint-cli2 --fix"
  }
}
```

**.husky/pre-commit:**
```bash
#!/bin/sh
npx lint-staged
```

## Common Patterns

### Lint All Except Generated Files

```bash
markdownlint-cli2 "**/*.md" "#node_modules" "#dist" "#coverage" "#.next"
```

### Lint Only Documentation

```bash
markdownlint-cli2 "docs/**/*.md" "README.md" "CONTRIBUTING.md"
```

### Lint and Fix in One Command

```bash
markdownlint-cli2 --fix "**/*.md" "#node_modules" || true
```

The `|| true` prevents the script from failing in CI if there are non-fixable errors.

### Check Staged Files Only

```bash
# In pre-commit hook
git diff --cached --name-only --diff-filter=ACMR "*.md" | xargs markdownlint-cli2 --fix
```

## Monorepo Usage

### Root Configuration

**.markdownlint-cli2.jsonc:**
```jsonc
{
  "globs": [
    "**/*.md"
  ],
  "ignores": [
    "**/node_modules/**",
    "**/dist/**",
    "packages/*/CHANGELOG.md"
  ],
  "config": {
    "default": true,
    "MD013": { "line_length": 120 }
  }
}
```

### Package Scripts

**package.json (root):**
```json
{
  "scripts": {
    "lint:md": "markdownlint-cli2",
    "lint:md:fix": "markdownlint-cli2 --fix"
  }
}
```

**packages/app/package.json:**
```json
{
  "scripts": {
    "lint:md": "markdownlint-cli2 \"**/*.md\" \"#node_modules\""
  }
}
```

## Troubleshooting

### No Files Found

```bash
# Issue: Pattern doesn't match
markdownlint-cli2 "*.md"  # Only matches current directory

# Solution: Use recursive pattern
markdownlint-cli2 "**/*.md"
```

### Config Not Found

```bash
# Issue: Config file not detected
markdownlint-cli2 "**/*.md"

# Solution: Specify config explicitly
markdownlint-cli2 --config .markdownlint.json "**/*.md"
```

### Too Many Files

```bash
# Issue: Argument list too long
markdownlint-cli2 $(find . -name "*.md")

# Solution: Use glob pattern
markdownlint-cli2 "**/*.md"
```

### Performance Issues

```bash
# Issue: Slow on large repos

# Solution: Limit scope
markdownlint-cli2 "src/**/*.md" "docs/**/*.md"

# Or: Exclude large directories
markdownlint-cli2 "**/*.md" "#vendor" "#node_modules" "#.git"
```

## Best Practices

1. **Use Config Files** - Store configuration in `.markdownlint-cli2.jsonc`
2. **Integrate with CI** - Fail builds on linting errors
3. **Pre-commit Hooks** - Catch errors before commit
4. **Auto-fix When Possible** - Use `--fix` in development
5. **Ignore Generated Files** - Exclude `node_modules`, `dist`, etc.
6. **Version Lock** - Pin markdownlint-cli2 version in package.json
7. **Document Exceptions** - Comment why certain files are ignored
8. **Fail Fast** - Don't use `|| true` in CI pipelines

## Common Pitfalls

1. **Wrong Glob Pattern** - Use `"**/*.md"` not `*.md`
2. **Missing Quotes** - Always quote patterns: `"**/*.md"`
3. **Forgetting Ignores** - Don't forget to ignore `node_modules`
4. **No Exit Code Check** - Always check exit code in scripts
5. **Slow CI** - Exclude unnecessary directories
6. **Config Not Applied** - Verify config file location and name
7. **Auto-fix in CI** - Don't use `--fix` in CI, only in development
