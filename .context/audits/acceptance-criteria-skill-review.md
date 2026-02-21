# Skill Evaluation Report: acceptance-criteria

**Review Date**: 2026-02-20  
**Skill Location**: `skills/acceptance-criteria/SKILL.md`  
**Reviewer**: codex agent

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 106/120 (88%) |
| **Grade** | B+ |
| **Pattern Detected** | Tool (appropriate size and focus) |
| **Knowledge Ratio** | E:A:R ~= 15:70:15 |
| **Verdict** | Strong, usable skill with minor determinism gaps |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | --- | --- | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly focused on actionable structure and quality checks |
| D2: Mindset + Procedures | 13 | 15 | Clear workflow and validation cadence |
| D3: Anti-Pattern Quality | 14 | 15 | Strong explicit NEVER list and practical examples |
| D4: Specification Compliance | 14 | 15 | Valid frontmatter, clear triggers, and metadata |
| D5: Progressive Disclosure | 13 | 15 | Good hub-level scope with references loaded on demand |
| D6: Freedom Calibration | 13 | 15 | Good balance between structure and author judgment |
| D7: Pattern Recognition | 9 | 10 | Correct tool pattern; one hybrid-output ambiguity remains |
| D8: Practical Usability | 14 | 15 | Immediately usable templates and checklist coverage |

---

## Critical Issues

### 1. Hybrid output ambiguity

The decision tree allows mixed usage of Given/When/Then and rule-oriented criteria, but the output section defines only one deterministic structure.

- `skills/acceptance-criteria/SKILL.md:52`
- `skills/acceptance-criteria/SKILL.md:148`

**Impact**: Inconsistent output formatting across agents when handling mixed stories.

### 2. No fallback when required inputs are incomplete

Inputs are listed as required (including full user story format), but no fallback is defined for feature briefs or bug reports.

- `skills/acceptance-criteria/SKILL.md:34`
- `skills/acceptance-criteria/SKILL.md:36`

**Impact**: Workflow can stall when upstream artifacts are partial.

### 3. One "testable" example is still interpretation-sensitive

"Checkout can be completed in <= 3 steps" may vary unless step definition is explicit.

- `skills/acceptance-criteria/SKILL.md:96`

**Impact**: Pass/fail can differ between QA and product.

---

## Top 3 Recommended Improvements

### Priority 1: Add explicit hybrid output template

Add a section under Output Format for stories requiring both scenario flow and independent rules.

```markdown
Acceptance Criteria (Hybrid):
1. Scenario-Based Criteria (Given/When/Then)
2. Rule-Based Criteria (independent constraints)
3. Negative/Edge Scenarios
4. Out of Scope (Won't Have)
```

### Priority 2: Define fallback behavior for missing inputs

Add guidance to derive a provisional user story from available context and mark assumptions for confirmation.

```markdown
If a full user story is unavailable:
- Draft: As a [role], I want [action], so that [benefit]
- List assumptions explicitly
- Flag AC set as "Draft pending stakeholder confirmation"
```

### Priority 3: Tighten ambiguous testability example

Refine "<= 3 steps" with a measurable step definition.

```markdown
"Checkout is completed in <= 3 user-visible screen transitions from cart to confirmation."
```

---

## Detailed Dimension Analysis

### D1: Knowledge Delta (16/20)

**Strengths**:

- Decision tree is practical and directly reusable.
- Quality framework and anti-patterns are operational, not conceptual.

**Weaknesses**:

- Minor overlap in sections could still be streamlined.

### D2: Mindset + Procedures (13/15)

**Strengths**:

- Workflow enforces outcome-first drafting and explicit validation.
- Scenario coverage checklist supports QA-minded thinking.

**Weaknesses**:

- No explicit fallback for low-quality inputs.

### D3: Anti-Pattern Quality (14/15)

**Strengths**:

- Clear NEVER list with practical bans.
- Mistake table maps bad wording to better criteria.

**Weaknesses**:

- Could add one sentence of rationale per anti-pattern for training value.

### D4: Specification Compliance (14/15)

| Requirement | Status | Notes |
| --- | --- | --- |
| Valid YAML frontmatter | PASS | Description block and metadata parse cleanly |
| Description has triggers | PASS | "Use when" scenarios included |
| Description has keywords | PASS | Search terms present |
| Title is clear | PASS | Precise and scoped |
| Version/date present | PASS | Present and current |

### D5: Progressive Disclosure (13/15)

**Current State**: Main file is compact and links to references for deeper examples.

**Residual Gap**: Hybrid-format examples should be added in references or template section.

### D6: Freedom Calibration (13/15)

**Assessment**: Good for semi-structured requirement writing with enough constraints to ensure quality.

### D7: Pattern Recognition (9/10)

**Intended Pattern**: Tool

**Assessment**: Correctly implemented as a concise operational guide; single ambiguity in mixed-output handling.

### D8: Practical Usability (14/15)

**Strengths**:

- Direct templates, decision tree, and checklist are immediately usable.
- MoSCoW mapping helps scope alignment.

**Weaknesses**:

- Hybrid story handling is not fully deterministic.

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
| 1 | Add Hybrid output template in `Output Format` | 10 min |
| 2 | Add fallback behavior for incomplete inputs in `Inputs Required` | 10 min |
| 3 | Clarify "<= 3 steps" example with step definition | 5 min |
| 4 | Add one hybrid example in `references/gherkin-examples.md` | 15 min |

---

## Conclusion

The current skill is high quality and largely production-ready. Remaining issues are narrow and fixable with small edits that improve determinism and reduce interpretation variance.

**Recommended Path Forward**:

1. Immediate: Add hybrid output template and missing-input fallback.
2. Short-term: Tighten one ambiguous example.
3. Optional: Add a hybrid reference example for consistency.

**Expected Outcome**: Improvement from 106/120 (B+) to ~112/120 (A-) with minimal edits.
