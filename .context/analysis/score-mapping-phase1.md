# Phase 1 Score Mapping

**Generated:** 2026-03-02  
**Purpose:** Map Tessl scores to skill-judge scores for 5 representative skills

---

## Score Mapping Table

| Skill | Tessl Score | skill-judge Score | Grade | Notes |
|-------|-------------|-------------------|-------|-------|
| skill-quality-auditor | N/A (meta) | 106/120 (88.33%) | B+ | Progressive Disclosure 73%, 2pts from A-grade |
| acceptance-criteria | 100% | 104/120 (86.67%) | B+ | Mindset 73%, 4pts from A-grade |
| ansible-generator | 93% | 93/120 (77.50%) | C+ | Anti-Patterns 53%, awk multibyte warning |
| nx-executors | 85→99% | 111/120 (92.50%) | A | Strongest performance, balanced dimensions |
| dockerfile-generator | 76→92% | 105/120 (87.50%) | B+ | Progressive Disclosure 67%, 3pts from A-grade |
| bdd-testing | 100% | 104/120 (86.67%) | B+ | Progressive Disclosure 73%, 4pts from A-grade |

---

## Notes Column Legend

Record one of the following for each skill:
- **Key dimension failures**: Which of the 8 dimensions scored lowest
- **Blockers encountered**: Script errors, missing files, timeouts
- **Recommendation**: "No action", "Minor fixes", "Major remediation"
- **Correlation**: How well tessl and skill-judge scores align

---

## Analysis Questions

1. **Correlation strength**: Do tessl scores predict skill-judge scores?
2. **Dimension insights**: Which dimensions cause most point loss?
3. **Threshold validation**: Is 108/120 (90%) a good A-grade cutoff?
4. **Tool reliability**: Did evaluate.sh run cleanly for all skills?

---

## Key Findings

### Correlation Analysis

**NO STRONG CORRELATION** between Tessl scores and skill-judge scores:
- acceptance-criteria: 100% tessl → 104/120 (B+)
- bdd-testing: 100% tessl → 104/120 (B+)
- ansible-generator: 93% tessl → 93/120 (C+) [WEAK OUTLIER]
- nx-executors: 99% tessl → 111/120 (A) [ONLY A-GRADE]

**Key Insight**: High tessl scores (100%) do NOT guarantee A-grade. nx-executors (99% tessl) is the only A-grade, suggesting tessl optimization may target different quality dimensions than skill-judge.

### Dimension Insights

**Common weak dimensions across all skills:**
1. **Progressive Disclosure** (D5): 67-73% - Most skills struggle here
2. **Mindset+Procedures** (D2): 73-93% - Significant variance
3. **Anti-Pattern Quality** (D3): 53-87% - ansible-generator critical failure

**Strong dimensions consistently:**
- Pattern Recognition (D7): 100% across all skills
- Specification Compliance (D4): 93-100% across most skills

### Threshold Validation

**A-grade threshold (108/120 = 90%) appears appropriate:**
- Only nx-executors (111/120) achieved A-grade
- 4 skills in B+ range (104-106): 2-4 points from A-grade
- 1 skill in C+ range (93): needs major remediation

**Distribution:**
- A: 1/6 (17%)
- B+: 4/6 (67%)
- C+: 1/6 (17%)

### Tool Reliability

**evaluate.sh performed reliably:**
- 6/6 skills completed successfully
- 1 minor warning (awk multibyte in ansible-generator)
- Consistent structure: `.context/audits/{skill}/latest/` with analysis.md, audit.json, remediation-plan.md
- Average runtime: ~30-60 seconds per skill

## Phase 1 Completion Checklist

- [x] All 6 skills scored
- [x] Notes populated for each skill
- [x] Correlation analysis documented
- [x] Decision recorded: **PROCEED to Phase 2** with confidence

## Recommendation

**PROCEED TO PHASE 2** with the following adjustments:
1. Do NOT assume tessl score predicts skill-judge quality
2. Focus remediation on Progressive Disclosure (D5) improvements
3. Target ansible-generator for immediate deep remediation (Anti-Patterns D3)
4. Use nx-executors as exemplar for other skills
