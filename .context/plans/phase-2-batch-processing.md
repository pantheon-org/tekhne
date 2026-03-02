# Phase 2: Batch Processing

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** 4-6 hours  
**Status:** PENDING PHASE 1 COMPLETION

---

## Overview

Apply validated audit process to all 58 remaining skills using automated batch processing.

**Total Skills:** 63  
**Phase 1 Skills:** 5 (acceptance-criteria, ansible-generator, nx-executors, dockerfile-generator, bdd-testing)  
**Phase 2 Skills:** 58 (all remaining skills)

## Prerequisites

- [ ] Phase 1 completed successfully
- [ ] Process validated on 5 representative skills
- [ ] Time estimates refined based on Phase 1 actuals
- [ ] Automation script tested and working

---

## Pre-Phase Setup

### 1. Backup Existing Audits

```bash
# Create backup before running batch processing
cp -r .context/audits .context/audits.backup-$(date +%Y-%m-%d)
echo "Backup created: .context/audits.backup-$(date +%Y-%m-%d)"
```

### 2. Verify Automation Script

**Location:** `scripts/audit-skill.sh` (already created in Phase 1 improvements)

The script:
- Checks dependencies (jq)
- Runs `evaluate.sh` with `--json --store` flags
- Verifies symlink creation (handled by evaluate.sh)
- Validates required files exist
- Uses correct JSON fields (`.total`, `.grade`)
- Uses correct threshold (108 for A grade)

### 3. Test Automation Script

```bash
# Test on a Phase 1 skill to verify it works
./scripts/audit-skill.sh acceptance-criteria
# Should complete without errors and show: ✅ Audit complete
```

### 4. Create Batch Helper Functions

Create `scripts/audit-helpers.sh`:

```bash
#!/usr/bin/env sh
# Helper functions for batch audit processing

# Check audit quality for a single skill
check_audit() {
  skill="$1"
  if [ -L ".context/audits/$skill/latest" ]; then
    score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERR")
    grade=$(jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERR")
    if [ "$score" = "ERR" ]; then
      echo "✗ $skill: INVALID AUDIT"
    else
      echo "✓ $skill: $score/120 ($grade)"
    fi
  else
    echo "✗ $skill: MISSING AUDIT"
  fi
}

# Export for use in subshells
export -f check_audit
```

Make it executable:
```bash
chmod +x scripts/audit-helpers.sh
```

- Quality checks using helper functions
- Git commit checkpoint after each batch
- Consistent checklist

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

```bash
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
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
for skill in agents-md ansible-validator azure-pipelines-generator azure-pipelines-validator bash-script-generator bash-script-validator biome-complete bun-development cdk-nag cfn-behavior-validator; do
  check_audit "$skill"
done
```

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
10. skill-quality-auditor

### Execution

```bash
for skill in nx-biome-integration nx-bun-integration nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english promql-generator promql-validator skill-quality-auditor; do
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
for skill in nx-biome-integration nx-bun-integration nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english promql-generator promql-validator skill-quality-auditor; do
  check_audit "$skill"
done
```

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
2. terraform-generator
3. terraform-validator
4. terragrunt-generator
5. terragrunt-validator
6. test-driven-development
7. typescript-advanced
8. ui-debug-workflow

### Execution

```bash
for skill in software-design-principles terraform-generator terraform-validator terragrunt-generator terragrunt-validator test-driven-development typescript-advanced ui-debug-workflow; do
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
for skill in software-design-principles terraform-generator terraform-validator terragrunt-generator terragrunt-validator test-driven-development typescript-advanced ui-debug-workflow; do
  check_audit "$skill"
done
```

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

### Final Verification

```bash
# Generate complete list of all audited skills
find .context/audits -name "latest" -type l | wc -l
# Should output: 63

# Find any missing audits
cd skills
for skill in */; do
  skill_name=$(basename "$skill")
  if [ ! -L "../.context/audits/$skill_name/latest" ]; then
    echo "MISSING: $skill_name"
  fi
done
cd ..
```

### Generate Phase 2 Summary

Create `.context/analysis/phase2-summary.md`:

```markdown
# Phase 2 Batch Processing Summary

**Date:** 2026-03-02
**Total Skills Processed:** 58 (Batches 1-6)
**Total Time:** [actual]

## Batch Results

| Batch | Skills | Time | Failures | Notes |
|-------|--------|------|----------|-------|
| 1 | 10 | [time] | [count] | [notes] |
| 2 | 10 | [time] | [count] | [notes] |
| 3 | 10 | [time] | [count] | [notes] |
| 4 | 10 | [time] | [count] | [notes] |
| 5 | 10 | [time] | [count] | [notes] |
| 6 | 8 | [time] | [count] | [notes] |

## Issues Encountered

1. [issue description]
2. [issue description]

## Success Rate

- Skills successfully audited: X/58
- Skills requiring manual intervention: Y
- Average time per skill: Z minutes
```

### Phase 2 Success Criteria (Overall)

- [ ] All 58 remaining skills have proper audit structure
- [ ] Latest symlinks point to current audit
- [ ] Remediation plans exist for skills scoring <108/120
- [ ] Phase 2 summary document created
- [ ] All failures documented and addressed

---

## Notes for Execution

1. **Pause between batches** - Don't run all 6 batches back-to-back. Verify quality after each batch.

2. **Use controlled parallelism carefully** - Only use `xargs -P 3` parallel processing after batch 1 if the script is proven stable.

3. **Document issues immediately** - If something breaks, create `.context/analysis/phase2-issues.md` and log it before continuing.

4. **Commit frequently** - After each batch completes successfully, commit the audit files with descriptive messages.

5. **Time tracking** - Log actual time for each batch in checklist to improve estimates.

6. **Rollback strategy** - If a batch corrupts data:
   ```bash
   # Restore from backup
   rm -rf .context/audits
   cp -r .context/audits.backup-YYYY-MM-DD .context/audits
   
   # Or use git
   git restore .context/audits
   ```

---

**Next Step:** Execute Phase 1, then proceed to batch processing  
**Owner:** AI Agent (OpenCode)  
**Status:** PENDING PHASE 1 COMPLETION
