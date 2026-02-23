---
plan_date: 2026-02-23
skill_name: nx-vite-integration
source_audit: .context/audits/nx-vite-integration-audit-2026-02-22.md
---

# Remediation Plan: nx-vite-integration

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 76/120 (63%) |
| **Current Grade** | F |
| **Target Score** | 96/120 (80%) |
| **Target Grade** | B |
| **Priority** | Critical |
| **Effort Estimate** | Large (L) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
|---|-------|----------|-----------|
| 1 | Progressive disclosure very weak (5/15) | Critical | D5 |
| 2 | Mindset + procedures weak (8/15) | Critical | D2 |
| 3 | Anti-pattern quality weak (8/15) | High | D3 |
| 4 | Pattern recognition low (6/10) | High | D7 |
| 5 | Large file (558 lines, 0 references) | Critical | D5 |
| 6 | Specification compliance low (10/15) | Medium | D4 |
| 7 | Freedom calibration low (10/15) | Medium | D6 |

This skill requires major restructuring - the audit recommends a major rewrite.

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5: 5/15 -> 12/15) - CRITICAL

**Files to create**:

1. **Create** `skills/nx-vite-integration/references/vite-config-patterns.md`:
   - Move detailed vite.config.ts examples
   - Document plugin configurations
   - Add framework-specific configs (React, Vue, Svelte)

2. **Create** `skills/nx-vite-integration/references/nx-vite-plugin-reference.md`:
   - Document nxViteTsPaths plugin
   - Document nxCopyAssetsPlugin
   - Add API reference

3. **Create** `skills/nx-vite-integration/references/library-mode-guide.md`:
   - Move library build configuration
   - Document dts plugin usage
   - Add external dependency handling

4. **Create** `skills/nx-vite-integration/references/vitest-integration.md`:
   - Move vitest configuration
   - Document test patterns
   - Add coverage configuration

5. **Create** `skills/nx-vite-integration/references/troubleshooting.md`:
   - Common issues and solutions
   - TypeScript path resolution problems
   - Build optimization tips

**File to modify**: `skills/nx-vite-integration/SKILL.md`

Reduce to ~200-250 lines with navigation structure:

```markdown
## Quick Reference

| Topic | Reference |
|-------|-----------|
| Vite Config Patterns | [references/vite-config-patterns.md](references/vite-config-patterns.md) |
| NX Vite Plugins | [references/nx-vite-plugin-reference.md](references/nx-vite-plugin-reference.md) |
| Library Mode | [references/library-mode-guide.md](references/library-mode-guide.md) |
| Vitest Integration | [references/vitest-integration.md](references/vitest-integration.md) |
| Troubleshooting | [references/troubleshooting.md](references/troubleshooting.md) |
```

### Phase 2: Mindset + Procedures (D2: 8/15 -> 12/15) - CRITICAL

**File**: `skills/nx-vite-integration/SKILL.md`

Complete workflow rewrite with deterministic steps:

````markdown
## Workflow

### Step 1: Install Dependencies

Preconditions: NX workspace exists, Node.js 18+

```bash
bunx nx add @nx/vite
bun add -d vite @vitejs/plugin-react
```

Expected: Vite and NX Vite plugin installed

### Step 2: Configure vite.config.ts

Preconditions: Dependencies installed

```ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  plugins: [react(), nxViteTsPaths()],
});
```

Expected: TypeScript paths resolve correctly

### Step 3: Update project.json

Preconditions: Vite config created

Add executor targets for build, serve, test.

Expected: `nx run <project>:build` succeeds

### Step 4: Verify Setup

```bash
bunx nx run <project>:build
bunx nx run <project>:test
```

Expected: Build and tests pass

### When NOT to Use

- Projects requiring Webpack-specific loaders
- Legacy browser support (IE11)
- Complex module federation setups (use @nx/webpack)
````

### Phase 3: Anti-Pattern Quality (D3: 8/15 -> 13/15)

**File**: `skills/nx-vite-integration/SKILL.md`

Add comprehensive anti-patterns:

````markdown
## Anti-Patterns

### NEVER: Skip nxViteTsPaths plugin in monorepos

**WHY**: TypeScript path aliases won't resolve across workspace packages.

**BAD**:
```ts
export default defineConfig({
  plugins: [react()],
});
```

**GOOD**:
```ts
export default defineConfig({
  plugins: [react(), nxViteTsPaths()],
});
```

### NEVER: Configure vitest in vite.config.ts without separate config

**WHY**: Mixing build and test config causes conflicts and slower builds.

**BAD**:
```ts
export default defineConfig({
  plugins: [react()],
  test: { /* vitest config */ },
});
```

**GOOD**:
```ts
// vite.config.ts
export default defineConfig({
  plugins: [react()],
});

// vitest.config.ts
export default defineConfig({
  plugins: [react()],
  test: { /* vitest config */ },
});
```

### NEVER: Bundle all dependencies in library mode

**WHY**: Defeats tree-shaking for consumers, increases bundle size.

**BAD**:
```ts
build: {
  lib: { /* ... */ },
  // No rollupOptions.external
}
```

**GOOD**:
```ts
build: {
  lib: { /* ... */ },
  rollupOptions: {
    external: ['react', 'react-dom', /^@my-org/],
  },
}
```
````

### Phase 4: Pattern Recognition (D7: 6/10 -> 9/10)

**File**: `skills/nx-vite-integration/SKILL.md`

Expand frontmatter and add trigger section:

```yaml
---
description: |
  Configure and integrate Vite build tool in Nx monorepo workspaces for 
  applications and libraries. Covers TypeScript path resolution, framework 
  plugins, asset handling, vitest configuration, library mode, and file 
  replacements.
  
  Triggers: "Add Vite", "Vite setup", "nx vite", "vitest config", 
  "vite.config.ts", "library mode", "React + Vite", "Vue + Vite"
---
```

```markdown
## Use When

- Setting up Vite in an NX app or library
- Configuring vitest for testing
- Migrating from Webpack to Vite
- Setting up library builds with dts plugin
- Fixing TypeScript path resolution issues
```

### Phase 5: Specification Compliance (D4: 10/15 -> 13/15)

**File**: `skills/nx-vite-integration/SKILL.md`

1. Verify frontmatter follows skill spec
2. Add explicit scope and intent
3. Remove ambiguous language

### Phase 6: Freedom Calibration (D6: 10/15 -> 13/15)

**File**: `skills/nx-vite-integration/SKILL.md`

Add constraint guidelines:

```markdown
## Constraint Guidelines

### Hard Constraints

- Always use nxViteTsPaths in monorepos
- Always externalize dependencies in library mode
- Always configure cache directories for vitest

### Flexible Choices

- Framework plugin choice (React, Vue, Svelte, etc.)
- CSS processor (PostCSS, Tailwind, vanilla)
- Test runner (vitest recommended, but jest possible)
```

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh nx-vite-integration --json
bunx @biomejs/biome check skills/nx-vite-integration/
bunx markdownlint-cli2 "skills/nx-vite-integration/**/*.md"
wc -l skills/nx-vite-integration/SKILL.md  # Should be < 300 lines
ls -la skills/nx-vite-integration/references/  # Should have 5 files
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 12/15 |
| D2 Mindset + Procedures | Score >= 12/15 |
| D3 Anti-Pattern Quality | Score >= 12/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| SKILL.md line count | < 300 (from 558) |
| References created | >= 4 files |
| Overall Score | >= 96/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | L | 3 hours |
| Phase 2: Procedures | M | 2 hours |
| Phase 3: Anti-patterns | M | 1.5 hours |
| Phase 4: Pattern Recognition | S | 30 min |
| Phase 5: Spec Compliance | S | 20 min |
| Phase 6: Freedom Calibration | S | 20 min |
| **Total** | **L** | **7+ hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/nx-vite-integration/
```

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with 6 phases - most detailed plan
- Identified as "Critical" priority with major restructuring needed
- Has Estimated Effort table, Dependencies, Rollback Plan
- Code examples in remediation steps are specific and actionable
