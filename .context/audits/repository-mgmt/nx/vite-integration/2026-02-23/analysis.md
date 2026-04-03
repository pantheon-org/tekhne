---
review_date: 2026-02-23
reviewer: automated audit
skill_location: skills/nx-vite-integration/SKILL.md
source_command: sh skills/skill-quality-auditor/scripts/evaluate.sh nx-vite-integration --json
---

# Skill Evaluation Report: nx-vite-integration

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 96/120 (80%) |
| **Grade** | B |
| **Pattern** | Good quality |
| **Verdict** | Remediation target achieved |

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 18 | 20 | Strong repository-specific depth |
| D2: Mindset + Procedures | 11 | 15 | Improved, but still moderate |
| D3: Anti-Pattern Quality | 13 | 15 | Strong and enforceable |
| D4: Specification Compliance | 10 | 15 | Moderate |
| D5: Progressive Disclosure | 10 | 15 | Moderate; references now present |
| D6: Freedom Calibration | 13 | 15 | Strong constraint balance |
| D7: Pattern Recognition | 6 | 10 | Weak; trigger detection still limited |
| D8: Practical Usability | 15 | 15 | Excellent command/example usability |

## Metadata

| Field | Value |
| --- | --- |
| **SKILL.md line count** | 291 |
| **Has references directory** | true |
| **Reference markdown files** | 5 |

## Key Findings

### 1. Overall target reached

- `96/120 (B)` meets the remediation target from the plan.

### 2. Progressive disclosure improved

- `references/` exists with 5 focused docs.
- `SKILL.md` reduced below 300 lines.

### 3. Remaining scoring headroom

- D7 remains at `6/10` (pattern recognition).
- D4 remains at `10/15` (specification compliance).
- D2 and D5 are improved but still moderate.

## Recommended Next Improvements

1. Expand frontmatter discoverability terms further to raise D7.
2. Tighten spec-facing metadata language and intent boundaries to raise D4.
3. Add one short decision matrix in SKILL.md to strengthen D2 while keeping file size below 300 lines.

## Re-run Command

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh nx-vite-integration --json
```
