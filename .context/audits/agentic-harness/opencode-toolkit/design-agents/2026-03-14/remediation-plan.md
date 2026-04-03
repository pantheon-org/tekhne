# Remediation Plan: agentic-harness/opencode-toolkit/design-agents

---
plan_date: "2026-03-14"
skill_name: "agentic-harness/opencode-toolkit/design-agents"
source_audit: ".context/audits/agentic-harness/opencode-toolkit/design-agents/2026-03-14/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 114/140 (81%) | 126/140 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | |
| **Effort** | M | |

**Focus Areas**:
- D2: Mindset + Procedures (11/15)
- D8: Practical Usability (10/15)
- D9: Eval Validation (14/20)

**Verdict**: Targeted improvements needed to reach grade A (+12 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Missing mindset/procedures | D2 (11/15) | High | Agents lack decision frameworks |
| Limited practical examples | D8 (10/15) | High | Agents struggle to apply skill |
| Missing or incomplete evals | D9 (14/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Practical Usability - Priority: High

**Target**: Increase D8 from 10/15 to 13/15 (+3 points)

#### Step 1.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/agentic-harness/opencode-toolkit/design-agents/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.


### Phase 2: Eval Validation - Priority: High

**Target**: Increase D9 from 14/20 to 17/20 (+3 points)

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
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh agentic-harness/opencode-toolkit/design-agents --json --store

# Check target score achieved
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh agentic-harness/opencode-toolkit/design-agents --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
| Grade | >= A |
| D8: Practical Usability | >= 13/15 |
| D9: Eval Validation | >= 17/20 |

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | M | 2-4 hours |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/agentic-harness/opencode-toolkit/design-agents/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
