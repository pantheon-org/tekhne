# Remediation Planning Guide

Guide for creating remediation plans from skill quality audit results.

## When to Use This Guide

Use this guide when:

- A skill has been audited and scored below target (typically <108/120, grade B or lower)
- You need to create an actionable plan to improve skill quality
- Converting audit findings into structured improvement tasks

## Prerequisites

1. **Completed Audit**: Run `sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json`
2. **Audit Report**: Location should be `.context/audits/<skill-name>-audit-YYYY-MM-DD.md`
3. **Template**: Load `skills/skill-quality-auditor/templates/remediation-plan-template.yaml`

## Standard Remediation Plan Format

All remediation plans follow this structure:

### Executive Summary

| Field | Description |
|-------|-------------|
| Current Score | Actual score from audit (e.g., 84/120, 70%) |
| Target Score | Desired score (typically +15-20 points) |
| Current/Target Grade | Letter grade (F/D/C/B/A) |
| Priority | Critical/High/Medium/Low |
| Effort | S/M/L estimate |

**Focus Areas**: Top 2-3 dimensions requiring attention

**Verdict**: One-line summary: "Priority improvements required" vs "Targeted improvements recommended"

### Critical Issues Table

Identify the top 3-5 issues from the audit:

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Description | D# (score/max) | Critical/High/Medium/Low | Why this matters |

Prioritize by:

1. Critical/High severity
2. Dimensions with lowest scores
3. Issues blocking other improvements

### Detailed Remediation Steps

For each priority dimension:

```
### Phase N: [Dimension Name] - Priority: [Priority]

**Target**: Increase from [current]/[max] to [target]/[max] (+[delta] points)

#### Step N.1: [Specific Action]

**[File to modify]**: `path/to/file.md`

[Code example if applicable]
```

#### Code Block Escaping

When documenting code examples that contain markdown code fences, use 4 backticks:

````markdown
**BAD**:
```markdown
Some markdown content here
```

**GOOD**:
```markdown
Good content here
```
````

### Success Criteria

Measurable targets for verification:

| Criterion | Measurement |
|-----------|-------------|
| Dimension Score | >= target/max |
| Overall Score | >= target-score (target-grade) |
| References Created | >= N files |

### Effort Estimate

| Phase | Effort | Time |
|-------|--------|-------|
| Phase 1 | S/M/L | HH hours |
| Phase 2 | S/M/L | HH hours |
| **Total** | **M/L** | **X hours** |

### Additional Sections

- **Dependencies**: Any prerequisites or blockers
- **Rollback Plan**: Git command to revert changes
- **Notes**: Honest assessment of plan quality (rating out of 10)

## Honest Rating Guidelines

Rate plans based on:

| Rating | Criteria |
|--------|----------|
| **9-10/10** | Template gold standard, comprehensive |
| **8/10** | Follows Format B well, detailed code examples |
| **7/10** | Good structure but gaps in specifics |
| **6/10** | Major restructuring needed |
| **<5/10** | Fundamental issues with approach |

## Common Remediation Patterns

### Pattern 1: Anti-Pattern Addition

For D3 (Anti-Pattern Quality) improvements:

````markdown
## Anti-Patterns

### NEVER: [Action]

**WHY**: [Reason]

**BAD**:
```[language]
[bad code]
```

**GOOD**:
```[language]
[good code]
```
````

### Pattern 2: Reference Creation

For D5 (Progressive Disclosure) improvements:

1. Create `skills/<skill>/references/` directory
2. Extract detailed content from SKILL.md
3. Add navigation table to SKILL.md:

```markdown
## Quick Reference

| Topic | Reference |
|-------|-----------|
| Topic Name | [references/topic.md](references/topic.md) |
```

### Pattern 3: Trigger Expansion

For D7 (Pattern Recognition) improvements:

Update frontmatter description:

```yaml
---
name: skill-name
description: |
  [Core description]. Use when: [trigger phrases].
  
  Keywords: [comma-separated keywords]
---
```

## Verification

After implementing remediation:

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json
bunx markdownlint-cli2 "skills/<skill-name>/**/*.md"
```

Compare new score against target in plan.

## Related References

- `framework-skill-judge-dimensions.md` - Understanding the 8 dimensions
- `framework-scoring-rubric.md` - How scores are calculated
- `aggregation-pattern.md` - For larger consolidation efforts
