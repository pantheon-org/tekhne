---
plan_date: 2026-02-23
skill_name: skill-quality-auditor
source_audit: .context/audits/skill-quality-auditor-audit-2026-02-22.md
---

# Remediation Plan: skill-quality-auditor

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 106/120 (88%) |
| **Current Grade** | B+ |
| **Target Score** | 110+/120 (A- or higher) |
| **Priority** | Medium - Targeted improvements recommended |
| **Estimated Effort** | Low (1-2 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Moderate progressive disclosure | Medium | D5 | 10/15 | 13/15 |
| Maintainability pressure | Low | - | 260 lines | <200 lines |

**Note**: This skill is the meta-skill that evaluates other skills. It should serve as the gold standard template.

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Medium Priority

**Goal**: Increase D5 score from 10/15 to 13/15

**File**: `skills/skill-quality-auditor/SKILL.md`

**Step 1.1**: Audit current content distribution

Current state:

- SKILL.md: 260 lines
- references/: 14 files

**Step 1.2**: Identify extraction candidates

Content to move to references:

- Detailed dimension explanations (already partially in references)
- Scoring rubrics (if duplicated)
- Example outputs

**Step 1.3**: Create consolidated reference files

Create `skills/skill-quality-auditor/references/scoring-rubric.md`:

```markdown
# Skill Quality Scoring Rubric

## Dimension Weights

| Dimension | Weight | Description |
| --- | ---: | --- |
| D1: Knowledge Delta | 20 | Non-obvious guidance depth |
| D2: Mindset + Procedures | 15 | Procedural clarity |
| D3: Anti-Pattern Quality | 15 | Specificity and enforceability |
| D4: Specification Compliance | 15 | Metadata/spec alignment |
| D5: Progressive Disclosure | 15 | SKILL-to-references balance |
| D6: Freedom Calibration | 15 | Constraint/flexibility balance |
| D7: Pattern Recognition | 10 | Trigger/intent discoverability |
| D8: Practical Usability | 15 | Actionable examples/commands |
| **Total** | **120** | |

## Grade Scale

| Grade | Score Range | Assessment |
| --- | --- | --- |
| A+ | 115-120 | Exceptional |
| A | 108-114 | Strong |
| B+ | 103-107 | Solid baseline |
| B | 96-102 | Good with gaps |
| C+ | 90-95 | Moderate |
| C | 84-89 | Mixed quality |
| D | 78-83 | Low quality |
| F | 0-77 | Unacceptable |

## Signal Levels

- **Strong**: Score >= 80% of max for dimension
- **Moderate**: Score 50-79% of max for dimension
- **Weak**: Score < 50% of max for dimension

## Priority Levels

- **High**: Weak signal, immediate improvement needed
- **Medium**: Moderate signal, targeted improvement
- **Low**: Strong signal, maintain or minor refinement
```

**Step 1.4**: Update SKILL.md with concise hub structure

Replace detailed rubrics with:

```markdown
## Scoring

See [Scoring Rubric](references/scoring-rubric.md) for complete dimension weights and grade scales.

### Quick Reference

- **Passing**: Score >= 84 (Grade C or higher)
- **Recommended**: Score >= 96 (Grade B or higher)
- **Target**: Score >= 108 (Grade A or higher)
```

---

### Phase 2: Maintainability Optimization - Low Priority

**Goal**: Reduce SKILL.md to under 200 lines

**File**: `skills/skill-quality-auditor/SKILL.md`

**Step 2.1**: Move detailed dimension analysis template

Create `skills/skill-quality-auditor/references/dimension-analysis-template.md`:

```markdown
# Dimension Analysis Template

Each dimension follows this analysis structure:

## D[N]: [Dimension Name]

**Assessment**: [score]/[max] (signal: [strong|moderate|weak], priority: [high|medium|low])

**Inspect**:

- [File reference] for [specific check].
- [Presence of specific elements].

**Fix steps**:

1. [Actionable step 1]
2. [Actionable step 2]
3. [Actionable step 3]

**Done criteria**:

- [Measurable criterion 1]
- [Measurable criterion 2]

**Re-check**: `sh skills/skill-quality-auditor/scripts/evaluate.sh [skill-name] --json`
```

**Step 2.2**: Condense verification section

Replace detailed verification block with:

````markdown
## Verification

```bash
# Evaluate specific skill
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json

# Audit all skills
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Full validation suite
make audit-skills
```

See [Verification Guide](references/verification-guide.md) for complete command reference.
````

---

### Phase 3: Enhance Meta-Quality - Enhancement

**Goal**: Ensure the auditor skill exemplifies best practices

**File**: `skills/skill-quality-auditor/SKILL.md`

**Step 3.1**: Add self-reference example

Add section:

````markdown
## Self-Audit

This skill should pass its own evaluation. Run:

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json
```

Expected: Score >= 100/120 (Grade B+ or higher)

If score falls below, prioritize fixes to this skill first as it serves as the quality standard.
````

**Step 3.2**: Add cross-skill consistency check

Create `skills/skill-quality-auditor/scripts/check-consistency.sh`:

```sh
#!/usr/bin/env sh
# Check that all skills follow the same structure pattern

set -e

SKILLS_DIR="${1:-skills}"
ISSUES=0

for skill_dir in "$SKILLS_DIR"/*/; do
    skill_name=$(basename "$skill_dir")
    
    # Check SKILL.md exists
    if [ ! -f "$skill_dir/SKILL.md" ]; then
        echo "MISSING: $skill_name/SKILL.md"
        ISSUES=$((ISSUES + 1))
    fi
    
    # Check frontmatter
    if ! grep -q "^---" "$skill_dir/SKILL.md" 2>/dev/null; then
        echo "NO_FRONTMATTER: $skill_name/SKILL.md"
        ISSUES=$((ISSUES + 1))
    fi
done

echo ""
echo "Consistency check complete: $ISSUES issues found"
exit $ISSUES
```

---

## Verification Commands

```bash
# Self-evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json

# Run consistency check
sh skills/skill-quality-auditor/scripts/check-consistency.sh skills

# Validate SKILL.md length
wc -l skills/skill-quality-auditor/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 110/120 |
| Grade | A- or higher |
| D5: Progressive Disclosure | >= 13/15 |
| SKILL.md line count | < 200 lines |
| Self-audit passes | Score >= 100 |
| References updated | Scoring rubric extracted |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Progressive Disclosure | 45 min | None |
| Phase 2: Maintainability | 30 min | Phase 1 |
| Phase 3: Meta-Quality | 30 min | Phase 2 |
| Verification | 15 min | All phases |

**Total Estimated Time**: 2 hours

## Notes

- Rating: **9/10** - Already follows Format B template very well - this IS the template skill!
- This is the meta-skill that evaluates other skills - should be gold standard
- Strong structure with 3 phases and Timeline table
- Has Estimated Effort in Executive Summary
- Includes self-audit capability - key feature for meta-skill
- Minor: Could add more self-referential examples
