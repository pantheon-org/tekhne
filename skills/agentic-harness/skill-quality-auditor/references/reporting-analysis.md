---
category: reporting
priority: MEDIUM
source: skill audit analysis
---

# Audit Report Analysis

Guide to interpreting skill quality audit reports and taking action based on findings.

## Report Types

### Quality Report (skill-audit-*.md)

Contains:

- Overall skill collection statistics
- Individual skill scores and grades
- Grade distribution
- Recommendations by priority

### Duplication Report (duplication-report-*.md)

Contains:

- Skill pairs with similarity percentages
- High-priority consolidation candidates
- Common content patterns
- Remediation recommendations

### Aggregation Plan (aggregation-plan-*.md)

Contains:

- Source skills for consolidation
- Proposed category structure
- Estimated effort and ROI
- Implementation steps

## Analyzing Quality Reports

### Grade Distribution

```markdown
| Grade | Count | Percentage |
|-------|-------|------------|
| A     | 35    | 70%        |
| B     | 12    | 24%        |
| C     | 3     | 6%         |
```

**Interpretation:**

- Target: 90%+ A-grade
- Warning: Any C-grade skills
- Critical: D or F grades

### Score Analysis

```markdown
| Skill | Score | Grade | Lines |
|-------|-------|-------|-------|
| bdd-testing | 98/120 | A | 64 |
| typescript-advanced | 85/120 | B | 450 |
| old-patterns | 72/120 | C | 800 |
```

**Actions by Score:**

| Score | Priority | Action |
|-------|----------|--------|
| 108+ | Maintain | Keep current, monitor |
| 96-107 | Low | Minor improvements optional |
| 90-95 | Medium | Review for improvements |
| 78-89 | High | Requires attention |
| <78 | Critical | Immediate action needed |

### Common Score Patterns

**High Lines, Low Score (450, 72):**

- Problem: Overly verbose, not focused
- Solution: Progressive disclosure, extract to references

**Low Lines, Low Score (80, 75):**

- Problem: Missing content, incomplete
- Solution: Add missing dimensions (anti-patterns, examples)

**High Lines, High Score (450, 105):**

- Problem: Good content but needs reorganization
- Solution: Split into navigation hub + references

## Analyzing Duplication Reports

### Similarity Matrix

```markdown
| Skill Pair | Similarity | Common Lines | Action |
|------------|------------|--------------|--------|
| bdd-gherkin ↔ cucumber | 42% | 287 | Aggregate |
| typescript-* family | 28% avg | 156 | Consider |
```

**Action by Similarity:**

| Similarity | Priority | Action |
|------------|----------|--------|
| >50% | Critical | Immediate consolidation |
| 35-50% | High | Plan aggregation |
| 20-35% | Medium | Review for opportunity |
| <20% | Low | Monitor |

### Family Analysis

```markdown
## BDD Family (6 skills, 2,032 lines, 35% avg duplication)
- bdd-collaboration
- bdd-gherkin
- bdd-patterns
- bdd-principles
- bdd-scenarios
- cucumber-best-practices
```

**Actions:**

- Calculate consolidation ROI
- Plan aggregation implementation
- Schedule remediation

## Key Metrics to Track

### Quality Metrics

| Metric | Target | Calculation |
|--------|--------|-------------|
| A-grade rate | >90% | A-count / total |
| Average score | >100 | Total score / count |
| Max file size | <500 lines | wc -l SKILL.md |
| Description quality | 100% | With keywords / total |

### Duplication Metrics

| Metric | Target | Calculation |
|--------|--------|-------------|
| Average duplication | <5% | Sum similarity / pairs |
| Critical pairs (>35%) | 0 | Count where >35% |
| Aggregation candidates | <5 | Count where >20% |

### Maintenance Metrics

| Metric | Target | Calculation |
|--------|--------|-------------|
| Deprecated skills | Minimize | Count in .deprecated/ |
| Orphan references | 0 | Broken @see links |
| Outdated reports | <7 days | Age of latest audit |

## Action Planning

### Immediate Actions (Critical Issues)

1. **Address C-grade skills**
   - Read skill content
   - Identify missing dimensions
   - Add anti-patterns, examples
   - Re-evaluate

2. **Fix critical duplication**
   - Consolidate >35% pairs
   - Follow aggregation implementation
   - Verify no breakage

3. **Fix broken references**
   - Identify orphan @see links
   - Update or remove references
   - Verify skill still works

### Short-term Actions (1-2 weeks)

1. **Improve B-grade skills**
   - Add missing content
   - Improve descriptions
   - Add examples

2. **Plan aggregations**
   - Identify families
   - Design category structure
   - Estimate effort

3. **Update documentation**
   - Refresh AGENTS.md files
   - Update metrics
   - Document changes

### Long-term Actions (Monthly)

1. **Review skill collection**
   - Identify unused skills
   - Consider deprecation
   - Archive outdated content

2. **Update baselines**
   - Re-run full audit
   - Compare with previous
   - Track trends

3. **Team review**
   - Present findings
   - Get feedback
   - Prioritize improvements

## Report Comparison

### Tracking Changes Over Time

```bash
# Compare with previous audit
diff .context/analysis/skill-audit-2026-01-15.md \
     .context/analysis/skill-audit-2026-02-20.md
```

**Look for:**

- Grade changes (improvements/regressions)
- New skills added
- Skills removed/deprecated
- Score trends

### Trend Indicators

| Trend | Meaning |
|-------|---------|
| A-grade increasing | Quality improving |
| Average score rising | Skills getting better |
| Duplication decreasing | Consolidation working |
| File sizes shrinking | Progressive disclosure applied |

## Reporting to Stakeholders

### Executive Summary

```markdown
# Skill Quality Summary

## Overall Health: Good (87/100)

- **Skills**: 50 active, 10 deprecated
- **Quality**: 72% A-grade (target: 90%)
- **Duplication**: 12% (target: <5%)
- **Trend**: Improving (+5% from last quarter)

## Key Actions
1. Consolidate BDD family (6 → 1 skill)
2. Improve 8 B-grade skills to A
3. Reduce duplication from 12% to <5%
```

### Technical Details

Provide full reports for technical teams with:

- Complete score breakdowns
- All dimension scores
- Specific improvement recommendations
- Code examples for fixes

## See Also

- `reporting-dashboards.md` - Visualization
- `advanced-trends-analysis.md` - Historical tracking
- `scripts-audit-workflow.md` - Running audits
