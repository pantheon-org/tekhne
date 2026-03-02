# Phase 4: Process Improvement

**Date:** 2026-03-02  
**Parent Plan:** complete-skill-audit-remediation.md  
**Duration:** 30 minutes  
**Status:** PENDING PHASE 3 COMPLETION

---

## Overview

Update repository documentation and tooling to prevent future bypass of audit standards.

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

Add new section after "Validation Commands":

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
# Audit must be recent (within 7 days of publish)
find .context/audits/<skill-name>/ -name "audit.json" -mtime -7
```

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

## Task 4.3: Create Audit Status Verification Command

**Goal:** Create a command that quickly checks compliance status of all skills

The script has been created at `scripts/check-skill-audit-status.sh` with:

- ✅ jq dependency check
- ✅ Terminal color detection
- ✅ Symlink target validation (not just symlink exists)
- ✅ Correct JSON fields (`.total`, `.grade`)
- ✅ Clear output with color coding
- ✅ Appropriate exit codes
- ✅ Helpful remediation instructions

### Test the Script

```bash
# Run compliance check
./scripts/check-skill-audit-status.sh

# Expected output:
# - Lists all skills with compliance status
# - Color-coded: green (compliant), yellow (outdated), red (missing)
# - Summary with counts
# - Exit code 0 if all compliant, 1 if issues found
```

### Optional: Add Slash Command

Add to `opencode.json`:

```json
{
  "commands": {
    "check-skill-audits": {
      "description": "Verify audit compliance for all skills",
      "prompt": "Run the script at scripts/check-skill-audit-status.sh and report the results. Show which skills are missing audits, which are outdated, and overall compliance rate."
    }
  }
}
```

### Success Criteria

- [ ] Script tested successfully
- [ ] Correctly identifies compliant/outdated/missing audits
- [ ] Clear output with color coding
- [ ] Returns appropriate exit code
- [ ] Optional slash command added

---

## Task 4.4: Add Pre-Commit Hook (Optional)

**Goal:** Prevent commits that modify skills without corresponding audit

### Implementation

Add to `.lefthook.yml`:

```yaml
pre-commit:
  parallel: true
  commands:
    # ... existing commands ...
    
    skill-audit-check:
      glob: "skills/**/*"
      run: |
        # Check if any skills were modified
        changed_skills=$(git diff --cached --name-only | grep "^skills/" | cut -d'/' -f2 | sort -u)
        
        if [ -n "$changed_skills" ]; then
          echo "🔍 Checking audit compliance for modified skills..."
          
          for skill in $changed_skills; do
            if [ ! -d ".context/audits/$skill/latest" ]; then
              echo "❌ Missing audit for skill: $skill"
              echo "   Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill --json --store"
              exit 1
            fi
            
            # Check if audit is recent (within 7 days)
            audit_file=".context/audits/$skill/latest/audit.json"
            if [ ! -f "$audit_file" ]; then
              echo "❌ Missing audit.json for skill: $skill"
              exit 1
            fi
            
            if find "$audit_file" -mtime +7 2>/dev/null | grep -q .; then
              echo "⚠️  Audit for $skill is >7 days old. Consider re-auditing."
              echo "   Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill --json --store"
            fi
          done
          
          echo "✅ All modified skills have recent audits"
        fi
```

### Execution

1. Add skill-audit-check to .lefthook.yml
2. Test by modifying a skill without audit:
   ```bash
   # Create test file
   touch skills/test-skill/SKILL.md
   git add skills/test-skill/SKILL.md
   git commit -m "test"
   # Should fail with error about missing audit
   ```
3. Verify hook prevents commit
4. Test by modifying a skill with audit
5. Verify hook allows commit

### Success Criteria

- [ ] Hook added to .lefthook.yml
- [ ] Prevents commits for skills without audits
- [ ] Warns about outdated audits
- [ ] Allows commits for compliant skills
- [ ] Tested with both passing and failing cases

---

## Phase 4 Wrap-Up

### Final Verification

- [ ] AGENTS.md updated with audit requirements
- [ ] skill-quality-auditor/SKILL.md updated with anti-patterns
- [ ] Audit status check command created and tested
- [ ] Pre-commit hook added (if implemented)
- [ ] All documentation changes committed
- [ ] Time logged for Phase 4

### Test the Improvements

```bash
# Run compliance check
./scripts/check-skill-audit-status.sh

# Should show current compliance state (likely all missing since Phase 2 not run yet)
```

---

## Post-Remediation Checklist

**Note:** This checklist summarizes all success criteria from Phases 1-4. Use after completing all phases.

### Structural Compliance

- [ ] All 63 skills have `.context/audits/<skill-name>/<date>/` structure
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
