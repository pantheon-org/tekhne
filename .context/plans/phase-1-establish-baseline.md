# Phase 1: Establish Baseline

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** 3 hours  
**Status:** READY FOR EXECUTION

---

## Overview

Create proper audit structure for 6 representative skills (including skill-quality-auditor meta-audit) to validate the remediation process before scaling to all 63 skills.

## Pre-Execution Validation

**Run ALL validation commands before starting skill audits:**

```bash
# 1. Verify all skills exist
for skill in skill-quality-auditor acceptance-criteria ansible-generator nx-executors dockerfile-generator bdd-testing; do
  if [ -d "skills/$skill" ]; then
    echo "✓ $skill"
  else
    echo "✗ MISSING: $skill"
    exit 1
  fi
done

# 2. Verify evaluate.sh exists and is executable
if [ -f "skills/skill-quality-auditor/scripts/evaluate.sh" ]; then
  echo "✓ evaluate.sh found"
else
  echo "✗ evaluate.sh missing"
  exit 1
fi

# 3. Verify required directories exist
mkdir -p .context/audits
mkdir -p .context/analysis
echo "✓ directories ready"

# 4. Verify template files exist
test -f .context/analysis/score-mapping-phase1.md && echo "✓ score-mapping template exists" || echo "✗ MISSING score-mapping template"
test -f .context/analysis/phase1-blockers.md && echo "✓ blockers template exists" || echo "✗ MISSING blockers template"

# 5. Test evaluate.sh on dummy skill (dry run)
# This validates script functionality before real audits
sh skills/skill-quality-auditor/scripts/evaluate.sh --help 2>&1 | head -5
```

**Pre-Phase Checklist:**

- [ ] All 6 skills exist in skills/ directory
- [ ] evaluate.sh script found and executable
- [ ] Template files created (.context/analysis/*.md)
- [ ] Dry run test passed (or documented failure)
- [ ] Start time logged in phase1-blockers.md

---

## Error Handling & Failure Protocol

**If any skill audit fails during execution:**

1. **Capture the error:**
   ```bash
   # Log error to blockers file immediately
   echo "### Issue #N - <skill-name>" >> .context/analysis/phase1-blockers.md
   echo "- **Step**: [describe step]" >> .context/analysis/phase1-blockers.md
   echo "- **Error**: [paste error message]" >> .context/analysis/phase1-blockers.md
   echo "- **Status**: UNRESOLVED" >> .context/analysis/phase1-blockers.md
   echo "" >> .context/analysis/phase1-blockers.md
   ```

2. **Triage the failure:**
   - **Script error**: Document and skip to next skill (continue Phase 1)
   - **Missing files**: Verify skill exists, document, skip to next skill
   - **Unexpected output**: Save output to file, document, continue
   - **Timeout**: Document timing, consider extending timeout, retry once

3. **Continue or abort:**
   - **Continue**: If 4+ skills complete successfully
   - **Abort**: If 3+ consecutive failures or critical script bug discovered

4. **Rollback procedure** (if Phase 1 must be aborted):
   ```bash
   # Move incomplete audits to quarantine
   mkdir -p .context/audits/.incomplete-$(date +%Y-%m-%d)
   mv .context/audits/*/$(date +%Y-%m-%d) .context/audits/.incomplete-$(date +%Y-%m-%d)/ 2>/dev/null || true
   
   # Mark phase as failed in blockers file
   echo "## PHASE 1 ABORTED" >> .context/analysis/phase1-blockers.md
   echo "Reason: [describe]" >> .context/analysis/phase1-blockers.md
   echo "Completed: X/6 skills" >> .context/analysis/phase1-blockers.md
   ```

5. **Decision criteria for Phase 2:**
   - **PROCEED**: 5+ skills scored successfully, script reliable
   - **ADJUST**: 4 skills scored, minor script issues identified
   - **HALT**: <4 skills scored, major blockers discovered

---

## Skill 0: skill-quality-auditor (meta-audit)

**Purpose:** Baseline audit of the auditing tool itself. Ensures skill-quality-auditor meets its own quality standards.

### Execution Steps

1. Run evaluation:
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh skill-quality-auditor --json --store
   ```

   This automatically creates:
   - `.context/audits/skill-quality-auditor/YYYY-MM-DD/` directory
   - `audit.json`, `analysis.md`, `remediation-plan.md` (if score < 108)
   - `latest/` symlink pointing to newest audit

2. Verify structure created:
   ```bash
   ls -la .context/audits/skill-quality-auditor/latest/
   # Expected: analysis.md, audit.json, remediation-plan.md (if score < 108)
   ```

3. Review audit results:
   ```bash
   cat .context/audits/skill-quality-auditor/latest/audit.json | jq '.total, .grade, .dimensions'
   ```

4. Document score:
   - Tessl score: Already published (assumed A-grade equivalent)
   - skill-judge score: ?/120
   - Grade: ?
   - Update `.context/analysis/score-mapping-phase1.md`

5. Review generated analysis:
   ```bash
   cat .context/audits/skill-quality-auditor/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all files
- Meta-audit baseline established
- If score < 108: Document as known issue (tool auditing itself)
- Time logged: actual vs. 20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

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

## Skill 3: nx-executors (85→99% improvement)

**Context:** This skill showed significant improvement during tessl review (+14% gain). Need to understand if skill-judge captures similar improvement trend.

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
   - Tessl score: 85% → 99% (improved +14%)
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

## Skill 4: dockerfile-generator (76→92% improvement)

**Context:** This skill showed significant improvement during tessl review (+16% gain). Need to understand if skill-judge reflects similar quality improvement.

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
   - Tessl score: 76% → 92% (improved +16%)
   - skill-judge score: ?/120
   - Grade: ?
   - Document what dimensions improved

5. Review generated analysis:
   ```bash
   cat .context/audits/dockerfile-generator/latest/analysis.md
   ```

### Success Criteria

- Complete audit directory exists with all files
- Can identify which dimensions contributed to improvement
- Remediation plan exists if score < 108
- Time logged: actual vs. 20min estimate

### Blockers/Issues Log

- Issue 1: [description]
- Issue 2: [description]

---

## Skill 5: bdd-testing (100% tessl, existing audit comparison)

**Context:** This skill has 100% tessl score and may already have an audit. Use it to validate process consistency and compare outputs.

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
   - Tessl score: 100%
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

1. **Populate score mapping table** in `.context/analysis/score-mapping-phase1.md`:

   The table template already exists. Fill in all "?" values with actual data:
   - skill-judge scores from audit.json files
   - Grades from audit.json files
   - Notes column: Record one of the following:
     - Key dimension failures (e.g., "Lost 8pts on specificity")
     - Blockers encountered (e.g., "Script timeout")
     - Recommendation (e.g., "No action needed", "Minor fixes", "Major remediation")
     - Correlation observation (e.g., "High tessl = high skill-judge")

2. **Document all blockers** in `.context/analysis/phase1-blockers.md`:

   Template already exists. Fill in:
   - All issues encountered during execution
   - Resolutions or workarounds applied
   - Time tracking table (estimated vs actual)
   - Process improvements identified for Phase 2

3. **Calculate metrics:**
   ```bash
   # Average time per skill
   # Total actual time for Phase 1
   # Success rate (X/6 skills completed)
   # Script reliability (failures / 6 total runs)
   ```

4. **Analyze correlation:**
   - Do high tessl scores (100%, 99%, 92%) map to high skill-judge scores?
   - What's the score distribution? (A/B/C/D/F grades)
   - Which dimensions cause most point loss across all skills?
   - Is the 108/120 (90%) A-grade threshold appropriate?

### Decision Checkpoint

**Can we proceed to Phase 2 batch processing?**

Use this decision matrix:

| Criteria | Threshold | Actual | Pass? |
|----------|-----------|--------|-------|
| Skills completed | ≥5/6 | ?/6 | ? |
| Script reliability | ≥80% | ?% | ? |
| Time predictability | ±30% of estimate | ?% | ? |
| Correlation clarity | Clear pattern | ? | ? |

**Decision outcomes:**

- **PROCEED TO PHASE 2** if:
  - 5+ skills completed successfully
  - Script ran reliably (≤1 failure)
  - Timing is predictable (within ±30%)
  - Clear correlation between tessl and skill-judge scores

- **ADJUST & RETRY** if:
  - 4 skills completed
  - Minor script issues identified and fixable
  - Timing needs adjustment but manageable
  - Correlation is unclear but data suggests refinement needed

- **HALT & REASSESS** if:
  - <4 skills completed
  - Major script failures or bugs
  - Timing wildly unpredictable (>50% variance)
  - No correlation or contradictory results

### Phase 1 Success Criteria (Overall)

- [ ] 6 skills have complete audit structure (including meta-audit)
- [ ] All score mapping table rows populated
- [ ] Tessl-to-skill-judge correlation documented
- [ ] Process validated and blockers documented
- [ ] Time estimates refined for Phase 2 (X min per skill)
- [ ] Decision recorded: PROCEED / ADJUST / HALT
- [ ] If PROCEED: Automation approach confirmed for Phase 2

---

## Execution Time Estimates

| Task | Estimated Time | Notes |
|------|---------------|-------|
| Pre-execution validation | 10 min | Run all validation checks |
| Skill 0 (skill-quality-auditor) | 20 min | Meta-audit baseline |
| Skill 1 (acceptance-criteria) | 20 min | Perfect tessl score |
| Skill 2 (ansible-generator) | 20 min | Good tessl score (93%) |
| Skill 3 (nx-executors) | 20 min | Improvement case |
| Skill 4 (dockerfile-generator) | 20 min | Improvement case |
| Skill 5 (bdd-testing) | 20 min | Comparison with existing audit |
| Wrap-up tasks | 30 min | Data collection, analysis, decision |
| **TOTAL** | **3 hours** | Includes buffer for issues |

**Timing notes:**
- Extended from 2hr to 3hr to accommodate 6 skills instead of 5
- Each skill: ~15min execution + 5min review/documentation
- Buffer built into wrap-up time for unexpected issues

---

## Notes for Execution

1. **Don't rush Phase 1** - This is the validation phase. Better to spend extra time here than to fix issues across 63 skills in Phase 2.

2. **Document everything immediately** - If something breaks or behaves unexpectedly, document it in phase1-blockers.md BEFORE continuing to the next skill.

3. **Follow the failure protocol** - Don't improvise if something fails. Use the documented error handling procedure.

4. **Keep all data** - Don't delete any files during Phase 1. This includes:
   - Original `.context/*.txt` files from ad-hoc tessl audits
   - Any failed audit attempts
   - Intermediate output or debug logs

5. **Verify assumptions early** - If skill-judge scoring is completely different from tessl scoring on the first 2 skills, pause and reassess before continuing.

6. **Track time religiously** - Log start/end time for each skill to improve Phase 2 estimates. Update phase1-blockers.md time tracking table after each skill.

7. **Use the decision matrix** - Don't skip the decision checkpoint at wrap-up. The decision matrix provides objective criteria for Phase 2 readiness.

8. **Parallel execution consideration** - If all 6 skills complete successfully and script is stable, document whether parallel execution is feasible for Phase 2 batch processing.

---

## Rollback & Recovery

**If Phase 1 must be aborted mid-execution:**

1. Quarantine incomplete audits:
   ```bash
   mkdir -p .context/audits/.incomplete-$(date +%Y-%m-%d)
   find .context/audits -type d -name "$(date +%Y-%m-%d)" -exec mv {} .context/audits/.incomplete-$(date +%Y-%m-%d)/ \;
   ```

2. Document abort reason in phase1-blockers.md

3. Preserve all error logs and partial outputs

4. Do NOT delete any generated files (needed for debugging)

**If a single skill fails but Phase 1 continues:**

1. Document in phase1-blockers.md immediately

2. Move to next skill (don't retry immediately)

3. Consider retry after all other skills complete

4. Mark skill as "INCOMPLETE" in score mapping table

---

**Next Step:** Run pre-execution validation, then begin with Skill 0 (skill-quality-auditor)  
**Owner:** AI Agent (OpenCode)  
**Status:** READY FOR EXECUTION
