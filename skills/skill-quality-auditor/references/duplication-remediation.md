---
category: duplication
priority: HIGH
source: skill consolidation experience
---

# Duplication Remediation

Strategies for fixing content duplication across skills. Provides actionable approaches based on duplication type and severity.

## Remediation Overview

**Goal**: Reduce duplication to <5% across skill collection  
**Approach**: Consolidate, Extract, or Eliminate based on duplication type

## Remediation Strategies

### Strategy 1: Aggregation (Recommended)

**Use When:**

- Skills are in same domain/family
- >20% content overlap
- Users confused about which skill to use
- Maintenance burden high

**Process:**

1. Design category structure
2. Create navigation hub (SKILL.md)
3. Extract content to references/
4. Deprecate original skills
5. Verify no broken references

**Result:** 96%+ size reduction, <5% duplication

### Strategy 2: Extraction

**Use When:**

- Shared content across unrelated skills
- Common patterns worth centralizing
- Multiple skills reference same concept

**Process:**

1. Identify shared content
2. Create shared reference file
3. Update all skills to reference shared file
4. Remove duplicated content
5. Verify all references work

**Result:** Single source of truth for shared concept

### Strategy 3: Elimination

**Use When:**

- Content is truly redundant
- No value in keeping duplicate
- One version is superior

**Process:**

1. Identify best version
2. Verify no unique content in others
3. Delete inferior versions
4. Update any references
5. Document deletion reason

**Result:** Remove redundancy entirely

## Decision Matrix

| Scenario | Similarity | Related? | Action |
|----------|------------|----------|--------|
| Same family, >35% | High | Yes | Aggregation |
| Same family, 20-35% | Medium | Yes | Aggregation |
| Different family, >35% | High | No | Extraction |
| Different family, 20-35% | Medium | No | Review |
| Any, <20% | Low | N/A | Keep separate |

## Remediation Workflow

### Step 1: Analyze Duplication Report

```bash
cat .context/analysis/duplication-report-*.md
```

Identify:

- Pairs with >35% similarity (critical)
- Skill families with multiple overlaps
- Shared content patterns

### Step 2: Choose Strategy

For each high-duplication pair:

1. **Are they in the same domain?**
   - Yes → Aggregation
   - No → Extraction or Elimination

2. **Is content identical or just similar?**
   - Identical → Elimination
   - Similar → Aggregation or Extraction

3. **Would consolidation improve clarity?**
   - Yes → Aggregation
   - No → Keep separate, extract shared

### Step 3: Execute Remediation

**For Aggregation:**

1. Follow `aggregation-implementation.md` 6-step process
2. Create navigation hub
3. Move content to references
4. Deprecate originals

**For Extraction:**

1. Create `references/shared-topic.md`
2. Add frontmatter with source skills
3. Replace duplicated content with `@see` references
4. Verify all skills still work

**For Elimination:**

1. Delete inferior skill file
2. Move to `.deprecated/` for history
3. Update any cross-references
4. Document in deprecation README

### Step 4: Verify Remediation

```bash
# Re-run duplication detection
./scripts/detect-duplication.sh

# Verify no broken references
grep -r "@see" skills/*/SKILL.md | while read ref; do
  # Check referenced file exists
done

# Run skill-judge evaluation
find skills -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | while read -r skill_name; do
  sh skills/skill-quality-auditor/scripts/evaluate.sh "$skill_name" --json >/dev/null
done
```

### Step 5: Update Documentation

- Update AGENTS.md with new structure
- Add to deprecation README if applicable
- Update skill count metrics

## Common Remediation Patterns

### Pattern: Shared Anti-Patterns

**Problem:** Same anti-patterns repeated across skills

**Solution:** Create `references/shared-anti-patterns.md`

```markdown
---
category: shared
priority: CRITICAL
source: multiple skills
---

# Universal Anti-Patterns

## Never Trust Without Verification
...
```

Then reference from skills:

```markdown
## Anti-Patterns
See @see references/shared-anti-patterns.md for universal anti-patterns.
```

### Pattern: Duplicate Code Examples

**Problem:** Same code example in multiple skills

**Solution:** Extract to shared reference

### Pattern: Overlapping Concepts

**Problem:** Skills cover adjacent topics with overlap

**Solution:** Aggregate into single skill with categories

## Metrics After Remediation

Track improvement:

| Metric | Before | After | Target |
|--------|--------|-------|--------|
| Total duplication | 35% | <5% | <5% |
| Skill count | 50 | 35 | Optimize |
| Avg file size | 450 lines | 120 lines | <200 |
| Maintenance points | 3+ | 1 | 1 |

## Common Mistakes

❌ **Over-aggregating** - Consolidating unrelated skills  
❌ **Deleting without backup** - Always use .deprecated/  
❌ **Breaking references** - Verify all @see links  
❌ **Rushing verification** - Test thoroughly before deprecating

## See Also

- `duplication-detection-algorithm.md` - Finding duplicates
- `aggregation-pattern.md` - Navigation Hub pattern
- `aggregation-implementation.md` - Step-by-step process
