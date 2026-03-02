# Phase 4: Process Improvement

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** 30-45 minutes (see breakdown below)  
**Status:** PENDING PHASE 3 COMPLETION

---

## Overview

Update repository documentation and tooling to prevent future bypass of audit standards.

## Duration Breakdown

**Without optional pre-commit hook:** 30 minutes
- Task 4.1 (AGENTS.md update): 10 min
- Task 4.2 (SKILL.md update): 10 min
- Task 4.3 (Verify script): 5 min
- Wrap-up: 5 min

**With optional pre-commit hook:** 45 minutes
- Tasks 4.1-4.3: 25 min
- Task 4.4 (Pre-commit hook): 15 min
- Wrap-up: 5 min

**Recommended:** 30 minutes (skip pre-commit hook initially, add later if needed)

## Prerequisites

- [ ] Phase 1 completed successfully
- [ ] Phase 2 completed successfully
- [ ] Phase 3 completed successfully
- [ ] Lessons learned documented
- [ ] Process improvements identified

---

## Task 4.1: Update AGENTS.md

**File:** `AGENTS.md`

### Changes to Make

Add new section after existing "Validation Commands" section (line ~43, before "Git Hooks" section):

```markdown
## Skill Quality Audits

All skills MUST be audited using the skill-quality-auditor framework before publishing or version bumps:

### Required Audit Workflow

**1. Run evaluation:**

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json --store
```

**2. Verify structure created:**

```bash
ls .context/audits/<skill-name>/latest/
# Expected: analysis.md, audit.json, remediation-plan.md (if score < 108)
```

**3. Review audit results:**

- Check `.context/audits/<skill-name>/latest/audit.json` for score and grade
- Target: Grade A (≥108/120)
- Minimum acceptable: Grade B (≥96/120)

**4. Address issues:**

- If score < 108/120, follow `remediation-plan.md`
- Re-audit after fixes
- Iterate until target grade achieved

**5. Verify audit before publishing:**

```bash
# Audit must be recent (within 30 days of publish)
find .context/audits/<skill-name>/ -name "audit.json" -mtime -30

# Or use compliance check script
./scripts/check-skill-audit-status.sh
```

### Audit Recency

By default, audits older than 30 days trigger a warning. This can be configured:

```bash
export SKILL_AUDIT_MAX_AGE=60  # Allow 60-day-old audits
```

**Rationale for 30 days:** Ensures audits reflect recent skill changes during active development. Stable skills may use longer periods (60-90 days).

### Prohibited Practices

❌ **DO NOT** use ad-hoc audit methods like `tessl skill review` without storing results in `.context/audits/` structure

❌ **DO NOT** bypass skill-quality-auditor framework for "quick checks"

❌ **DO NOT** publish skills without corresponding audit in `.context/audits/<skill-name>/`

❌ **DO NOT** commit skill changes without verifying audit exists and is recent

### Rationale

Structured audits provide:
- Consistent quality bar across all skills
- Dimensional analysis for targeted improvements
- Historical baseline for tracking improvements
- Actionable remediation plans for low-scoring skills
- Audit trail for compliance and review

See `skills/skill-quality-auditor/SKILL.md` for complete audit workflow documentation.
```

**Fallback:** If "Validation Commands" section doesn't exist (future AGENTS.md changes), insert before "Git Hooks" section or at end of repository rules.

### Execution

1. Read current AGENTS.md
2. Add new section in appropriate location
3. Verify formatting and links
4. Test commands in examples

### Success Criteria

- [ ] New section added to AGENTS.md
- [ ] All commands tested and verified
- [ ] Links and references correct
- [ ] No markdown lint errors

---

## Task 4.2: Update skill-quality-auditor/SKILL.md

**File:** `skills/skill-quality-auditor/SKILL.md`

### Changes to Make

Add new section "Common Mistakes and Anti-Patterns":

```markdown
## Common Mistakes and Anti-Patterns

### ❌ Anti-Pattern 1: Ad-Hoc Audit Methods

**What it looks like:**
```bash
# Running tessl skill review and saving output to flat file
tessl skill review skills/my-skill > .context/my-skill-audit.txt
```

**Why it's wrong:**
- No structured data (audit.json) for analysis
- No dimensional breakdown
- Cannot track improvements over time
- Doesn't follow repository standards

**Correct approach:**
```bash
# Use skill-quality-auditor framework
sh skills/skill-quality-auditor/scripts/evaluate.sh my-skill --json --store
# Creates .context/audits/my-skill/<date>/ structure
```

### ❌ Anti-Pattern 2: Bypassing Audit for "Simple" Skills

**What it looks like:**
"This skill is simple, I don't need to run a full audit"

**Why it's wrong:**
- Inconsistent quality bar across skills
- No baseline for future comparison
- Simple skills can have complex issues
- Breaks repository compliance

**Correct approach:**
Audit ALL skills, regardless of perceived complexity

### ❌ Anti-Pattern 3: Ignoring Remediation Plans

**What it looks like:**
Publishing skills with score < 108/120 without addressing issues

**Why it's wrong:**
- Codifies low-quality patterns
- Harder to fix after publication
- Sets poor example for other skills

**Correct approach:**
1. Review remediation-plan.md
2. Address issues systematically
3. Re-audit after fixes
4. Only publish when target grade achieved

### ✅ Correct Workflow Example

```bash
# 1. Initial audit
sh skills/skill-quality-auditor/scripts/evaluate.sh my-skill --json --store

# 2. Review results
cat .context/audits/my-skill/latest/audit.json | jq '.total, .grade'

# 3. If score < 108, review remediation plan
cat .context/audits/my-skill/latest/remediation-plan.md

# 4. Fix issues in skill

# 5. Re-audit
sh skills/skill-quality-auditor/scripts/evaluate.sh my-skill --json --store

# 6. Repeat until grade A achieved

# 7. Publish with confidence
```
```

### Execution

1. Read current skill-quality-auditor/SKILL.md
2. Add anti-patterns section
3. Include concrete examples
4. Verify formatting

### Success Criteria

- [ ] Anti-patterns section added
- [ ] Examples are clear and actionable
- [ ] Correct workflow documented
- [ ] No markdown lint errors

---

## Task 4.3: Verify Audit Status Check Command

**Goal:** Verify the audit compliance verification script exists and works correctly, then optionally add a slash command

The script exists at `scripts/check-skill-audit-status.sh` and includes:

- ✅ jq dependency check
- ✅ Terminal color detection
- ✅ Symlink target validation (not just symlink exists)
- ✅ Correct JSON fields (`.total`, `.grade`)
- ✅ Clear output with color coding
- ✅ Appropriate exit codes
- ✅ Helpful remediation instructions
- ✅ 30-day recency threshold (configurable via SKILL_AUDIT_MAX_AGE)

### Verify Script Exists and Works

```bash
# 1. Verify script exists
if [ -f scripts/check-skill-audit-status.sh ]; then
  echo "✅ Script exists"
  chmod +x scripts/check-skill-audit-status.sh
else
  echo "❌ Script missing - check scripts/ directory"
  exit 1
fi

# 2. Test the script
./scripts/check-skill-audit-status.sh

# Expected output (before Phase 1-2 execution):
# - Lists all skills with compliance status
# - Color-coded: green (compliant), yellow (outdated >30 days), red (missing)
# - Summary with counts (likely all missing at this stage)
# - Exit code 1 since audits don't exist yet

echo ""
echo "Exit code: $?"
# Should be 1 if any skills are non-compliant
```

### Optional: Add Slash Command

Add to `opencode.json` (creating `commands` object if it doesn't exist):

```json
{
  "$schema": "https://opencode.ai/schemas/opencode.json",
  "commands": {
    "check-skill-audits": {
      "description": "Verify audit compliance for all skills",
      "prompt": "Run the script at scripts/check-skill-audit-status.sh and report the results. Show which skills are missing audits, which are outdated, and overall compliance rate."
    }
  },
  "instructions": "",
  "plugin": [
    // ... existing plugins ...
  ]
}
```

**Note:** If `opencode.json` doesn't have a `commands` object, create it at the root level (same level as `plugin`, `instructions`, etc.).

### Success Criteria

- [ ] Script verified and executable
- [ ] Test run completed (shows current compliance state)
- [ ] Correctly identifies compliant/outdated/missing audits
- [ ] Clear output with color coding
- [ ] Returns appropriate exit code (1 if issues found)
- [ ] Optional slash command added to opencode.json (if desired)

---

## Task 4.4: Add Pre-Commit Hook (Optional)

**Goal:** Prevent commits that modify skills without corresponding audit

**⚠️ Important Considerations:**

- **Performance Impact:** Runs checks on every commit touching skills/, which can slow down workflow
- **Developer Friction:** May frustrate developers making minor typo fixes
- **Recommendation:** Start without this hook, add later if audit bypasses become a recurring problem
- **Alternative:** Use non-blocking warning mode (shown below)

### Implementation (Blocking Mode)

Add to `.lefthook.yml`:

```yaml
pre-commit:
  parallel: true
  commands:
    # ... existing commands ...
    
    skill-audit-check:
      glob: "skills/**/*"
      run: |
        # Extract modified skill directories (handles nested paths correctly)
        changed_skills=$(git diff --cached --name-only | \
          grep "^skills/[^/][^/]*/" | \
          cut -d'/' -f2 | \
          sort -u | \
          while read skill; do
            # Verify it's a real skill directory with SKILL.md
            [ -f "skills/$skill/SKILL.md" ] && echo "$skill"
          done
        )
        
        if [ -n "$changed_skills" ]; then
          echo "🔍 Checking audit compliance for modified skills..."
          
          # Use configurable max age (default 30 days)
          AUDIT_MAX_AGE_DAYS="${SKILL_AUDIT_MAX_AGE:-30}"
          
          for skill in $changed_skills; do
            if [ ! -L ".context/audits/$skill/latest" ]; then
              echo "❌ Missing audit for skill: $skill"
              echo "   Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill --json --store"
              exit 1
            fi
            
            # Check if audit is recent
            audit_file=".context/audits/$skill/latest/audit.json"
            if [ ! -f "$audit_file" ]; then
              echo "❌ Missing audit.json for skill: $skill"
              exit 1
            fi
            
            if find "$audit_file" -mtime "+$AUDIT_MAX_AGE_DAYS" 2>/dev/null | grep -q .; then
              echo "⚠️  Audit for $skill is >$AUDIT_MAX_AGE_DAYS days old. Consider re-auditing."
              echo "   Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill --json --store"
            fi
          done
          
          echo "✅ All modified skills have recent audits"
        fi
```

### Implementation (Non-Blocking Warning Mode - Recommended)

Use this softer approach initially:

```yaml
    skill-audit-check:
      glob: "skills/**/*"
      run: |
        # Same skill extraction logic as above
        changed_skills=$(git diff --cached --name-only | \
          grep "^skills/[^/][^/]*/" | \
          cut -d'/' -f2 | \
          sort -u | \
          while read skill; do
            [ -f "skills/$skill/SKILL.md" ] && echo "$skill"
          done
        )
        
        if [ -n "$changed_skills" ]; then
          echo "🔍 Checking audit compliance for modified skills..."
          
          missing_audits=""
          AUDIT_MAX_AGE_DAYS="${SKILL_AUDIT_MAX_AGE:-30}"
          
          for skill in $changed_skills; do
            if [ ! -L ".context/audits/$skill/latest" ]; then
              echo "⚠️  WARNING: Missing audit for skill: $skill"
              missing_audits="$missing_audits $skill"
            elif [ ! -f ".context/audits/$skill/latest/audit.json" ]; then
              echo "⚠️  WARNING: Invalid audit for skill: $skill"
              missing_audits="$missing_audits $skill"
            elif find ".context/audits/$skill/latest/audit.json" -mtime "+$AUDIT_MAX_AGE_DAYS" 2>/dev/null | grep -q .; then
              echo "ℹ️  INFO: Audit for $skill is >$AUDIT_MAX_AGE_DAYS days old"
            fi
          done
          
          if [ -n "$missing_audits" ]; then
            echo ""
            echo "⚠️  WARNING: Some skills missing audits (not blocking commit)"
            echo "   Run audit with: sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json --store"
            echo ""
            echo "   To make this check strict (blocking), set: export STRICT_AUDIT_CHECK=true"
            
            # Only fail if strict mode enabled
            if [ "${STRICT_AUDIT_CHECK:-false}" = "true" ]; then
              echo "   STRICT_AUDIT_CHECK=true - blocking commit"
              exit 1
            fi
          else
            echo "✅ All modified skills have recent audits"
          fi
        fi
```

### Execution

**Choose one approach:**

1. **No hook (recommended initially):** Skip this task, rely on manual checks and documentation
2. **Non-blocking warnings:** Add warning mode hook (allows commit but reminds developer)
3. **Blocking enforcement:** Add strict mode hook (prevents commit without audit)

**Testing the hook:**

```bash
# 1. Add skill-audit-check to .lefthook.yml (your chosen mode)

# 2. Install/update lefthook
lefthook install

# 3. Test with a skill modification (without audit)
echo "# Test change" >> skills/test-skill/SKILL.md
git add skills/test-skill/SKILL.md
git commit -m "test: verify pre-commit hook"

# Expected behavior:
# - Blocking mode: Commit fails with error about missing audit
# - Warning mode: Commit succeeds with warning message
# - No hook: Commit succeeds silently

# 4. Clean up test
git reset HEAD~1
git restore skills/test-skill/SKILL.md

# 5. Test with a skill that HAS audit (after Phase 1-2)
echo "# Test change" >> skills/acceptance-criteria/SKILL.md
git add skills/acceptance-criteria/SKILL.md
git commit -m "test: verify pre-commit hook with audit"

# Expected: Commit succeeds (both modes)

# 6. Clean up
git reset HEAD~1
git restore skills/acceptance-criteria/SKILL.md
```

### Success Criteria

**If implementing (either mode):**
- [ ] Hook added to .lefthook.yml with chosen approach
- [ ] Lefthook installed/updated
- [ ] Tested with skill without audit (blocking fails, warning warns)
- [ ] Tested with skill with audit (both modes allow commit)
- [ ] SKILL_AUDIT_MAX_AGE configurable via environment variable
- [ ] STRICT_AUDIT_CHECK toggle works (if using warning mode)

**If skipping:**
- [ ] Decision documented (rely on manual checks and documentation)
- [ ] Team informed to use `./scripts/check-skill-audit-status.sh` manually

---

## Phase 4 Wrap-Up

### Final Verification

```bash
echo "========================================="
echo "Phase 4 Verification"
echo "========================================="
echo ""

# 1. Verify AGENTS.md updated
if grep -q "Skill Quality Audits" AGENTS.md; then
  echo "✅ AGENTS.md updated with audit requirements"
else
  echo "❌ AGENTS.md missing audit section"
fi

# 2. Verify skill-quality-auditor/SKILL.md updated
if grep -q "Common Mistakes and Anti-Patterns" skills/skill-quality-auditor/SKILL.md; then
  echo "✅ skill-quality-auditor/SKILL.md updated with anti-patterns"
else
  echo "❌ skill-quality-auditor/SKILL.md missing anti-patterns section"
fi

# 3. Verify check script works
if ./scripts/check-skill-audit-status.sh >/dev/null 2>&1; then
  echo "✅ check-skill-audit-status.sh verified (exit code: $?)"
else
  echo "⚠️  check-skill-audit-status.sh exited with code: $?"
  echo "   (Expected if audits don't exist yet)"
fi

# 4. Check if pre-commit hook added (optional)
if grep -q "skill-audit-check" .lefthook.yml 2>/dev/null; then
  echo "✅ Pre-commit hook added"
else
  echo "ℹ️  Pre-commit hook not added (optional)"
fi

# 5. Verify opencode.json slash command (optional)
if grep -q "check-skill-audits" opencode.json 2>/dev/null; then
  echo "✅ Slash command added to opencode.json"
else
  echo "ℹ️  Slash command not added (optional)"
fi

echo ""
echo "========================================="
echo "Phase 4 complete"
echo "========================================="
```

### Checklist

- [ ] AGENTS.md updated with audit requirements
- [ ] skill-quality-auditor/SKILL.md updated with anti-patterns
- [ ] Audit status check script verified and tested
- [ ] Pre-commit hook added (if implemented)
- [ ] All documentation changes committed
- [ ] Time logged for Phase 4 (add to Phase 3 lessons learned or track separately)

### Test the Improvements

```bash
# Run compliance check
./scripts/check-skill-audit-status.sh

# Expected output (before Phase 1-2 execution):
# - All skills shown as MISSING (red)
# - Summary: 0 compliant, 0 outdated, 63 missing
# - Exit code: 1 (issues found)

# Expected output (after Phase 1-2 execution):
# - All skills shown as COMPLIANT (green) or OUTDATED (yellow)
# - Summary: X compliant, Y outdated, 0 missing
# - Exit code: 0 (all compliant) or 1 (some outdated)
```

---

## Post-Remediation Checklist

**Note:** This comprehensive checklist spans all 4 phases and serves as the final verification after completing the entire remediation workflow (Phases 1-4). Consider moving this to `.context/plans/complete-skill-audit-remediation.md` as the final verification section for better discoverability.

### Structural Compliance

- [ ] All skills have `.context/audits/<skill-name>/<date>/` structure
  ```bash
  # Dynamic verification
  skill_count=$(find skills -maxdepth 2 -name "SKILL.md" | wc -l)
  audit_count=$(find .context/audits -maxdepth 2 -name "latest" -type l | wc -l)
  
  echo "Skills found: $skill_count"
  echo "Audits found: $audit_count"
  
  if [ "$skill_count" -eq "$audit_count" ]; then
    echo "✅ All skills audited ($skill_count/$skill_count)"
  else
    echo "⚠️  Missing audits: $((skill_count - audit_count))"
  fi
  ```
- [ ] All audit directories contain `analysis.md`, `audit.json`
- [ ] All skills with score <108 have `remediation-plan.md`
- [ ] All skills have `latest` symlink pointing to most recent audit

### Documentation

- [ ] Audit summary report exists (`.context/audits/summary-<DATE>.md`)
- [ ] Lessons learned document exists (`.context/analysis/audit-remediation-lessons-<DATE>.md`)
- [ ] Phase 1 and Phase 2 execution notes documented
- [ ] Tessl comparison completed OR explicitly skipped (optional)

### Process Improvements

- [ ] AGENTS.md updated with explicit audit requirements
- [ ] skill-quality-auditor/SKILL.md updated with anti-patterns section
- [ ] Audit status check command created and working
- [ ] Pre-commit hook added (optional)

### Quality Metrics

- [ ] Grade distribution calculated and documented
- [ ] Top 10 highest scoring skills identified
- [ ] Top 10 skills needing improvement identified
- [ ] Common patterns in low-scoring dimensions analyzed
- [ ] Dimensional averages calculated across all skills

### Communication

- [ ] Remediation plan marked as COMPLETE
- [ ] Summary shared with team/stakeholders (if applicable)
- [ ] Lessons learned reviewed
- [ ] Next steps identified

---

**Next Step:** Begin Task 4.1 - Update AGENTS.md  
**Owner:** AI Agent (OpenCode)  
**Status:** PENDING PHASE 3 COMPLETION
