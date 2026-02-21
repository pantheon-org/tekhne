---
review_date: 2026-02-21
reviewer: automated audit
skill_location: `skills/acceptance-criteria/SKILL.md`
---

# Skill Evaluation Report: acceptance-criteria

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 90/120 (75.0%) |
| **Grade** | C+ |
| **Pattern** | Navigation Hub |
| **Lines** | 169 |
| **References** | 2 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 17 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present but not fully deterministic |
| D3: Anti-Pattern Quality | 11 | 15 | Anti-patterns are present but coverage is partial |
| D4: Specification Compliance | 10 | 15 | Core spec elements present with notable gaps |
| D5: Progressive Disclosure | 11 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 13 | 15 | Good balance of constraints and implementation freedom |
| D7: Pattern Recognition | 6 | 10 | Pattern intent exists but trigger language is shallow |
| D8: Practical Usability | 10 | 15 | Usable in practice but examples or checks need expansion |

---

## Critical Issues

### 1. Hybrid output ambiguity

The decision tree allows mixed usage of Given/When/Then and rule-oriented criteria, but the output section defines only one deterministic structure.

- `skills/acceptance-criteria/SKILL.md:51`
- `skills/acceptance-criteria/SKILL.md:144`
- **Impact**: Inconsistent output formatting across agents when handling mixed stories.

### 2. No fallback when required inputs are incomplete

Inputs are listed as required (including full user story format), but no fallback is defined for partial briefs or bug reports.

- `skills/acceptance-criteria/SKILL.md:34`
- `skills/acceptance-criteria/SKILL.md:36`
- **Impact**: Workflow can stall when upstream artifacts are incomplete.

### 3. One testability example is interpretation-sensitive

The phrase "<= 3 steps" can be interpreted differently unless step boundaries are explicitly defined.

- `skills/acceptance-criteria/SKILL.md:96`
- **Impact**: QA and product can disagree on pass/fail for the same criterion.

---

## Top 3 Recommended Improvements

### Priority 1: Add explicit hybrid output template

Add a section under Output Format for mixed stories that combines scenario flow and independent rules.

```markdown
Acceptance Criteria (Hybrid):
1. Scenario-Based Criteria (Given/When/Then)
2. Rule-Based Criteria (independent constraints)
3. Negative/Edge Scenarios
4. Out of Scope (Won't Have)
```

### Priority 2: Define fallback behavior for missing inputs

Add guidance for when only partial context is available.

```markdown
If a full user story is unavailable:
- Draft: As a [role], I want [action], so that [benefit]
- List assumptions explicitly
- Flag AC set as "Draft pending stakeholder confirmation"
```

### Priority 3: Tighten ambiguous testability example

Replace "<= 3 steps" with an explicit step boundary definition.

```markdown
"Checkout is completed in <= 3 user-visible screen transitions from cart to confirmation."
```

---

## Detailed Dimension Analysis

### D1: Knowledge Delta (17/20)

**Assessment**: Mostly expert content with some redundancy

**Strengths**:

- Skill has domain-specific direction beyond generic writing guidance.

**Weaknesses**:

- Scores below 18 usually indicate overlap that can be collapsed into references.

### D2: Mindset + Procedures (12/15)

**Assessment**: Procedural guidance present but not fully deterministic

**Strengths**:

- Workflow sequence is documented and executable.

**Weaknesses**:

- Lower scores indicate missing fallback behavior or weak decision points.

### D3: Anti-Pattern Quality (11/15)

**Assessment**: Anti-patterns are present but coverage is partial

**Strengths**:

- Anti-pattern section gives guardrails for common failure modes.

**Weaknesses**:

- Lower scores indicate missing "why this fails" and corrected examples.

### D4: Specification Compliance (10/15)

**Assessment**: Core spec elements present with notable gaps

| Requirement | Status | Notes |
| --- | --- | --- |
| Valid frontmatter | PASS | Metadata keys detected |
| Trigger-oriented description | PASS | Description is present in skill frontmatter |
| Structural consistency | PASS | Headings and sections are parseable |

### D5: Progressive Disclosure (11/15)

**Assessment**: Has references, hub could be smaller

**Strengths**:

- Reference-based structure exists when references/ is present.

**Weaknesses**:

- Lower scores indicate hub is too large or references are underused.

### D6: Freedom Calibration (13/15)

**Assessment**: Good balance of constraints and implementation freedom

**Strengths**:

- Constraints are present to shape output quality.

**Weaknesses**:

- Lower scores suggest over-permissive guidance or over-constraining rules.

### D7: Pattern Recognition (6/10)

**Assessment**: Pattern intent exists but trigger language is shallow

**Strengths**:

- Trigger keywords exist and pattern intent is identifiable.

**Weaknesses**:

- Lower scores imply ambiguous activation cues and mixed-output risk.

### D8: Practical Usability (10/15)

**Assessment**: Usable in practice but examples or checks need expansion

**Strengths**:

- Includes actionable examples or verification commands.

**Weaknesses**:

- Lower scores indicate insufficient end-to-end examples and testable checks.

---

## Proposed Restructured SKILL.md

````markdown
## Output Format

Produce acceptance criteria in this structure:

```markdown
User Story:
As a [role], I want [action], so that [benefit].

Acceptance Criteria (Must Have):
1. ...
2. ...

Negative/Edge Scenarios:
1. ...
2. ...

Out of Scope (Won't Have):
- ...
```

For mixed behavior and constraints, use Hybrid:

```markdown
Acceptance Criteria (Hybrid):
1. Scenario-Based Criteria (Given/When/Then)
2. Rule-Based Criteria (independent measurable constraints)

Negative/Edge Scenarios:
1. ...
2. ...

Out of Scope (Won't Have):
- ...
```

If inputs are incomplete, produce a Draft section with explicit assumptions.
````

---

## Action Items

| Priority | Action | Effort |
| --- | --- | --- |
| 1 | Address all critical issues listed above | 15-30 min |
| 2 | Apply top 3 recommended improvements in order | 20-45 min |
| 3 | Re-run evaluation and markdown validation | 5-10 min |

---

## Files Inventory

```text
skills/acceptance-criteria/
├── SKILL.md
└── references/gherkin-examples.md
└── references/patterns-by-type.md
```

---

## Verification

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json
bunx markdownlint-cli2 "skills/acceptance-criteria/SKILL.md"
```

---

## Conclusion

`acceptance-criteria` scores **90/120 (C+)**.
