# TypeDoc Configuration and Setup

TypeDoc generates API documentation from TypeScript source code and JSDoc comments. It produces clean, searchable HTML or Markdown documentation.

## Installation

```bash
npm install --save-dev typedoc

# Optional plugins
npm install --save-dev typedoc-plugin-markdown
npm install --save-dev typedoc-plugin-missing-exports
```

## Basic Configuration

Create `typedoc.json` in your project root:

```json
{
  "$schema": "https://typedoc.org/schema.json",
  "entryPoints": ["src/index.ts"],
  "out": "docs/api",
  "excludePrivate": true,
  "excludeProtected": false,
  "excludeInternal": true,
  "readme": "README.md",
  "plugin": []
}
```

## Configuration Options

### Entry Points

**Single entry point:**
```json
{
  "entryPoints": ["src/index.ts"]
}
```

**Multiple entry points:**
```json
{
  "entryPoints": [
    "src/index.ts",
    "src/auth/index.ts",
    "src/database/index.ts"
  ]
}
```

**Glob patterns:**
```json
{
  "entryPointStrategy": "expand",
  "entryPoints": ["src/**/*.ts"]
}
```

### Output Configuration

```json
{
  "out": "docs/api",
  "theme": "default",
  "name": "My Project API",
  "includeVersion": true,
  "disableSources": false,
  "sourceLinkTemplate": "https://github.com/user/repo/blob/{gitRevision}/{path}#L{line}"
}
```

### Visibility Control

```json
{
  "excludePrivate": true,        // Exclude private members
  "excludeProtected": false,     // Include protected members
  "excludeInternal": true,       // Exclude @internal tagged items
  "excludeExternals": true,      // Exclude external modules
  "excludeNotDocumented": false  // Include items without docs
}
```

### Module Organization

```json
{
  "categorizeByGroup": true,     // Group by @group tags
  "defaultCategory": "Other",    // Default category name
  "categoryOrder": [             // Category display order
    "Core",
    "Authentication",
    "Database",
    "*"
  ]
}
```

## Using @group and @category Tags

Organize documentation with JSDoc tags:

```typescript
/**
 * @group Authentication
 */
export class AuthService { }

/**
 * @group Database
 */
export class UserRepository { }

/**
 * @category Utilities
 */
export function formatDate(date: Date): string { }
```

## Markdown Output

For Markdown output (useful for static site generators):

```json
{
  "plugin": ["typedoc-plugin-markdown"],
  "out": "docs/api",
  "theme": "markdown",
  "entryDocument": "index.md",
  "hideInPageTOC": false
}
```

## Package.json Scripts

```json
{
  "scripts": {
    "docs": "typedoc",
    "docs:watch": "typedoc --watch",
    "docs:serve": "typedoc && npx http-server docs/api"
  }
}
```

## Advanced Configuration

### Custom Theme

```json
{
  "theme": "./custom-theme",
  "customCss": "./docs/custom.css"
}
```

### Navigation Configuration

```json
{
  "navigation": {
    "includeCategories": true,
    "includeGroups": true,
    "includeFolders": false
  },
  "navigationLinks": {
    "GitHub": "https://github.com/user/repo",
    "NPM": "https://www.npmjs.com/package/my-package"
  }
}
```

### Search Configuration

```json
{
  "searchInComments": true,
  "searchInDocuments": true,
  "searchGroupBoosts": {
    "Core": 2.0,
    "Authentication": 1.5
  }
}
```

## Monorepo Configuration

For monorepo setups with multiple packages:

```json
{
  "entryPoints": [
    "packages/core/src/index.ts",
    "packages/auth/src/index.ts",
    "packages/database/src/index.ts"
  ],
  "entryPointStrategy": "packages"
}
```

## Validation Configuration

```json
{
  "validation": {
    "notExported": true,     // Warn about non-exported types
    "invalidLink": true,     // Warn about invalid @link references
    "notDocumented": false   // Warn about undocumented items
  },
  "requiredToBeDocumented": [
    "Class",
    "Function",
    "Method"
  ]
}
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Generate Docs

on:
  push:
    branches: [main]

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm ci
      - run: npm run docs
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/api
```

### GitLab CI

```yaml
pages:
  stage: deploy
  script:
    - npm ci
    - npm run docs
    - mv docs/api public
  artifacts:
    paths:
      - public
  only:
    - main
```

## Performance Optimization

For large codebases:

```json
{
  "excludeNotDocumented": true,
  "excludePrivate": true,
  "excludeInternal": true,
  "cacheBust": true,
  "cleanOutputDir": true
}
```

## Complete Example Configuration

```json
{
  "$schema": "https://typedoc.org/schema.json",
  "name": "My Project API Documentation",
  "entryPoints": ["src/index.ts"],
  "entryPointStrategy": "resolve",
  "out": "docs/api",
  "theme": "default",
  "readme": "README.md",
  "includeVersion": true,
  "excludePrivate": true,
  "excludeProtected": false,
  "excludeInternal": true,
  "excludeNotDocumented": false,
  "categorizeByGroup": true,
  "defaultCategory": "Other",
  "categoryOrder": [
    "Core",
    "Authentication",
    "Database",
    "*"
  ],
  "searchInComments": true,
  "validation": {
    "notExported": true,
    "invalidLink": true,
    "notDocumented": false
  },
  "navigation": {
    "includeCategories": true,
    "includeGroups": true
  },
  "navigationLinks": {
    "GitHub": "https://github.com/user/repo",
    "NPM": "https://www.npmjs.com/package/my-package"
  },
  "plugin": ["typedoc-plugin-missing-exports"],
  "sourceLinkTemplate": "https://github.com/user/repo/blob/{gitRevision}/{path}#L{line}"
}
```

## Troubleshooting

**Issue: Types not appearing**
- Check `excludePrivate`, `excludeInternal` settings
- Ensure types are exported from entry point
- Use `--logLevel Verbose` for debugging

**Issue: Slow generation**
- Enable `excludeNotDocumented`
- Reduce entry points
- Use `--skipErrorChecking` for quicker builds

**Issue: Broken links**
- Enable `validation.invalidLink`
- Use `@link` instead of markdown links for type references
- Check that linked symbols are exported

## References

- [TypeDoc Documentation](https://typedoc.org/)
- [TypeDoc GitHub](https://github.com/TypeStrong/typedoc)
- [TypeDoc Plugins](https://typedoc.org/guides/plugins/)
- [Configuration Options](https://typedoc.org/options/)
