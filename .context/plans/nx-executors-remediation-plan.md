---
plan_date: 2026-02-23
skill_name: nx-executors
source_audit: .context/audits/nx-executors-audit-2026-02-22.md
---

# Remediation Plan: nx-executors

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 101/120 (84%) |
| **Current Grade** | B |
| **Target Score** | 110/120 (92%) |
| **Target Grade** | A |
| **Priority** | Medium |
| **Effort Estimate** | Small (S) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
|---|-------|----------|-----------|
| 1 | Anti-pattern quality moderate (10/15) | Medium | D3 |
| 2 | Progressive disclosure needed (10/15) | Medium | D5 |
| 3 | No reference files (205 lines in SKILL.md) | Low | D5 |
| 4 | Mindset + procedures can improve (12/15) | Low | D2 |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3: 10/15 -> 14/15)

**File**: `skills/nx-executors/SKILL.md`

Add explicit anti-patterns with NEVER/WHY/BAD-GOOD format:

````markdown
## Anti-Patterns

### NEVER: Put business logic directly in executors

**WHY**: Executors should orchestrate tools, not implement logic. 
Violates single responsibility and makes testing harder.

**BAD**:
```ts
export default async function executor(schema: Schema) {
  // 200 lines of transformation logic
  const result = complexBusinessLogic(schema);
  return { success: true };
}
```

**GOOD**:
```ts
import { transform } from './lib/transform';
export default async function executor(schema: Schema) {
  const result = await transform(schema);
  return { success: true };
}
```

### NEVER: Ignore ExecutorContext outputs

**WHY**: NX relies on outputs for caching. Missing outputs break incremental builds.

**BAD**:
```ts
return { success: true };  // No outputs
```

**GOOD**:
```ts
return { 
  success: true,
  outputs: ['{workspaceRoot}/dist/{projectRoot}']
};
```

### NEVER: Use synchronous file operations in executors

**WHY**: Blocks event loop and degrades performance in parallel execution.

**BAD**:
```ts
const content = fs.readFileSync(path);
```

**GOOD**:
```ts
const content = await fs.promises.readFile(path);
```
````

Add 2+ more anti-patterns covering:
- Schema validation mistakes
- Incorrect dependencies specification

### Phase 2: Progressive Disclosure (D5: 10/15 -> 14/15)

**Files to create**:

1. **Create** `skills/nx-executors/references/executor-schema-design.md`:
   - Move detailed schema.json examples
   - Add validation patterns
   - Include complex schema examples (conditional, dependent schemas)

2. **Create** `skills/nx-executors/references/context-api-reference.md`:
   - Move ExecutorContext API details
   - Document available context properties
   - Add usage examples

**File to modify**: `skills/nx-executors/SKILL.md`

Add quick reference table:

```markdown
## Quick Reference

| Topic | Reference |
|-------|-----------|
| Schema Design | [references/executor-schema-design.md](references/executor-schema-design.md) |
| Context API | [references/context-api-reference.md](references/context-api-reference.md) |
```

### Phase 3: Mindset + Procedures (D2: 12/15 -> 14/15)

**File**: `skills/nx-executors/SKILL.md`

Enhance workflow with explicit entry/exit conditions:

```markdown
## Workflow

### Step 1: Define Schema
Preconditions: None
Actions: Create schema.json with properties, required fields
Exit: Schema file validates against JSON Schema

### Step 2: Implement Executor
Preconditions: Schema defined
Actions: Create executor.ts, implement default export
Exit: Executor compiles without errors

### Step 3: Register in generators.json
Preconditions: Executor implemented
Actions: Add executor entry with implementation path
Exit: `nx list @scope/plugin` shows executor

### Step 4: Test Executor
Preconditions: Executor registered
Actions: Run with test project
Exit: Expected outputs produced
```

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh nx-executors --json
bunx @biomejs/biome check skills/nx-executors/
bunx markdownlint-cli2 "skills/nx-executors/**/*.md"
ls -la skills/nx-executors/references/  # Should have 2+ files
```

## Success Criteria

- [ ] Re-run audit shows score >= 110/120 (grade A)
- [ ] D3 (Anti-Pattern Quality) >= 14/15
- [ ] D5 (Progressive Disclosure) >= 13/15
- [ ] At least 2 reference files created
- [ ] All verification commands pass
