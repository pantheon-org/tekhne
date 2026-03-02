# Phase 1: Establish Baseline

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** 2 hours  
**Status:** READY FOR EXECUTION

---

## Overview

Create proper audit structure for 5 representative skills to validate the remediation process before scaling to all 63 skills.

## Pre-Phase Checklist

- [ ] Verify skill-quality-auditor scripts are executable
- [ ] Confirm evaluate.sh accepts --json and --store flags
- [ ] Test script on one skill to verify directory structure creation
- [ ] Document any script dependencies or environment requirements

---

## Skill 1: acceptance-criteria (100% tessl score)

### Execution Steps

1. Run evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json --store
   ```

   This automatically creates:
   - `.context/audits/acceptance-criteria/YYYY-MM-DD/` directory
   - `audit.json`, `analysis.md`, `remediation-plan.md` (if score < 108)
   - `latest/` symlink pointing to newest audit

2. Verify structure created:
   ```bash
   ls -la .context/audits/acceptance-criteria/latest/
   # Expected: analysis.md, audit.json, remediation-plan.md (if score < 108)
   ```

3. Review audit results:
   ```bash
   cat .context/audits/acceptance-criteria/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

4. Compare scores:
   - Tessl score: 100%
   - skill-judge score: ?/120
   - Grade: ?
   - Document mapping in `.context/analysis/score-mapping-phase1.md`

5. Review generated analysis:
   ```bash
   cat .context/audits/acceptance-criteria/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all 3 files (or 2 if score >= 108)
- Can map tessl 100% to skill-judge score
- Remediation plan exists if score < 108
- Time logged: actual vs. 15-20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Skill 2: ansible-generator (93% tessl score)

### Execution Steps

1. Run evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh ansible-generator --json --store
   ```

2. Verify structure created:
   ```bash
   ls -la .context/audits/ansible-generator/latest/
   ```

3. Review audit results:
   ```bash
   cat .context/audits/ansible-generator/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

4. Compare scores:
   - Tessl score: 93%
   - skill-judge score: ?/120
   - Grade: ?
   - Document mapping in `.context/analysis/score-mapping-phase1.md`

5. Review generated analysis:
   ```bash
   cat .context/audits/ansible-generator/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all files (2-3 depending on score)
- Can map tessl 93% to skill-judge score
- Remediation plan exists if score < 108
- Time logged: actual vs. 15-20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Skill 3: nx-executors (85→100% improvement)

**Context:** This skill showed improvement during tessl review. Need to understand if skill-judge captures similar improvement trend.

### Execution Steps

1. Run evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh nx-executors --json --store
   ```

2. Verify structure created:
   ```bash
   ls -la .context/audits/nx-executors/latest/
   ```

3. Review audit results:
   ```bash
   cat .context/audits/nx-executors/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

4. Compare scores:
   - Tessl score: 85% → 100% (improved)
   - skill-judge score: ?/120
   - Grade: ?
   - Document what dimensions improved

5. Review generated analysis:
   ```bash
   cat .context/audits/nx-executors/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all files (2-3 depending on score)
- Can identify which dimensions would have been flagged at 85%
- Remediation plan exists if score < 108
- Time logged: actual vs. 15-20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Skill 4: dockerfile-generator (93→85% regression)

**Context:** This skill showed regression during tessl review. Need to understand if skill-judge identifies similar issues.

### Execution Steps

1. Run evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh dockerfile-generator --json --store
   ```

2. Verify structure created:
   ```bash
   ls -la .context/audits/dockerfile-generator/latest/
   ```

3. Review audit results:
   ```bash
   cat .context/audits/dockerfile-generator/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

4. Compare scores:
   - Tessl score: 93% → 85% (regressed)
   - skill-judge score: ?/120
   - Grade: ?
   - Document what dimensions regressed

5. Verify remediation-plan.md was generated (should exist for regression case)

6. Review generated analysis:
   ```bash
   cat .context/audits/dockerfile-generator/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all files (should have remediation plan)
- Can identify which dimensions flagged the regression
- Remediation plan clearly articulates issues
- Time logged: actual vs. 15-20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Skill 5: bdd-testing (existing audit for comparison)

**Context:** This skill should already have an audit. Use it to validate process and compare outputs.

### Execution Steps

1. Check for existing audit:
   ```bash
   ls -la .context/audits/bdd-testing/
   # Document what exists
   ```

2. Run new evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh bdd-testing --json --store
   ```

3. Verify structure created:
   ```bash
   ls -la .context/audits/bdd-testing/latest/
   ```

4. Compare old vs new audit (if old audit exists):
   ```bash
   # Compare old and new scores if multiple dated directories exist
   find .context/audits/bdd-testing/ -name "audit.json" -type f | sort
   ```

5. Review audit results:
   ```bash
   cat .context/audits/bdd-testing/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

6. Compare scores:
   - Tessl score: ?
   - Previous skill-judge score: ?/120 (if exists)
   - New skill-judge score: ?/120
   - Grade: ?
   - Document any score drift in `.context/analysis/score-mapping-phase1.md`

### Success Criteria

- Complete audit directory exists with all files (2-3 depending on score)
- Can compare old and new audit structure
- Scores are consistent with previous audit (if exists)
- Time logged: actual vs. 15-20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Phase 1 Wrap-Up Tasks

### Data Collection

1. Create mapping table in `.context/analysis/score-mapping-phase1.md`:
   ```markdown
   | Skill | Tessl Score | skill-judge Score | Grade | Notes |
   |-------|-------------|-------------------|-------|-------|
   | acceptance-criteria | 100% | ?/120 | ? | ... |
   | ansible-generator | 93% | ?/120 | ? | ... |
   | nx-executors | 85→100% | ?/120 | ? | ... |
   | dockerfile-generator | 93→85% | ?/120 | ? | ... |
   | bdd-testing | ? | ?/120 | ? | ... |
   ```

2. Document process issues in `.context/analysis/phase1-blockers.md`:
   - Script failures
   - Missing dependencies
   - Unexpected behaviors
   - Time overruns

3. Calculate metrics:
   - Average time per skill (actual)
   - Total time for Phase 1 (actual vs. 2hr estimate)
   - Success rate (5/5 skills completed?)
   - Script reliability (failures / total runs)

4. Decision checkpoint:
   - Can we proceed to Phase 2 batch processing?
   - Do we need to fix evaluate.sh script?
   - Do we need to adjust time estimates?
   - Do we need to modify the automation script?

### Phase 1 Success Criteria (Overall)

- [ ] 5 skills have complete audit structure
- [ ] Tessl-to-skill-judge mapping established
- [ ] Process validated and documented
- [ ] Time estimates refined for Phase 2
- [ ] Automation script ready (if needed)

---

## Notes for Execution

1. **Don't rush Phase 1** - This is the validation phase. Better to spend extra time here than to fix issues across 63 skills.

2. **Document everything** - If something breaks or behaves unexpectedly, document it immediately before continuing.

3. **Keep original data** - Don't delete any `.context/*.txt` files from the ad-hoc tessl audits until Phase 1 is complete and validated.

4. **Verify assumptions** - If skill-judge scoring is completely different from tessl scoring, reassess the strategy before Phase 2.

5. **Time tracking** - Log actual time for each skill to improve future estimates.

---

**Next Step:** Begin with acceptance-criteria skill  
**Owner:** AI Agent (OpenCode)  
**Status:** READY FOR EXECUTION
