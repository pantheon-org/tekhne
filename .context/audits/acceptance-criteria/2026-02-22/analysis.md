---
review_date: 2026-02-22
reviewer: automated audit
skill_location: skills/acceptance-criteria/SKILL.md
---

# Skill Evaluation Report: acceptance-criteria

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 90/120 (75%) |
| **Grade** | C+ |
| **Pattern** | Mixed quality |
| **Knowledge Ratio** | E:A:R = 70:20:10 |
| **Verdict** | Priority improvements required |

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 17 | 20 | Depth of non-obvious guidance |
| D2: Mindset + Procedures | 12 | 15 | Procedural clarity and sequencing |
| D3: Anti-Pattern Quality | 11 | 15 | Specificity and enforceability |
| D4: Specification Compliance | 10 | 15 | Metadata/spec alignment |
| D5: Progressive Disclosure | 11 | 15 | SKILL-to-references balance |
| D6: Freedom Calibration | 13 | 15 | Constraint/flexibility balance |
| D7: Pattern Recognition | 6 | 10 | Trigger/intent discoverability |
| D8: Practical Usability | 10 | 15 | Actionable examples/commands |

## Critical Issues

### 1. Score-to-grade risk

- skills/acceptance-criteria/SKILL.md:1
- **Evidence**: Score is 90/120 (grade C+).
- **Impact**: Lower score reduces reliability and clarity for execution.

### 2. Maintainability pressure

- skills/acceptance-criteria/SKILL.md:1
- **Evidence**: File size is 169 lines; references count is 2.
- **Impact**: Structure may be harder to maintain as content grows.

## Top 3 Recommended Improvements

### Priority 1: Tighten activation cues

Make trigger phrases specific and deterministic near the top of SKILL.md.

### Priority 2: Improve anti-pattern precision

Add explicit NEVER/WHY/BAD-GOOD examples where weak.

### Priority 3: Improve progressive disclosure

Keep SKILL.md concise and move deep details to references/.

## Detailed Dimension Analysis

### D1: Knowledge Delta

**Assessment**: 17/20 (signal: strong, priority: low)

**Inspect**:

- skills/acceptance-criteria/SKILL.md for generic tutorial language vs project-specific guidance.
- Presence of advanced constraints, gotchas, and production caveats.

**Fix steps**:

1. Replace generic basics with project-contextual rules and edge cases.
2. Add at least 2 concrete anti-generic examples (bad vs good).
3. Keep only non-obvious guidance that changes execution quality.

**Done criteria**:

- Reader can identify what to do differently in this repository.
- Re-run score for D1 increases or remains strong.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D2: Mindset + Procedures

**Assessment**: 12/15 (signal: strong, priority: low)

**Inspect**:

- Whether workflow is a deterministic sequence with clear entry/exit conditions.
- Missing preconditions, assumptions, or decision points.

**Fix steps**:

1. Rewrite workflow as ordered steps with explicit outcomes.
2. Add "when to use / when not to use" for scope control.
3. Add guardrails for ambiguous inputs.

**Done criteria**:

- Another human/agent can execute the process without clarification.
- Procedure order maps to real execution order.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D3: Anti-Pattern Quality

**Assessment**: 11/15 (signal: moderate, priority: medium)

**Inspect**:

- Count and quality of explicit anti-patterns.
- Whether each anti-pattern explains WHY and consequence.

**Fix steps**:

1. Add concrete anti-pattern entries with NEVER + WHY + consequence.
2. Include one BAD/GOOD example per major failure mode.
3. Tie each anti-pattern to repository-specific risk.

**Done criteria**:

- Anti-patterns are actionable, not generic warnings.
- BAD/GOOD examples directly map to expected author behavior.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D4: Specification Compliance

**Assessment**: 10/15 (signal: moderate, priority: medium)

**Inspect**:

- Frontmatter quality and description precision.
- Alignment with repository conventions and naming rules.

**Fix steps**:

1. Tighten description to include explicit trigger intent and scope.
2. Ensure structure matches current skill/spec expectations.
3. Remove ambiguous language and undefined outputs.

**Done criteria**:

- Frontmatter fields are complete and precise.
- Description is sufficient for routing without opening references.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D5: Progressive Disclosure

**Assessment**: 11/15 (signal: moderate, priority: medium)

**Inspect**:

- SKILL.md length vs reference extraction balance.
- Whether deep details are placed in references/ and linked from hub sections.

**Fix steps**:

1. Keep SKILL.md as a navigation hub; move deep content to references/.
2. Add concise section summaries with links to details.
3. Remove duplicated content across SKILL.md and references.

**Done criteria**:

- SKILL.md is scannable and task-oriented.
- Detailed content is discoverable but not blocking quick use.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D6: Freedom Calibration

**Assessment**: 13/15 (signal: strong, priority: low)

**Inspect**:

- Balance between rigid rules and adaptable guidance.
- Places where wording over-constrains or under-specifies behavior.

**Fix steps**:

1. Keep hard constraints only for safety/consistency-critical rules.
2. Convert soft decisions to explicit options with tradeoffs.
3. Add fallback paths for missing context.

**Done criteria**:

- Instructions are strict where required and flexible where safe.
- Agent can adapt to context without violating core constraints.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D7: Pattern Recognition

**Assessment**: 6/10 (signal: moderate, priority: medium)

**Inspect**:

- Trigger keywords and problem statements in frontmatter description.
- Coverage of user phrasings that should activate this skill.

**Fix steps**:

1. Expand description with high-signal keywords and concrete examples.
2. Add concise "use when" phrases tied to real requests.
3. Remove overlapping wording that causes misrouting.

**Done criteria**:

- User intents map to this skill without ambiguity.
- Skill is discoverable from multiple equivalent phrasings.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

### D8: Practical Usability

**Assessment**: 10/15 (signal: moderate, priority: medium)

**Inspect**:

- Presence of executable commands and deterministic output expectations.
- Missing command examples for common workflows.

**Fix steps**:

1. Add copy/paste commands for core tasks.
2. Add explicit expected outputs or completion checks.
3. Ensure examples match repository toolchain and paths.

**Done criteria**:

- Human/agent can execute commands and verify outcomes directly.
- No ambiguous "do X" steps without runnable examples.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json

## Proposed Restructured SKILL.md

Use a deterministic template with sections for purpose, when to apply, workflow, anti-patterns, quick commands, and verification.

## Verification

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/acceptance-criteria-audit-2026-02-22.md
```

## Files Inventory

```text
skills/acceptance-criteria/
├── SKILL.md
├── AGENTS.md (optional)
├── references/ (optional)
├── templates/ (optional)
├── schemas/ (optional)
└── scripts/ (optional)
```

## Audit Execution

- Evaluator: skills/skill-quality-auditor/scripts/evaluate.sh
- Report validator: skills/skill-quality-auditor/scripts/validate-review-format.sh
- Date: 2026-02-22

## Score Evolution

- Baseline: Not captured in this single-run report
- Current: 90/120 (C+)
- Next: Re-run after edits and compare deltas

## Grade Scale Reference

- A+/A: 108-120
- B+/B: 96-107
- C+/C: 84-95
- D: 78-83
- F: 0-77

## Conclusion

Priority improvements required
