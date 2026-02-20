# Skill Evaluation Report: [skill-name]

**Review Date**: YYYY-MM-DD  
**Reviewer**: [reviewer]  
**Skill Location**: `skills/[skill-name]/`

---

## Summary

| Metric | Value |
| --- | --- |
| **Total Score** | [score]/120 ([percent]%) |
| **Grade** | [grade] |
| **Pattern** | [pattern] |
| **Knowledge Ratio** | E:A:R = [ratio] |
| **Verdict** | [verdict] |

---

## Dimension Scores

| Dimension | Score | Max | Notes |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | [n] | 20 | [notes] |
| D2: Mindset + Procedures | [n] | 15 | [notes] |
| D3: Anti-Pattern Quality | [n] | 15 | [notes] |
| D4: Specification Compliance | [n] | 15 | [notes] |
| D5: Progressive Disclosure | [n] | 15 | [notes] |
| D6: Freedom Calibration | [n] | 15 | [notes] |
| D7: Pattern Recognition | [n] | 10 | [notes] |
| D8: Practical Usability | [n] | 15 | [notes] |
| **TOTAL** | **[n]** | **120** | |

---

## Audit Execution

| Check | Result | Evidence |
| --- | --- | --- |
| Single-skill semantic evaluation | Pass/Fail | [command + outcome] |
| Full collection audit script | Pass/Fail | [command + outcome] |
| Duplication detection script | Pass/Fail | [command + outcome] |

---

## Findings

### High Severity

1. [finding]

### Medium Severity

1. [finding]

---

## Improvements Applied

1. [change]
2. [change]

---

## Score Evolution

| Review | Score | Grade | Status |
| --- | --- | --- | --- |
| Baseline | [score] | [grade] | [status] |
| Current | [score] | [grade] | [status] |

---

## Dimension Analysis

### D3: Anti-Pattern Quality

[analysis]

### D4: Specification Compliance

[analysis]

### D8: Practical Usability

[analysis]

---

## Files Inventory

```text
skills/[skill-name]/
├── SKILL.md
├── AGENTS.md
├── references/
└── scripts/
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts skill-quality-auditor --json
skills/skill-quality-auditor/scripts/audit-skills.sh
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
bun run skills/skill-quality-auditor/scripts/validate-review-format.ts .context/reviews/skill-quality-auditor-review.md
```

---

## Grade Scale Reference

| Grade | Percentage | Meaning |
| --- | --- | --- |
| A | 90%+ (108+) | Excellent |
| B | 80-89% (96-107) | Good |
| C | 70-79% (84-95) | Adequate |
| D | 60-69% (72-83) | Below Average |
| F | <60% (<72) | Poor |

---

## Conclusion

[concise final verdict]
