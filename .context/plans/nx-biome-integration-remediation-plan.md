---
plan_date: 2026-02-23
skill_name: nx-biome-integration
source_audit: .context/audits/nx-biome-integration-audit-2026-02-22.md
---

# Remediation Plan: nx-biome-integration

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 85/120 (70%) |
| **Current Grade** | C |
| **Target Score** | 102/120 (85%) |
| **Target Grade** | B |
| **Priority** | High |
| **Effort Estimate** | Medium (S) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
|---|-------|----------|-----------|
| 1 | Anti-pattern quality weak (8/15) | High | D3 |
| 2 | Pattern recognition low (6/10) | High | D7 |
| 3 | Progressive disclosure needed (10/15) | Medium | D5 |
| 4 | Mindset + procedures unclear (10/15) | Medium | D2 |
| 5 | Large file size (383 lines) with only 1 reference | Medium | D5 |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3: 8/15 -> 13/15)

**File**: `skills/nx-biome-integration/SKILL.md`

1. Add explicit anti-patterns section with NEVER/WHY/BAD-GOOD format:

```markdown
## Anti-Patterns

### NEVER: Use nested biome.json for simple project setups

**WHY**: Nested configs increase maintenance burden and cause rule conflicts.

**BAD**:
- Create biome.json in every sub-project

**GOOD**:
- Use single root biome.json with overrides
- Use `include`/`exclude` patterns for project-specific rules
```

1. Add 3-5 repository-specific anti-patterns covering:
   - ESLint/Prettier coexistence conflicts
   - Incorrect formatter enablement in CI
   - Cache invalidation issues
   - Rule severity misconfigurations

### Phase 2: Pattern Recognition (D7: 6/10 -> 9/10)

**File**: `skills/nx-biome-integration/SKILL.md`

1. Expand frontmatter description with trigger keywords:

```yaml
---
description: |
  Integrate Biome (Rust-based linter/formatter) into NX monorepos with custom 
  executors, inferred tasks, and caching. Use proactively when setting up Biome 
  in NX workspaces, creating custom tool executors, or migrating from ESLint/Prettier to Biome.
  
  Triggers: "add Biome", "Biome setup", "NX Biome", "migrate to Biome", 
  "ESLint replacement", "Prettier alternative", "biome.json", "lint and format"
---
```

1. Add "Use When" section at top of SKILL.md:

```markdown
## Use When

- Setting up Biome in an NX workspace
- Migrating from ESLint/Prettier to Biome
- Creating NX executors for Biome
- Configuring Biome caching with NX
```

### Phase 3: Progressive Disclosure (D5: 10/15 -> 13/15)

**Files to create/modify**:

1. Create `skills/nx-biome-integration/references/biome-configuration-deep-dive.md`:
   - Move detailed configuration options from SKILL.md
   - Include advanced rule tuning examples
   - Add performance optimization tips

2. Create `skills/nx-biome-integration/references/migration-guide.md`:
   - ESLint to Biome migration steps
   - Prettier to Biome migration steps
   - Common pitfalls and solutions

3. Update SKILL.md to be navigation hub:
   - Keep only high-level workflow and commands
   - Link to references for deep details

### Phase 4: Mindset + Procedures (D2: 10/15 -> 13/15)

**File**: `skills/nx-biome-integration/SKILL.md`

1. Add explicit workflow with numbered steps:

````markdown
## Workflow

### Step 1: Install and Initialize

Preconditions: NX workspace exists, Node.js 18+

```bash
bunx nx add @nx-biome/nx
bunx biome init
```

Expected: `biome.json` created at workspace root

### Step 2: Configure Executor

...

### Step 3: Verify Setup

...
````

1. Add "When NOT to use" section

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with clear priorities (Critical, High, Medium)
- Has Estimated Effort table, Dependencies, Rollback Plan
- Code examples in remediation steps are specific and actionable
- Minor: Already comprehensive - just needs minor polish
