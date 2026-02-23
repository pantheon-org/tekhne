---
plan_date: 2026-02-23
skill_name: nx-workspace-patterns
source_audit: .context/audits/nx-workspace-patterns-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: nx-workspace-patterns

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 88/120 (73%) |
| **Current Grade** | C |
| **Target Score** | 102/120 (85%) |
| **Target Grade** | B |
| **Priority** | High |
| **Effort Estimate** | Medium (M) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
| --- | --- | --- | --- |
| 1 | Progressive disclosure weak (7/15) | High | D5 |
| 2 | Anti-pattern quality weak (8/15) | High | D3 |
| 3 | Mindset + procedures unclear (10/15) | Medium | D2 |
| 4 | Freedom calibration moderate (10/15) | Medium | D6 |
| 5 | Large file (457 lines, 0 references) | High | D5 |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5: 7/15 -> 13/15)

**Files to create**:

1. **Create** `skills/nx-workspace-patterns/references/project-graph-configuration.md`:
   - Move detailed nx.json configuration
   - Document implicit dependencies
   - Add graph visualization tips

2. **Create** `skills/nx-workspace-patterns/references/caching-strategies.md`:
   - Move cache configuration details
   - Document remote caching setup
   - Add cache optimization patterns

3. **Create** `skills/nx-workspace-patterns/references/project-boundaries.md`:
   - Move dependency constraint configuration
   - Document @nx/enforce-module-boundaries
   - Add tagging strategies

4. **Create** `skills/nx-workspace-patterns/references/affected-commands.md`:
   - Move affected command documentation
   - Document CI integration patterns
   - Add base/HEAD configuration

**File to modify**: `skills/nx-workspace-patterns/SKILL.md`

Reduce to navigation hub (~200 lines):

```markdown
## Quick Reference

| Topic | Reference |
|-------|-----------|
| Project Graph | [references/project-graph-configuration.md](references/project-graph-configuration.md) |
| Caching | [references/caching-strategies.md](references/caching-strategies.md) |
| Boundaries | [references/project-boundaries.md](references/project-boundaries.md) |
| Affected | [references/affected-commands.md](references/affected-commands.md) |
```

### Phase 2: Anti-Pattern Quality (D3: 8/15 -> 13/15)

**File**: `skills/nx-workspace-patterns/SKILL.md`

Add explicit anti-patterns:

````markdown
## Anti-Patterns

### NEVER: Create circular dependencies between projects

**WHY**: Breaks the build graph, causes incremental build failures.

**BAD**:
- libs/ui depends on libs/data
- libs/data depends on libs/ui

**GOOD**:
- Create shared libs/core for common types
- Use dependency injection for cross-cutting concerns

### NEVER: Skip affected:build in CI

**WHY**: Builds unrelated projects, wastes CI time, slows feedback.

**BAD**:
```yaml
- nx run-many --target=build --all
```

**GOOD**:
```yaml
- nx affected --target=build --base=origin/main
```

### NEVER: Tag projects inconsistently

**WHY**: Tags drive dependency constraints. Inconsistent tags bypass protections.

**BAD**:
- Some projects have `scope:frontend`, others `type:app`
- Mixed conventions within same workspace

**GOOD**:
- Define tag vocabulary in AGENTS.md
- Enforce via .eslintrc.json rules
- Use consistent prefix (scope:, type:, platform:)

### NEVER: Ignore task pipeline configuration

**WHY**: NX can't parallelize optimally without explicit dependencies.

**BAD**:
- No targetDefaults in nx.json
- Implicit build->test ordering

**GOOD**:
```json
{
  "targetDefaults": {
    "build": { "dependsOn": ["^build"] },
    "test": { "dependsOn": ["build"] }
  }
}
```
````

### Phase 3: Mindset + Procedures (D2: 10/15 -> 13/15)

**File**: `skills/nx-workspace-patterns/SKILL.md`

Add deterministic workflow:

```markdown
## Workflow

### Step 1: Structure Workspace

Preconditions: None
Actions:
- Create apps/ for deployable applications
- Create libs/ for shared libraries
- Create tools/ for scripts

Exit: Directory structure follows NX conventions

### Step 2: Configure Project Graph

Preconditions: Workspace structured
Actions:
- Add tags to each project.json
- Configure implicit dependencies in nx.json
- Set up targetDefaults

Exit: `nx graph` shows expected connections

### Step 3: Enforce Boundaries

Preconditions: Tags defined
Actions:
- Configure @nx/enforce-module-boundaries
- Add .eslintrc.json rules for tags
- Test with intentional violation

Exit: Invalid imports fail lint

### Step 4: Optimize CI

Preconditions: Boundaries enforced
Actions:
- Configure affected commands
- Set up caching (local and/or remote)
- Add CI workflow with affected targets

Exit: CI only runs affected tasks

### When NOT to Use

- Single-project repositories (no monorepo benefits)
- Projects with complex Webpack configurations (consider Turborepo)
- Teams new to monorepos (start simpler)
```

### Phase 4: Freedom Calibration (D6: 10/15 -> 13/15)

**File**: `skills/nx-workspace-patterns/SKILL.md`

Add constraint guidelines:

```markdown
## Constraint Guidelines

### Hard Constraints

- Always use tags for dependency constraints
- Always configure targetDefaults for pipelines
- Always use affected commands in CI

### Flexible Choices

- Directory structure (apps/libs vs packages)
- Tag naming convention (scope: vs type:)
- Remote caching provider (Nx Cloud, self-hosted)
- Number of libraries (granularity)

### Fallback Behaviors

| Missing Config | Fallback |
|----------------|----------|
| No tags | Allow all dependencies (warning) |
| No targetDefaults | Run tasks independently |
| No CI base | Default to main branch |
```

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh nx-workspace-patterns --json
bunx @biomejs/biome check skills/nx-workspace-patterns/
bunx markdownlint-cli2 "skills/nx-workspace-patterns/**/*.md"
wc -l skills/nx-workspace-patterns/SKILL.md  # Should be < 250 lines
ls -la skills/nx-workspace-patterns/references/  # Should have 4 files
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 12/15 |
| D3 Anti-Pattern Quality | Score >= 12/15 |
| D2 Mindset + Procedures | Score >= 12/15 |
| SKILL.md line count | < 250 (from 457) |
| References created | >= 4 files |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | M | 1.5 hours |
| Phase 2: Anti-patterns | M | 1 hour |
| Phase 3: Procedures | S | 45 min |
| Phase 4: Freedom Calibration | S | 30 min |
| **Total** | **M** | **4 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/nx-workspace-patterns/SKILL.md
```

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with 4 phases and clear priorities
- Has Estimated Effort table, Dependencies, Rollback Plan
- Code examples in remediation steps are specific and actionable
