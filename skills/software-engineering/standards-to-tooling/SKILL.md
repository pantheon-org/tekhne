---
name: standards-to-tooling
description: >
  Translates project coding standards into concrete linting and formatting tool
  configurations. Given human-readable conventions (from AGENTS.md, code review
  guidelines, or team standards), this skill produces ESLint, Prettier, Biome,
  TypeScript, and other tool configs that enforce them automatically. Covers
  discovery, mapping, implementation, verification, and CI integration.
---

# Standards to Tooling

Maps project-specific coding conventions to linting/formatting tool configuration.
For the Stars project: JS/TS/Vue codebase with ESLint 10 + Prettier, migrating to
full TypeScript.

## Mindset

A convention that cannot be auto-enforced will drift. Before implementing any
standard, always ask: *Can a tool catch this?* If yes, configure it. If no,
document it as a code review checklist item.

Prefer tools that provide fix-on-save or `--fix` support — adoption is higher
when enforcement is automatic rather than manual.

## When to Use

- Onboarding a new project and need to set up linting/formatting from scratch
- Adding a new code convention that should be machine-enforced
- Reviewing a PR where a convention was violated and a tool should catch it
- Migrating a codebase (e.g., JS → TS) and need to update or add tooling
- Standardising tooling across multiple projects or monorepo workspaces

## Workflow

### 1. Discovery — Extract actionable standards

Source the project's conventions from:

| Source | What to look for |
|--------|-----------------|
| `AGENTS.md` | Naming, import style, component conventions, file structure |
| `.github/CONTRIBUTING.md` | PR requirements, commit style |
| Existing configs | Patterns already configured (eslint.config.js, .prettierrc, tsconfig.json) |
| Code review history | Repeated comments about the same issue |
| Project language docs | Best practices for the language/framework (Vue 3 style guide, etc.) |

For each convention, classify it:

| Category | Example | Tool |
|----------|---------|------|
| **Naming** | kebab-case files, PascalCase components | ESLint, lint-staged |
| **Imports** | type imports, `.js` extensions, ordering | ESLint, Prettier plugins |
| **Formatting** | single quotes, trailing commas, print width | Prettier |
| **Types** | no implicit any, strict null checks | TypeScript `tsconfig.json` |
| **Unused code** | no unused vars, params, imports | ESLint |
| **Style** | no-var, prefer-const, no console.log | ESLint |
| **Vue** | `<script setup>` only, component name casing | eslint-plugin-vue |

### 2. Mapping — Convention → Rule

Use this reference for the Stars project's tech stack:

#### JavaScript / TypeScript (ESLint)

| Convention | ESLint rule / plugin |
|---|---|
| Consistent type imports | `@typescript-eslint/consistent-type-imports: ["error", { prefer: "type-imports" }]` |
| No unused variables | `@typescript-eslint/no-unused-vars: ["warn", { argsIgnorePattern: "^_", varsIgnorePattern: "^_" }]` |
| No `var` | `no-var: "error"` |
| Prefer `const` | `prefer-const: "error"` |
| Named exports only | `import/no-default-export: "error"` (eslint-plugin-import) |
| No console.log (except warn/error) | `no-console: ["warn", { allow: ["warn", "error"] }]` |
| Max params | `max-params: ["warn", 3]` |
| Complexity gate | `complexity: ["warn", 10]` |
| Explicit `.js` extensions in imports | Custom or `import/extensions` rule |
| JSDoc required on public functions | `jsdoc/require-jsdoc` (eslint-plugin-jsdoc) |

#### Vue SFCs (eslint-plugin-vue)

| Convention | Rule |
|---|---|
| PascalCase component names in templates | `vue/component-name-in-template-casing: ["error", "PascalCase"]` |
| No unused components | `vue/no-unused-components: "warn"` |
| `<script setup>` only (no Options API) | `vue/component-api-style: ["error", ["script-setup"]]` |
| Multi-word component names (per Vue 3 style guide) | `vue/multi-word-component-names: "error"` |
| No `v-html` (XSS risk) | `vue/no-v-html: "warn"` |
| Attribute ordering convention | `vue/attributes-order` |
| Require `:key` in `v-for` | `vue/require-v-for-key` |

#### Formatting (Prettier)

| Convention | Prettier option |
|---|---|
| Single quotes | `singleQuote: true` |
| Trailing commas | `trailingComma: "all"` |
| 100 char print width | `printWidth: 100` |
| Import ordering | `@ianvs/prettier-plugin-sort-imports` with `importOrder` config |
| No semicolons (if desired) | `semi: false` |
| 2-space indent | `tabWidth: 2` (default) |

#### TypeScript (tsconfig.json)

| Convention | Compiler option |
|---|---|
| Strict null checks | `strictNullChecks: true` |
| No implicit any | `noImplicitAny: true` |
| Strict mode (all) | `strict: true` |
| No unchecked indexed access | `noUncheckedIndexedAccess: true` |
| Force consistent casing in imports | `forceConsistentCasingInFileNames: true` |

### 3. Implementation — Generate config

When creating or updating config files, follow these principles:

**ESLint flat config** (eslint.config.js):
- Use ESM (`export default`), not `.eslintrc`
- Organise by file glob: separate JS/TS blocks from Vue SFC blocks
- Use the `files` array to scope rules correctly
- Vue SFCs need `vue-eslint-parser` as outer parser with TS parser inside

**Prettier config** (.prettierrc):
- Keep it minimal — formatting rules belong here, logic rules belong in ESLint
- Use `prettier-plugin-sort-imports` for import order when ESLint import plugin is not used
- Pair with `format` and `format:check` npm scripts

**TypeScript config** (tsconfig.json):
- Separate `tsconfig.json` (IDE/typecheck) from Vite config
- Set `"noEmit": true` when using Vite for bundling
- Enable `strict: true` as a baseline, relax specific checks only with documented justification

### 4. Verification — Tooling checklist

After implementing, verify with:

```bash
# Lint all source files
npx eslint 'web/src/**/*.{ts,js,vue}'

# Check formatting (without writing)
npx prettier --check 'web/src/**/*.{ts,vue,css}'

# TypeScript type check
npx vue-tsc --noEmit

# Fix all auto-fixable issues
npx eslint --fix 'web/src/**/*.{ts,js,vue}'
npx prettier --write 'web/src/**/*.{ts,vue,css}'
```

### 5. CI Integration

Ensure lint/format/typecheck runs in CI:

```
name: Lint & Typecheck
steps:
  - run: npm run format:check   # Prettier
  - run: npm run lint           # ESLint
  - run: npm run typecheck      # vue-tsc
```

Add to `package.json` scripts:
```json
{
  "lint": "eslint 'web/src/**/*.{ts,js,vue}'",
  "lint:fix": "eslint --fix 'web/src/**/*.{ts,js,vue}'",
  "format": "prettier --write 'web/src/**/*.{ts,vue,css}'",
  "format:check": "prettier --check 'web/src/**/*.{ts,vue,css}'",
  "typecheck": "vue-tsc --noEmit -p web/tsconfig.json"
}
```

Consider adding pre-commit hooks (lefthook, husky + lint-staged) to catch
issues before they reach CI.

## Language Quick Reference

When the project's language differs from Stars (JS/TS/Vue), use these mappings:

| Language | Linter | Formatter | Type System |
|----------|--------|-----------|-------------|
| JavaScript/TypeScript | ESLint | Prettier / Biome | TypeScript |
| Python | Ruff | Ruff | mypy / pyright |
| Rust | Clippy | rustfmt | — |
| Go | golangci-lint | gofmt / gofumpt | — |
| Java | Checkstyle / PMD | Spotless | — |
| Ruby | RuboCop | RuboCop | Sorbet / RBS |
| Kotlin | detekt | ktlint | — |
| Swift | SwiftLint | swift-format | — |

## Never

- Never add a tool that the project doesn't already use without asking — introducing a new linter/formatter is a team decision
- Never change `prettier` formatting options without verifying the change across the full codebase (formatting wars are expensive)
- Never override a tool's default without a documented code convention that justifies it
- Never configure ESLint rules that conflict with Prettier rules (use `eslint-config-prettier` to disable style rules ESLint handles)
- Never add a rule that produces noise — warnings that are always ignored reduce trust in the tool
- Never skip documenting *why* a rule exists — future maintainers need to know if the convention still applies

## References

- [Vue 3 Style Guide](https://vuejs.org/style-guide/)
- [ESLint Flat Config docs](https://eslint.org/docs/latest/use/configure/configuration-files)
- [typescript-eslint rules](https://typescript-eslint.io/rules/)
- [Prettier options](https://prettier.io/docs/en/options.html)
- [Biome (alternative to ESLint + Prettier for JS/TS)](https://biomejs.dev/)
- Related skills: `vault-capture` (persist tooling decisions as semantic memories)
