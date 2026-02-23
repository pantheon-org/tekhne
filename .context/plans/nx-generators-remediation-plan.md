---
plan_date: 2026-02-23
skill_name: nx-generators
source_audit: .context/audits/nx-generators-audit-2026-02-22.md
---

# Remediation Plan: nx-generators

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 96/120 (80%) |
| **Current Grade** | B |
| **Target Score** | 108/120 (90%) |
| **Target Grade** | A |
| **Priority** | Medium |
| **Effort Estimate** | Medium (S) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
|---|-------|----------|-----------|
| 1 | Anti-pattern quality weak (8/15) | High | D3 |
| 2 | Freedom calibration moderate (10/15) | Medium | D6 |
| 3 | Progressive disclosure needed (10/15) | Medium | D5 |
| 4 | No reference files (288 lines in SKILL.md) | Low | D5 |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3: 8/15 -> 14/15)

**File**: `skills/nx-generators/SKILL.md`

Add comprehensive anti-patterns:

````markdown
## Anti-Patterns

### NEVER: Modify files outside the Tree API

**WHY**: Tree API tracks changes for dry-run mode and rollback. 
Direct file system modifications bypass safety mechanisms.

**BAD**:
```ts
import { writeFileSync } from 'fs';
writeFileSync('/path/to/file.ts', content);
```

**GOOD**:
```ts
tree.write('/path/to/file.ts', content);
```

### NEVER: Hardcode paths in generators

**WHY**: Breaks when workspace structure changes. Use context properties.

**BAD**:
```ts
tree.write('libs/my-lib/src/index.ts', content);
```

**GOOD**:
```ts
const { projectRoot } = context;
tree.write(join(projectRoot, 'src/index.ts'), content);
```

### NEVER: Skip schema validation

**WHY**: Invalid inputs cause cryptic errors later. Schema provides clear error messages.

**BAD**:
```ts
export default async function (tree: Tree, schema: any) {
  // No validation
}
```

**GOOD**:
```ts
export default async function (tree: Tree, schema: Schema) {
  validateSchema(schema);  // Or use Zod/class-validator
}
```

### NEVER: Generate without respecting project boundaries

**WHY**: Violates NX project graph and causes circular dependencies.

**BAD**:
```ts
// Generator in libs/ui adds imports to libs/data
```

**GOOD**:
```ts
// Verify dependency direction with project graph
// Use --dry-run to preview changes
```
````

### Phase 2: Freedom Calibration (D6: 10/15 -> 13/15)

**File**: `skills/nx-generators/SKILL.md`

Add explicit flexibility guidelines:

```markdown
## Constraint vs Flexibility Guidelines

### Hard Constraints (MUST)

- Always use Tree API for file operations
- Always validate schema inputs
- Always respect project.json structure
- Never bypass the NX graph

### Flexible Choices (CAN)

- Template engine choice (EJS, Handlebars, plain strings)
- Schema validation library (Zod, class-validator, JSON Schema)
- File naming conventions (can be customized via schema options)
- Directory structure (configurable via options)

### Fallback Behaviors

| Missing Input | Fallback |
|---------------|----------|
| No project name | Derive from current directory |
| No directory | Use project default |
| No template | Generate minimal boilerplate |
```

### Phase 3: Progressive Disclosure (D5: 10/15 -> 13/15)

**Files to create**:

1. **Create** `skills/nx-generators/references/tree-api-reference.md`:
   - Complete Tree API documentation
   - Method signatures and examples
   - Common patterns

2. **Create** `skills/nx-generators/references/schema-design-patterns.md`:
   - Advanced schema patterns
   - Conditional schemas
   - Dependent fields

3. **Create** `skills/nx-generators/references/template-engine-guide.md`:
   - Template syntax reference
   - EJS examples
   - Reusable template libraries

**File to modify**: `skills/nx-generators/SKILL.md`

Reduce to navigation hub format with quick reference table.

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh nx-generators --json
bunx @biomejs/biome check skills/nx-generators/
bunx markdownlint-cli2 "skills/nx-generators/**/*.md"
ls -la skills/nx-generators/references/  # Should have 3 files
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D6 Freedom Calibration | Score >= 13/15 |
| D5 Progressive Disclosure | Score >= 12/15 |
| References created | >= 3 files |
| Overall Score | >= 108/120 (A) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | M | 1.5 hours |
| Phase 2: Freedom Calibration | S | 45 min |
| Phase 3: Disclosure | S | 45 min |
| **Total** | **M** | **3 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/nx-generators/SKILL.md
```

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Good baseline score (B, targeting A)
- Has Estimated Effort table, Dependencies, Rollback Plan
- Code examples in remediation steps are specific and actionable
