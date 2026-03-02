# Phase 1 Score Mapping

**Generated:** 2026-03-02  
**Purpose:** Map Tessl scores to skill-judge scores for 5 representative skills

---

## Score Mapping Table

| Skill | Tessl Score | skill-judge Score | Grade | Notes |
|-------|-------------|-------------------|-------|-------|
| skill-quality-auditor | ? | ?/120 | ? | Meta-audit baseline |
| acceptance-criteria | 100% | ?/120 | ? | Perfect tessl score control |
| ansible-generator | 93% | ?/120 | ? | Good tessl score |
| nx-executors | 85→99% | ?/120 | ? | Improved via optimization |
| dockerfile-generator | 76→92% | ?/120 | ? | Regressed post-optimization |
| bdd-testing | 100% | ?/120 | ? | Existing audit comparison |

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

## Phase 1 Completion Checklist

- [ ] All 6 skills scored
- [ ] Notes populated for each skill
- [ ] Correlation analysis documented
- [ ] Decision recorded: Proceed to Phase 2 or adjust strategy
