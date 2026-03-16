# Remediation Plan: documentation/obsidian/defuddle

---
plan_date: "2026-03-16"
skill_name: "documentation/obsidian/defuddle"
source_audit: ".context/audits/documentation/obsidian/defuddle/2026-03-16/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 119/140 (85%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D3: Anti-Pattern Quality (11/15)
- D9: Eval Validation (10/20)

**Verdict**: Targeted improvements needed to reach grade A (+7 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Insufficient anti-patterns | D3 (11/15) | High | Agents repeat common mistakes |
| Missing or incomplete evals | D9 (10/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from 11/15 to 13/15 (+2 points)

#### Step 1.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: `skills/documentation/obsidian/defuddle/SKILL.md`

**Action**: Add section with BAD vs GOOD examples.


### Phase 2: Eval Validation - Priority: High

**Target**: Increase D9 from 10/20 to 17/20 (+7 points)

#### Step 2.1: Create Eval Scenarios

Use the `creating-eval-scenarios` skill to generate evaluation scenarios.

**Action**: Run the eval scenario creation workflow to produce:
- `evals/instructions.json` - Extract all instructions from SKILL.md
- `evals/summary.json` - Coverage statistics (target >= 80%)
- `evals/scenario-N/` - 5 scenarios with task.md, criteria.json, capability.txt

#### Step 2.2: Run Evals

```bash
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
```

#### Step 2.3: Validate Coverage

Verify `summary.json` shows `coverage_percentage >= 80` and all criteria.json files sum to 100.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh documentation/obsidian/defuddle --json --store

# Check target score achieved
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh documentation/obsidian/defuddle --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
| Grade | >= A |
| D3: Anti-Pattern Quality | >= 13/15 |
| D9: Eval Validation | >= 17/20 |

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | S | 1-2 hours |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/documentation/obsidian/defuddle/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
