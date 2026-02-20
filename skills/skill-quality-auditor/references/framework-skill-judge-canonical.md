---
category: framework
priority: CRITICAL
source: skill-judge canonical snapshot (vendored notes, 2026-02-20)
---

# Skill-Judge Canonical Integration

This file integrates the canonical `skill-judge` framework into `skill-quality-auditor`.

## Source

- Upstream snapshot path: `../deadalus/.agents/skills/skill-judge/SKILL.md`
- Upstream skill name: `skill-judge`
- Upstream purpose: evaluate skill design quality against official specifications and best practices.

## Canonical Principles

1. Skills should maximize knowledge delta.
2. Skills should transfer expert thinking patterns, not tutorials.
3. Skills should include explicit anti-patterns with consequences.
4. Description quality controls activation quality.
5. Progressive disclosure should keep `SKILL.md` concise.

## Canonical Dimensions

The canonical framework evaluates across 8 dimensions totaling 120 points.

| Dimension | Max | Local implementation |
| --- | --- | --- |
| Knowledge Delta | 20 | `framework-skill-judge-dimensions.md` |
| Mindset + Procedures | 15 | `framework-skill-judge-dimensions.md` |
| Anti-Pattern Quality | 15 | `framework-skill-judge-dimensions.md` |
| Specification Compliance | 15 | `framework-scoring-rubric.md` |
| Progressive Disclosure | 15 | `framework-scoring-rubric.md` |
| Freedom Calibration | 15 | `framework-scoring-rubric.md` |
| Pattern Recognition | 10 | `framework-scoring-rubric.md` |
| Practical Usability | 15 | `framework-quality-standards.md` |

## Sync Procedure

1. Compare this file and `framework-skill-judge-dimensions.md` against upstream `skill-judge`.
2. If upstream semantics change, update local dimensions and rubric references first.
3. Re-run audits using `scripts/evaluate.ts` and `scripts/audit-skills.sh`.
4. Record version notes in the audit report under `.context/analysis/`.

## Notes

This file intentionally keeps only canonical integration metadata and mapping.
Detailed evaluator guidance remains in the existing framework reference files.
