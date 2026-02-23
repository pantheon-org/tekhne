---
plan_date: 2026-02-23
skill_name: extending-nx-plugins
source_audit: .context/audits/extending-nx-plugins-audit-2026-02-22.md
---

# Remediation Plan: extending-nx-plugins

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 88/120 (73%) | 108/120 (90%) |
| **Grade** | C | A |
| **Priority** | High | - |

**Verdict**: Priority improvements required. Critical: SKILL.md is 553 lines with 0 references - must extract content.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| SKILL.md too long (553 lines, 0 refs) | D5 (5/15) | **Critical** | Maintainability, discoverability |
| Weak anti-pattern precision | D3 (8/15) | High | Agents may repeat mistakes |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - CRITICAL PRIORITY

**Problem**: 553-line SKILL.md with zero reference files is unmaintainable.

**Files to create**:

1. **Create `skills/extending-nx-plugins/references/` directory**

2. **Extract to `references/generator-development.md`**:
   - All generator-related content
   - Tree API details
   - Schema validation patterns
   - Template file handling

3. **Extract to `references/executor-development.md`**:
   - All executor-related content
   - ExecutorContext API
   - Schema definitions
   - Best practices

4. **Extract to `references/migrations.md`**:
   - Migration patterns
   - Version handling
   - Breaking change management

5. **Extract to `references/testing-patterns.md`**:
   - Testing generators
   - Testing executors
   - Mock patterns

6. **Restructure SKILL.md as navigation hub**:

```markdown
---
name: extending-nx-plugins
description: Comprehensive guide for creating and managing Nx plugins including generators, inferred tasks, migrations, and best practices for extending Nx workspaces
---

# Extending Nx Plugins

## Purpose

Create and manage Nx plugins for workspace automation.

## When to Use

- Creating custom generators for code scaffolding
- Building executors for build tasks
- Implementing inferred tasks
- Writing migrations for breaking changes

## Quick Start

[Navigation links to detailed references]

## Core Concepts

[Concise overview - max 150 lines]

### Generators
See [Generator Development](references/generator-development.md) for full details.

### Executors
See [Executor Development](references/executor-development.md) for full details.

### Migrations
See [Migrations](references/migrations.md) for full details.

## Anti-Patterns

[Concise list with links to detailed examples in references]

## Quick Commands

[Essential commands only - detailed commands in references]

## Verification

[Standard verification section]
```

**Target SKILL.md length**: 150-200 lines max

### Phase 2: Anti-Pattern Quality (D3) - HIGH PRIORITY

**File**: `skills/extending-nx-plugins/SKILL.md` and `references/anti-patterns.md`

1. **Create `references/anti-patterns.md`** with detailed examples:

```markdown
# Nx Plugin Anti-Patterns

## Generator Anti-Patterns

### NEVER modify files outside tree.applyChanges
- **WHY**: Tree API tracks all changes for dry-run and rollback
- **BAD**: Using `fs.writeFileSync` directly
- **GOOD**: Using `tree.write(path, content)`
- **CONSEQUENCE**: Changes not tracked, dry-run fails, migrations break

### NEVER hardcode paths in generators
- **WHY**: Workspaces have different structures
- **BAD**: `tree.write('apps/my-app/src/main.ts', content)`
- **GOOD**: `tree.write(join(project.root, 'src', 'main.ts'), content)`

### NEVER skip schema validation
- **WHY**: Invalid inputs cause cryptic runtime errors
- **BAD**: Direct property access without validation
- **GOOD**: Use Zod or JSON Schema validation

## Executor Anti-Patterns

### NEVER return without success/failure
- **WHY**: Nx depends on return value for status
- **BAD**: `return;` or `return {};`
- **GOOD**: `return { success: true };`

### NEVER ignore cancellation signals
- **WHY**: Long-running tasks must support cancellation
- **CONSEQUENCE**: Stuck builds, resource leaks

## Migration Anti-Patterns

### NEVER assume package versions
- **WHY**: Users may have different versions installed
- **CONSEQUENCE**: Migration fails or corrupts workspace

### NEVER modify node_modules
- **WHY**: Changes lost on reinstall, may cause corruption
```

2. **Add concise anti-pattern section to SKILL.md**:

```markdown
## Anti-Patterns

| Pattern | Why | See Details |
| --- | --- | --- |
| Direct filesystem writes | Breaks tree tracking | [Anti-Patterns Reference](references/anti-patterns.md) |
| Hardcoded paths | Workspace structure varies | [Anti-Patterns Reference](references/anti-patterns.md) |
| Missing return values | Nx status tracking fails | [Anti-Patterns Reference](references/anti-patterns.md) |
| Unvalidated schemas | Runtime errors | [Anti-Patterns Reference](references/anti-patterns.md) |
```

### Phase 3: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/extending-nx-plugins/SKILL.md`

1. **Add explicit workflow with decision points**:

````markdown
## Development Workflow

### Step 1: Choose Plugin Type
- **Input**: User requirement
- **Decision**: 
  - Scaffolding code? -> Generator
  - Build/test task? -> Executor
  - Version migration? -> Migration
  - Auto-detection? -> Inferred Tasks
- **Output**: Plugin type confirmed

### Step 2: Generate Plugin Structure
```bash
nx g @nx/plugin:plugin {plugin-name}
```
- **Exit condition**: Plugin directory created

### Step 3: Implement Plugin
- **Generator path**: See [Generator Development](references/generator-development.md)
- **Executor path**: See [Executor Development](references/executor-development.md)
- **Migration path**: See [Migrations](references/migrations.md)

### Step 4: Add Tests
- See [Testing Patterns](references/testing-patterns.md)

### Step 5: Document
- Update README with usage examples
- Add to workspace generators/executors list

### Step 6: Verify
```bash
nx test {plugin-name}
nx build {plugin-name}
```
````

2. **Add scope control**:

```markdown
## Scope Control

### Use This Skill When
- Creating workspace-specific automation
- Building reusable build tasks
- Implementing code scaffolding
- Managing Nx workspace migrations

### Do NOT Use This Skill When
- Simple script execution (use npm scripts)
- One-off file changes (manual edit)
- External tool integration (use existing plugins)
```

## File Restructuring Summary

```
skills/extending-nx-plugins/
├── SKILL.md                    (150-200 lines, navigation hub)
├── AGENTS.md                   (optional deep navigation)
└── references/
    ├── generator-development.md
    ├── executor-development.md
    ├── migrations.md
    ├── testing-patterns.md
    └── anti-patterns.md
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh extending-nx-plugins --json

# Check file size reduction
wc -l skills/extending-nx-plugins/SKILL.md

# Verify references exist
ls -la skills/extending-nx-plugins/references/

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D5: Progressive Disclosure | Score increase | >= 13/15 |
| SKILL.md line count | Lines | <= 200 |
| Reference files | Count | >= 4 |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| Overall Score | Total points | >= 108/120 |
| Grade | Letter grade | >= A |

## Effort Estimate

- **T-shirt size**: L (8-12 hours)
- **Complexity**: High (significant restructure)
- **Risk**: Medium (content migration required)

## Dependencies

- Access to original skill content for extraction
- Understanding of Nx plugin architecture

## Notes

- Prioritize Phase 1 (progressive disclosure) - it's the root cause of other issues
- Consider creating AGENTS.md for plugin-type-specific navigation
- Template files could standardize generator/executor schemas
