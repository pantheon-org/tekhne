# Remediation Plan: documentation/obsidian/obsidian-markdown

---
plan_date: "2026-03-16"
skill_name: "documentation/obsidian/obsidian-markdown"
source_audit: ".context/audits/documentation/obsidian/obsidian-markdown/2026-03-16/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 91/140 (65%) | 112/140 (80%) |
| **Grade** | D | B |
| **Priority** | Critical | |
| **Effort** | L | |

**Focus Areas**:
- D1: Knowledge Delta (15/20)
- D2: Mindset + Procedures (10/15)
- D3: Anti-Pattern Quality (8/15)
- D5: Progressive Disclosure (11/15)
- D6: Freedom Calibration (10/15)
- D9: Eval Validation (0/20)

**Verdict**: Targeted improvements needed to reach grade B (+21 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (15/20) | High | Skill may duplicate basic docs |
| Missing mindset/procedures | D2 (10/15) | High | Agents lack decision frameworks |
| Insufficient anti-patterns | D3 (8/15) | High | Agents repeat common mistakes |
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |
| Imbalanced constraint language | D6 (10/15) | Medium | Over/under-prescriptive |
| Missing or incomplete evals | D9 (0/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: `skills/documentation/obsidian/obsidian-markdown/SKILL.md`

**Action**: Add section with BAD vs GOOD examples.


### Phase 2: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 11/15 to 13/15 (+2 points)

#### Step 2.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.


### Phase 3: Eval Validation - Priority: High

**Target**: Increase D9 from 0/20 to 17/20 (+17 points)

#### Step 3.1: Create Eval Scenarios

Use the `creating-eval-scenarios` skill to generate evaluation scenarios.

**Action**: Run the eval scenario creation workflow to produce:
- `evals/instructions.json` - Extract all instructions from SKILL.md
- `evals/summary.json` - Coverage statistics (target >= 80%)
- `evals/scenario-N/` - 5 scenarios with task.md, criteria.json, capability.txt

#### Step 3.2: Run Evals

```bash
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
```

#### Step 3.3: Validate Coverage

Verify `summary.json` shows `coverage_percentage >= 80` and all criteria.json files sum to 100.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh documentation/obsidian/obsidian-markdown --json --store

# Check target score achieved
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh documentation/obsidian/obsidian-markdown --json | jq ".total >= 112"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 112/140 |
| Grade | >= B |
| D3: Anti-Pattern Quality | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |
| D9: Eval Validation | >= 17/20 |

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | L | 4-8 hours |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/documentation/obsidian/obsidian-markdown/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
