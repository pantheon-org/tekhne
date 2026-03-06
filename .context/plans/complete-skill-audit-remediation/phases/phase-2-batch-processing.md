# Phase 2: Batch Processing

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** TBD (calculate after Phase 1 completion)  
**Status:** PENDING PHASE 1 COMPLETION

---

## Overview

Apply validated audit process to all 57 remaining skills using automated batch processing.

**Total Skills:** 63  
**Phase 1 Skills:** 6 (skill-quality-auditor, acceptance-criteria, ansible-generator, nx-executors, dockerfile-generator, bdd-testing)  
**Phase 2 Skills:** 57 (all remaining skills in 6 batches)

## Prerequisites

- [ ] Phase 1 completed successfully (6 skills audited)
- [ ] Process validated and decision = PROCEED
- [ ] Time estimates calculated using Phase 1 actuals
- [ ] Automation scripts verified (audit-skill.sh, batch-audit.sh, audit-helpers.sh)
- [ ] phase2-issues.md template created and ready

---

## Time Estimation (Calculate After Phase 1)

**Fill this section after Phase 1 completion:**

From Phase 1, average time per skill: **[X] minutes**

**Phase 2 time calculation:**
- Batch 1-5 (10 skills each): 10 × X = **[Y] minutes per batch**
- Batch 6 (7 skills): 7 × X = **[Z] minutes**
- Buffer for issues (15%): **[B] minutes**
- Wrap-up tasks: **30 minutes**

**Total estimated time: [T] hours**

---

## Quality Thresholds & Halt Criteria

After each batch, check score distribution in `.context/analysis/phase2-issues.md`:

### ✅ PASS - Continue to Next Batch
- ≥60% of skills score A-grade (≥108/120)
- ≤10% of skills score F-grade (<72/120)
- Average batch score ≥90/120

### ⚠️ WARNING - Document and Continue with Caution
- 40-59% score A-grade
- 11-20% score F-grade
- Average batch score 75-89/120
- **Action:** Document concerns in phase2-issues.md, watch for patterns

### 🛑 HALT - Stop Phase 2 and Reassess
- <40% score A-grade
- >20% score F-grade  
- Average batch score <75/120
- **Action:** Indicates systematic issues with evaluate.sh or widespread skill quality problems

---

## Pre-Phase Setup

### 1. Backup Existing Audits

```bash
# Create timestamped backup before running batch processing
BACKUP_DIR=".context/audits.backup-$(date +%Y-%m-%d-%H%M%S)"
cp -r .context/audits "$BACKUP_DIR"
echo "✅ Backup created: $BACKUP_DIR"
```

### 2. Verify Skill-Quality-Auditor Capability

The skill-quality-auditor provides batch auditing capability via `batch-audit.sh`.

**Verification:**

```bash
# Verify evaluate.sh exists (single skill audit)
test -x skills/skill-quality-auditor/scripts/evaluate.sh && echo "✅ evaluate.sh" || echo "❌ MISSING"

# Verify batch-audit.sh exists (batch wrapper)
test -x skills/skill-quality-auditor/scripts/batch-audit.sh && echo "✅ batch-audit.sh" || echo "❌ MISSING"
```

**Expected result:** Both scripts exist and are executable

### 3. Test Batch Audit Capability

Use the skill-quality-auditor's batch capability on Phase 1 skills to verify it works:

```bash
# Test batch audit on 2 Phase 1 skills
sh skills/skill-quality-auditor/scripts/batch-audit.sh acceptance-criteria ansible-generator

# Expected output:
# ✅ acceptance-criteria audit complete
# ✅ ansible-generator audit complete
# Summary: 2/2 succeeded, 0 failed
```

**Decision:**
- ✅ **PROCEED** if both skills audit successfully
- ⚠️ **HALT** if any failures occur - investigate before continuing

### 4. Initialize phase2-issues.md

Verify template exists and is ready:
```bash
test -f .context/analysis/phase2-issues.md && echo "✅ Ready" || echo "❌ Missing template"
```

---

## Batch 1: Skills 6-15 (10 skills)

**Skills:**
1. agents-md
2. ansible-validator
3. azure-pipelines-generator
4. azure-pipelines-validator
5. bash-script-generator
6. bash-script-validator
7. biome-complete
8. bun-development
9. cdk-nag
10. cfn-behavior-validator

### Execution

**Using batch-audit.sh (recommended):**
```bash
sh skills/skill-quality-auditor/scripts/batch-audit.sh agents-md ansible-validator azure-pipelines-generator \
  azure-pipelines-validator bash-script-generator bash-script-validator \
  biome-complete bun-development cdk-nag cfn-behavior-validator
```

**Manual loop (alternative):**
```bash
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
  echo ""
  echo "========================================="
  echo "Processing: $skill"
  echo "========================================="
  ./scripts/audit-skill.sh "$skill"
done
```

### Quality Check & Score Analysis

```bash
# Check all audits completed successfully
. scripts/audit-helpers.sh
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
  check_audit "$skill"
done

# Calculate score distribution
echo ""
echo "Score distribution:"
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
  jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERROR"
done | sort | uniq -c

# Calculate average score
echo ""
echo "Average score:"
total=0
count=0
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
  score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null)
  if [ -n "$score" ] && [ "$score" != "null" ]; then
    total=$((total + score))
    count=$((count + 1))
  fi
done
if [ $count -gt 0 ]; then
  avg=$((total / count))
  echo "$avg/120"
fi
```

**Verify halt criteria:**
- Document results in `.context/analysis/phase2-issues.md` (Score Distribution table)
- Check if batch meets PASS/WARNING/HALT thresholds
- Decision: Continue to Batch 2 / Pause and investigate / Abort Phase 2

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 1 complete (skills 6-15)"
```

### Checklist

- [ ] All 10 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes

---

## Batch 2: Skills 16-25 (10 skills)

**Skills:**
1. cfn-template-compare
2. colyseus-multiplayer
3. commanderjs
4. conventional-commits
5. create-context-file
6. dockerfile-validator
7. extending-nx-plugins
8. fluentbit-generator
9. fluentbit-validator
10. github-actions-generator

### Execution

```bash
for skill in cfn-template-compare colyseus-multiplayer commanderjs conventional-commits create-context-file dockerfile-validator extending-nx-plugins fluentbit-generator fluentbit-validator github-actions-generator; do
  echo ""
  echo "========================================="
  echo "Processing: $skill"
  echo "========================================="
  ./scripts/audit-skill.sh "$skill" || {
    echo "❌ FAILED: $skill"
    read -p "Continue? (y/N) " -n 1 -r; echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 1
  }
done
```

### Quality Check

```bash
. scripts/audit-helpers.sh
for skill in cfn-template-compare colyseus-multiplayer commanderjs conventional-commits create-context-file dockerfile-validator extending-nx-plugins fluentbit-generator fluentbit-validator github-actions-generator; do
  check_audit "$skill"
done
```

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 2 complete (skills 16-25)"
```

### Checklist

- [ ] All 10 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes

---

## Batch 3: Skills 26-35 (10 skills)

**Skills:**
1. github-actions-validator
2. github-copilot-models
3. gitlab-api
4. gitlab-ci-generator
5. gitlab-ci-validator
6. helm-generator
7. helm-validator
8. implementation-plan-splitter
9. jenkinsfile-generator
10. jenkinsfile-validator

### Execution

```bash
for skill in github-actions-validator github-copilot-models gitlab-api gitlab-ci-generator gitlab-ci-validator helm-generator helm-validator implementation-plan-splitter jenkinsfile-generator jenkinsfile-validator; do
  echo ""
  echo "========================================="
  echo "Processing: $skill"
  echo "========================================="
  ./scripts/audit-skill.sh "$skill" || {
    echo "❌ FAILED: $skill"
    read -p "Continue? (y/N) " -n 1 -r; echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 1
  }
done
```

### Quality Check

```bash
. scripts/audit-helpers.sh
for skill in github-actions-validator github-copilot-models gitlab-api gitlab-ci-generator gitlab-ci-validator helm-generator helm-validator implementation-plan-splitter jenkinsfile-generator jenkinsfile-validator; do
  check_audit "$skill"
done
```

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 3 complete (skills 26-35)"
```

### Checklist

- [ ] All 10 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes

---

## Batch 4: Skills 36-45 (10 skills)

**Skills:**
1. journal-entry-creator
2. k8s-debug
3. k8s-yaml-generator
4. k8s-yaml-validator
5. logql-generator
6. makefile-generator
7. makefile-validator
8. markdown-authoring
9. mise-complete
10. moscow-prioritization

### Execution

```bash
for skill in journal-entry-creator k8s-debug k8s-yaml-generator k8s-yaml-validator logql-generator makefile-generator makefile-validator markdown-authoring mise-complete moscow-prioritization; do
  echo ""
  echo "========================================="
  echo "Processing: $skill"
  echo "========================================="
  ./scripts/audit-skill.sh "$skill" || {
    echo "❌ FAILED: $skill"
    read -p "Continue? (y/N) " -n 1 -r; echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 1
  }
done
```

### Quality Check

```bash
. scripts/audit-helpers.sh
for skill in journal-entry-creator k8s-debug k8s-yaml-generator k8s-yaml-validator logql-generator makefile-generator makefile-validator markdown-authoring mise-complete moscow-prioritization; do
  check_audit "$skill"
done
```

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 4 complete (skills 36-45)"
```

### Checklist

- [ ] All 10 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes

---

## Batch 5: Skills 46-55 (10 skills)

**Skills:**
1. nx-biome-integration
2. nx-bun-integration
3. nx-generators
4. nx-vite-integration
5. nx-workspace-patterns
6. opencode-config
7. plain-english
8. promql-generator
9. promql-validator
10. sop-structure

### Execution

**Using batch-audit.sh (recommended):**
```bash
sh skills/skill-quality-auditor/scripts/batch-audit.sh nx-biome-integration nx-bun-integration nx-generators \
  nx-vite-integration nx-workspace-patterns opencode-config plain-english \
  promql-generator promql-validator sop-structure
```

### Quality Check & Score Analysis

```bash
# Check all audits completed successfully
. scripts/audit-helpers.sh
for skill in nx-biome-integration nx-bun-integration nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english promql-generator promql-validator sop-structure; do
  check_audit "$skill"
done

# Calculate score distribution
echo ""
echo "Score distribution:"
for skill in nx-biome-integration nx-bun-integration nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english promql-generator promql-validator sop-structure; do
  jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERROR"
done | sort | uniq -c

# Calculate average score
echo ""
echo "Average score:"
total=0
count=0
for skill in nx-biome-integration nx-bun-integration nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english promql-generator promql-validator sop-structure; do
  score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null)
  if [ -n "$score" ] && [ "$score" != "null" ]; then
    total=$((total + score))
    count=$((count + 1))
  fi
done
if [ $count -gt 0 ]; then
  avg=$((total / count))
  echo "$avg/120"
fi
```

**Verify halt criteria:**
- Document results in `.context/analysis/phase2-issues.md`
- Check if batch meets PASS/WARNING/HALT thresholds
- Decision: Continue to Batch 6 / Pause and investigate / Abort Phase 2

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 5 complete (skills 46-55)"
```

### Checklist

- [ ] All 10 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes

---

## Batch 6: Skills 56-63 (8 skills - FINAL BATCH)

**Skills:**
1. software-design-principles
2. supabase-postgres-best-practices
3. terraform-generator
4. terraform-validator
5. test-driven-development
6. typescript-advanced
7. ui-debug-workflow
8. user-story-writing

### Execution

**Using batch-audit.sh (recommended):**
```bash
sh skills/skill-quality-auditor/scripts/batch-audit.sh software-design-principles supabase-postgres-best-practices \
  terraform-generator terraform-validator test-driven-development typescript-advanced \
  ui-debug-workflow user-story-writing
```

### Quality Check & Score Analysis

```bash
# Check all audits completed successfully
. scripts/audit-helpers.sh
for skill in software-design-principles supabase-postgres-best-practices terraform-generator terraform-validator test-driven-development typescript-advanced ui-debug-workflow user-story-writing; do
  check_audit "$skill"
done

# Calculate score distribution
echo ""
echo "Score distribution:"
for skill in software-design-principles supabase-postgres-best-practices terraform-generator terraform-validator test-driven-development typescript-advanced ui-debug-workflow user-story-writing; do
  jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERROR"
done | sort | uniq -c

# Calculate average score
echo ""
echo "Average score:"
total=0
count=0
for skill in software-design-principles supabase-postgres-best-practices terraform-generator terraform-validator test-driven-development typescript-advanced ui-debug-workflow user-story-writing; do
  score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null)
  if [ -n "$score" ] && [ "$score" != "null" ]; then
    total=$((total + score))
    count=$((count + 1))
  fi
done
if [ $count -gt 0 ]; then
  avg=$((total / count))
  echo "$avg/120"
fi
```

**Verify halt criteria:**
- Document results in `.context/analysis/phase2-issues.md`
- Check if batch meets PASS/WARNING/HALT thresholds
- This is the final batch - comprehensive verification follows

### Commit Checkpoint

```bash
git add .context/audits/
git commit -m "audit: batch 6 complete (skills 56-63) - PHASE 2 COMPLETE"
```

### Checklist

- [ ] All 8 skills audited successfully
- [ ] No missing analysis.md or audit.json files
- [ ] All scores within expected range
- [ ] Changes committed to git
- [ ] Time logged: _____ minutes
- [ ] **PHASE 2 COMPLETE** - Proceed to wrap-up

---

## Phase 2 Wrap-Up Tasks

### 1. Verify All 63 Skills Audited

**Improved skill discovery (handles edge cases):**

```bash
echo "========================================="
echo "Verifying all skills have audits..."
echo "========================================="
echo ""

missing_audits=""
audited_count=0
total_count=0

# Find all skills with SKILL.md files
find skills -maxdepth 2 -name "SKILL.md" | while read -r skill_file; do
  skill_dir=$(dirname "$skill_file")
  skill_name=$(basename "$skill_dir")
  
  total_count=$((total_count + 1))
  
  if [ -L ".context/audits/$skill_name/latest" ]; then
    audited_count=$((audited_count + 1))
    echo "✓ $skill_name"
  else
    echo "✗ MISSING: $skill_name"
    missing_audits="$missing_audits $skill_name"
  fi
done

echo ""
echo "========================================="
echo "Verification Summary"
echo "========================================="
echo "Total skills found: $total_count"
echo "Audited: $audited_count"
echo "Missing: $((total_count - audited_count))"

if [ -n "$missing_audits" ]; then
  echo ""
  echo "Skills without audits:$missing_audits"
  echo ""
  echo "To audit missing skills:"
  echo "  sh skills/skill-quality-auditor/scripts/batch-audit.sh$missing_audits"
  exit 1
else
  echo ""
  echo "✅ All skills have been audited!"
fi
```

**Alternative: Quick count verification (63 expected):**
```bash
# Count skills with audits
audit_count=$(find .context/audits -maxdepth 2 -name "latest" -type l | wc -l)
echo "Total audits created: $audit_count"

if [ "$audit_count" -eq 63 ]; then
  echo "✅ All 63 skills audited"
else
  echo "⚠️  Expected 63, found $audit_count"
fi
```

### 2. Comprehensive Quality Analysis

**Generate overall statistics:**

```bash
echo ""
echo "========================================="
echo "Phase 2 Overall Quality Analysis"
echo "========================================="
echo ""

# Count grades across all Phase 2 skills (57 skills)
echo "Grade distribution (all 57 Phase 2 skills):"
phase2_count=0
for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do
  # Skip Phase 1 skills
  case "$skill" in
    skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing)
      continue
      ;;
  esac
  
  if [ -f ".context/audits/$skill/latest/audit.json" ]; then
    jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERROR"
    phase2_count=$((phase2_count + 1))
  fi
done | sort | uniq -c

echo ""
echo "Phase 2 skills processed: $phase2_count/57"

echo ""
echo "Average score:"
total=0
count=0
for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do
  # Skip Phase 1 skills
  case "$skill" in
    skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing)
      continue
      ;;
  esac
  
  if [ -f ".context/audits/$skill/latest/audit.json" ]; then
    score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null)
    if [ -n "$score" ] && [ "$score" != "null" ]; then
      total=$((total + score))
      count=$((count + 1))
    fi
  fi
done

if [ $count -gt 0 ]; then
  avg=$((total / count))
  echo "$avg/120 (across $count Phase 2 skills)"
else
  echo "ERROR: No valid scores found"
fi

echo ""
echo "Detailed grade breakdown (Phase 2 only):"
echo "  A-grade (≥108): $(for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do case "$skill" in skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing) continue ;; esac; [ -f ".context/audits/$skill/latest/audit.json" ] && jq -r 'select(.total >= 108) | .grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null; done | wc -l)"
echo "  B-grade (96-107): $(for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do case "$skill" in skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing) continue ;; esac; [ -f ".context/audits/$skill/latest/audit.json" ] && jq -r 'select(.total >= 96 and .total < 108) | .grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null; done | wc -l)"
echo "  C-grade (84-95): $(for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do case "$skill" in skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing) continue ;; esac; [ -f ".context/audits/$skill/latest/audit.json" ] && jq -r 'select(.total >= 84 and .total < 96) | .grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null; done | wc -l)"
echo "  D-grade (72-83): $(for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do case "$skill" in skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing) continue ;; esac; [ -f ".context/audits/$skill/latest/audit.json" ] && jq -r 'select(.total >= 72 and .total < 84) | .grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null; done | wc -l)"
echo "  F-grade (<72): $(for skill in $(find .context/audits -maxdepth 1 -type d -not -name ".*" -not -name "audits" | xargs -I {} basename {}); do case "$skill" in skill-quality-auditor|acceptance-criteria|ansible-generator|nx-executors|dockerfile-generator|bdd-testing) continue ;; esac; [ -f ".context/audits/$skill/latest/audit.json" ] && jq -r 'select(.total < 72) | .grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null; done | wc -l)"
```

**Document in phase2-issues.md:**
- Complete the "Score Distribution by Batch" section
- Verify overall quality thresholds were met across all 57 Phase 2 skills
- Note any patterns or systemic concerns discovered

### 3. Generate Phase 2 Summary

Complete `.context/analysis/phase2-issues.md` with final data:

- Populate all batch rows in "Score Distribution by Batch" table
- Complete "Time Tracking" table with actuals
- Document all retry attempts in "Retry Log"
- Summarize process improvements for Phase 3

Create additional summary if needed:

```markdown
# Phase 2 Batch Processing Summary

**Date:** 2026-03-02
**Total Skills Processed:** 57 (Batches 1-6, excluding 6 Phase 1 skills)
**Total Time:** [actual hours]

## Batch Results

| Batch | Skills | Time | Failures | Retries | Notes |
|-------|--------|------|----------|---------|-------|
| 1 (6-15) | 10 | [X]min | [Y] | [Z] | [notes] |
| 2 (16-25) | 10 | [X]min | [Y] | [Z] | [notes] |
| 3 (26-35) | 10 | [X]min | [Y] | [Z] | [notes] |
| 4 (36-45) | 10 | [X]min | [Y] | [Z] | [notes] |
| 5 (46-55) | 10 | [X]min | [Y] | [Z] | [notes] |
| 6 (56-63) | 7 | [X]min | [Y] | [Z] | [notes] |
| **TOTAL** | **57** | **[T]hr** | **[F]** | **[R]** | |

## Final Quality Metrics

- A-grade skills: [X]/57 ([Y]%)
- B-grade skills: [X]/57 ([Y]%)
- C-grade skills: [X]/57 ([Y]%)
- D-grade skills: [X]/57 ([Y]%)
- F-grade skills: [X]/57 ([Y]%)
- Average score: [X]/120

## Issues Encountered

1. [issue description and resolution]
2. [issue description and resolution]

## Success Rate

- Skills successfully audited: [X]/57
- Skills requiring retry: [Y]
- Average time per skill: [Z] minutes
- Overall quality threshold: PASS / WARNING / concern areas noted
```

### 4. Phase 2 Success Criteria (Overall)

- [ ] All 57 remaining skills have proper audit structure
- [ ] Latest symlinks point to current audit for all Phase 2 skills
- [ ] Remediation plans exist for skills scoring <108/120
- [ ] phase2-issues.md fully populated with data
- [ ] All failures documented and resolved (or documented as unresolved with reason)
- [ ] Quality thresholds met (or documented if WARNING/HALT status)
- [ ] Ready to proceed to Phase 3 (remediation)

---

## Rollback & Error Recovery

### Safe Rollback for Specific Batch

If a batch corrupts data or needs to be re-run:

```bash
# Example: Rolling back Batch 3
batch_num=3
batch_skills="cfn-behavior-testing cfn-template-compare code-reviewer colyseus-multiplayer commanderjs creating-eval-scenarios docker-containerization dotenvx-complete extending-nx-plugins find-skills"

# Move failed batch to quarantine (preserves Phase 1 + other batches)
mkdir -p .context/audits/.failed-batch-${batch_num}-$(date +%Y-%m-%d-%H%M%S)
for skill in $batch_skills; do
  if [ -d ".context/audits/$skill/$(date +%Y-%m-%d)" ]; then
    echo "Quarantining $skill audit from today..."
    mv ".context/audits/$skill/$(date +%Y-%m-%d)" \
       ".context/audits/.failed-batch-${batch_num}-$(date +%Y-%m-%d-%H%M%S)/"
  fi
done

# Alternatively, use git to restore specific paths
git restore .context/audits/<skill-name>

# Re-run the batch
sh skills/skill-quality-auditor/scripts/batch-audit.sh $batch_skills
```

### Full Phase 2 Rollback (Nuclear Option)

**ONLY if Phase 2 must be completely aborted and restarted:**

```bash
# Restore from pre-Phase 2 backup (preserves Phase 1)
BACKUP_DIR=$(ls -td .context/audits.backup-* | head -1)
echo "Restoring from: $BACKUP_DIR"

# Move current audits to quarantine (don't delete - might need for debugging)
mv .context/audits .context/audits.corrupted-$(date +%Y-%m-%d-%H%M%S)

# Restore backup
cp -r "$BACKUP_DIR" .context/audits

# Verify Phase 1 audits still exist
for skill in skill-quality-auditor acceptance-criteria ansible-generator nx-executors dockerfile-generator bdd-testing; do
  if [ -L ".context/audits/$skill/latest" ]; then
    echo "✓ Phase 1 skill preserved: $skill"
  else
    echo "⚠️  Phase 1 skill missing: $skill"
  fi
done
```

**Important:** Never use `rm -rf .context/audits` - always move to quarantine first!

---

## Notes for Execution

### Best Practices

1. **Pause between batches** - Don't run all 6 batches back-to-back without verification. Check quality thresholds after each batch using the PASS/WARNING/HALT criteria.

2. **Use batch-audit.sh script** - Prefer the wrapper script over manual loops. It provides better failure tracking and prompts for decisions.

3. **Document issues immediately** - When failures occur, log them in `.context/analysis/phase2-issues.md` BEFORE continuing. Include error messages, not just "failed".

4. **Commit after each batch** - Git commits provide natural rollback points. Use descriptive messages like "audit: batch 3 complete (skills 26-35)".

5. **Time tracking** - Log actual time for each batch in phase2-issues.md time tracking table. This data is critical for future planning.

6. **Monitor score distributions** - If you see concerning patterns (e.g., many F-grades, very low averages), investigate before continuing. Don't blindly push through all batches.

7. **Keep backups safe** - The pre-phase backup in `.context/audits.backup-*` is your safety net. Don't delete it until Phase 2 is successfully complete.

8. **Never use rm -rf on audits** - Always move to quarantine directories for debugging. Storage is cheap; recreating lost audits is expensive.

### Parallel Processing (Advanced - Use with Caution)

**Only consider after Batch 1 succeeds completely:**

```bash
# Test parallel processing on a small set first
echo "agents-md ansible-validator azure-pipelines-generator" | \
  xargs -n 1 -P 3 ./scripts/audit-skill.sh

# If stable, can parallelize batch processing (max 3 concurrent)
# But lose interactive failure handling
```

**Risks of parallel execution:**
- Harder to debug failures
- No interactive "Continue?" prompts
- Potential file system race conditions
- Less visibility into progress

**Recommendation:** Stick with sequential batch-audit.sh unless time is critical and script is proven stable.

### Failed Skill Retry Strategy

If skills fail during a batch:

1. **Complete the batch** - Let batch-audit.sh finish processing remaining skills
2. **Document failures** - Add to phase2-issues.md Failure Tracking table
3. **Investigate root cause** - Check error messages, verify skill structure
4. **Retry individually** - Use `./scripts/audit-skill.sh <skill-name>` for targeted retry
5. **Batch retry if needed** - If multiple skills failed with same error, fix root cause then retry all:
   ```bash
   sh skills/skill-quality-auditor/scripts/batch-audit.sh skill1 skill2 skill3
   ```

### Quality Threshold Responses

**If batch gets WARNING status:**
- Document the concern in phase2-issues.md
- Look for patterns (e.g., all validator skills score low)
- Continue but watch next batch closely
- Consider spot-checking a few low-scoring audits manually

**If batch gets HALT status:**
- Stop immediately - don't proceed to next batch
- Investigate root cause (script bug? skill content issues? scoring threshold wrong?)
- Review several low-scoring skills manually
- Decide: Fix evaluate.sh and re-run, adjust thresholds, or continue with documentation

---

**Next Step:** Execute Phase 1 successfully, then proceed to batch processing  
**Owner:** AI Agent (OpenCode)  
**Status:** PENDING PHASE 1 COMPLETION
