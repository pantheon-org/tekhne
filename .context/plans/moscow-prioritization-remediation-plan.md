---
plan_date: 2026-02-23
skill_name: moscow-prioritization
source_audit: .context/audits/moscow-prioritization-audit-2026-02-22.md
---

# Remediation Plan: moscow-prioritization

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 80/120 (66%) | 104/120 (87%) |
| **Grade** | D | B+ |
| **Priority** | Critical | - |

**Verdict**: Major rewrite recommended. Critical: SKILL.md is 753 lines with 0 references - must extract content.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| SKILL.md too long (753 lines, 0 refs) | D5 (5/15) | **Critical** | Maintainability, discoverability |
| Moderate anti-pattern quality | D3 (9/15) | Medium | Prioritization mistakes |
| Poor trigger discoverability | D7 (6/10) | High | Skill not activated |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |
| Moderate practical usability | D8 (10/15) | Medium | Commands missing |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - CRITICAL PRIORITY

**Problem**: 753-line SKILL.md with zero reference files is unmaintainable.

**Files to create**:

1. **Create `skills/moscow-prioritization/references/` directory**

2. **Extract to `references/category-definitions.md`**:
   - Full MoSCoW category definitions
   - Detailed criteria for Must/Should/Could/Won't
   - Category boundary cases
   - Industry-specific variations

3. **Extract to `references/prioritization-techniques.md`**:
   - Comparison with other frameworks (RICE, ICE, Kano)
   - When to use MoSCoW vs alternatives
   - Hybrid approaches
   - Weighted scoring methods

4. **Extract to `references/stakeholder-management.md`**:
   - Facilitating prioritization sessions
   - Handling disagreements
   - Getting buy-in
   - Communication templates

5. **Extract to `references/examples.md`**:
   - Real-world prioritization examples
   - Case studies
   - Anti-pattern examples with BAD/GOOD

6. **Restructure SKILL.md as navigation hub**:

```markdown
---
name: moscow-prioritization
description: Prioritize requirements using MoSCoW method (Must/Should/Could/Won't). Use when: "prioritize features", "rank requirements", "MoSCoW analysis", "what to build first", "feature prioritization", "requirements ranking", "MVP scope".
---

# MoSCoW Prioritization

## Purpose

Prioritize features and requirements using the MoSCoW method.

## When to Use

- Defining MVP scope
- Ranking backlog items
- Stakeholder alignment sessions
- Release planning

## Quick Reference

| Category | Definition | Action |
|----------|------------|--------|
| **Must** | Critical for success | Always include |
| **Should** | Important but not critical | Include if possible |
| **Could** | Nice to have | Include if time permits |
| **Won't** | Not in scope | Explicitly exclude |

## Category Definitions

See [Category Definitions](references/category-definitions.md) for detailed criteria.

### Quick Criteria
- **Must**: Without this, project fails
- **Should**: Significant value, workarounds exist
- **Could**: Enhances value, not required
- **Won't**: Out of scope for this release

## Anti-Patterns

[Concise list - see references/examples.md for detailed examples]

## Quick Commands

[Essential commands only]

## Workflow

[Deterministic step-by-step process]

## Verification

[Standard verification section]
```

**Target SKILL.md length**: 150-200 lines max

### Phase 2: Anti-Pattern Quality (D3) - MEDIUM PRIORITY

**File**: `skills/moscow-prioritization/SKILL.md` and `references/anti-patterns.md`

1. **Create `references/anti-patterns.md`** with detailed examples:

```markdown
# MoSCoW Anti-Patterns

## Classification Anti-Patterns

### NEVER mark everything as Must
- **WHY**: If everything is critical, nothing is
- **BAD**: 50 items marked as Must in a 60-item backlog
- **GOOD**: Limit Must to ~20-30% of items
- **CONSEQUENCE**: No prioritization achieved, stakeholder fatigue

### NEVER use MoSCoW without stakeholder input
- **WHY**: Priorities reflect business value, not technical preference
- **BAD**: Developer-only prioritization session
- **GOOD**: Include product owner, stakeholders, users
- **CONSEQUENCE**: Misaligned priorities, rejected deliverables

### NEVER skip Won't category
- **WHY**: Explicit exclusion manages expectations
- **BAD**: Only defining Must/Should/Could
- **GOOD**: Actively document what is out of scope
- **CONSEQUENCE**: Scope creep, stakeholder disappointment

### NEVER re-prioritize mid-sprint
- **WHY**: Destroys team velocity and trust
- **BAD**: Moving Should to Must during active sprint
- **GOOD**: Re-prioritize only at sprint boundaries
- **CONSEQUENCE**: Team burnout, unpredictable delivery

## Process Anti-Patterns

### NEVER prioritize without success criteria
- **WHY**: Categories need measurable definitions
- **BAD**: "This feels like a Must"
- **GOOD**: "This is a Must because without it, users cannot complete checkout"
- **CONSEQUENCE**: Subjective debates, inconsistent decisions

### NEVER use single-dimensional scoring
- **WHY**: Value has multiple dimensions
- **BAD**: Prioritizing only by revenue impact
- **GOOD**: Consider value, effort, risk, dependencies
- **CONSEQUENCE**: Biased prioritization, hidden risks

### NEVER ignore dependencies
- **WHY**: Dependent items constrain ordering
- **BAD**: Marking UI as Must while API is Could
- **GOOD**: Map dependencies before categorization
- **CONSEQUENCE**: Blocked delivery, rework

## Communication Anti-Patterns

### NEVER present priorities without rationale
- **WHY**: Stakeholders need to trust decisions
- **BAD**: "These are the priorities, deal with it"
- **GOOD**: "Item A is Must because [specific business impact]"
- **CONSEQUENCE**: Stakeholder resistance, overridden decisions

### NEVER commit to exact dates for Should/Could
- **WHY**: These are conditional by definition
- **BAD**: "Should items will be delivered by Q2"
- **GOOD**: "Should items are targets for Q2, pending Must completion"
- **CONSEQUENCE**: Broken commitments, credibility loss
```

2. **Add concise anti-pattern section to SKILL.md**:

```markdown
## Anti-Patterns

| Pattern | Why | Impact |
|---------|-----|--------|
| Everything is Must | No prioritization achieved | See [Anti-Patterns](references/anti-patterns.md) |
| Skip Won't category | Unclear scope boundaries | See [Anti-Patterns](references/anti-patterns.md) |
| Re-prioritize mid-sprint | Destroys velocity | See [Anti-Patterns](references/anti-patterns.md) |
| Ignore dependencies | Blocked delivery | See [Anti-Patterns](references/anti-patterns.md) |
| No stakeholder input | Misaligned priorities | See [Anti-Patterns](references/anti-patterns.md) |
```

### Phase 3: Pattern Recognition (D7) - HIGH PRIORITY

**File**: `skills/moscow-prioritization/SKILL.md`

1. **Expand frontmatter description**:

```markdown
---
name: moscow-prioritization
description: Prioritize requirements using MoSCoW method (Must/Should/Could/Won't). Use when: "prioritize features", "rank requirements", "MoSCoW analysis", "what to build first", "feature prioritization", "requirements ranking", "MVP scope", "backlog prioritization", "define release scope", "must have should have".
---
```

2. **Add explicit trigger phrases**:

```markdown
## Activation Triggers

This skill activates when users ask:

### Prioritization Requests
- "Prioritize these features"
- "Rank these requirements"
- "What should we build first?"
- "Help me categorize this backlog"

### MoSCoW-Specific
- "Run a MoSCoW analysis"
- "Apply MoSCoW to this list"
- "Categorize as Must/Should/Could/Won't"

### Scope Definition
- "Define MVP scope"
- "What's critical for launch?"
- "What can we cut from this release?"
- "Scope this project"

### Facilitation
- "Run a prioritization session"
- "Help stakeholders agree on priorities"
- "Resolve prioritization conflict"
```

### Phase 4: Specification Compliance (D4) - MEDIUM PRIORITY

**File**: `skills/moscow-prioritization/SKILL.md`

1. **Add output specification**:

````markdown
## Output Specification

### Prioritization Matrix Format
```markdown
## Priority Matrix

### Must Have (Critical)
| Item | Rationale | Dependencies |
|------|-----------|--------------|
| [feature] | [why critical] | [blocking items] |

### Should Have (Important)
| Item | Rationale | Dependencies |
|------|-----------|--------------|
| [feature] | [why important] | [blocking items] |

### Could Have (Nice to Have)
| Item | Rationale | Dependencies |
|------|-----------|--------------|
| [feature] | [value add] | [blocking items] |

### Won't Have (This Release)
| Item | Rationale | Future Consideration |
|------|-----------|----------------------|
| [feature] | [why excluded] | [when to revisit] |
```

### Target Distribution
- Must: 20-30% of items
- Should: 30-40% of items
- Could: 20-30% of items
- Won't: 10-20% of items
````

### Phase 5: Practical Usability (D8) - MEDIUM PRIORITY

**File**: `skills/moscow-prioritization/SKILL.md`

1. **Add Quick Commands section**:

````markdown
## Quick Commands

### Create Priority Matrix
1. List all items to prioritize
2. Apply category criteria
3. Review distribution
4. Validate dependencies
5. Document rationale

### Facilitation Checklist
- [ ] Identify stakeholders
- [ ] Prepare item list
- [ ] Define success criteria
- [ ] Schedule session
- [ ] Document decisions
- [ ] Communicate results

### Validation Commands
```bash
# Count items per category
grep -c "^### Must" priority-matrix.md
grep -c "^### Should" priority-matrix.md
grep -c "^### Could" priority-matrix.md
grep -c "^### Won't" priority-matrix.md
```
````

## File Restructuring Summary

```
skills/moscow-prioritization/
├── SKILL.md                    (150-200 lines, navigation hub)
├── AGENTS.md                   (optional deep navigation)
└── references/
    ├── category-definitions.md
    ├── prioritization-techniques.md
    ├── stakeholder-management.md
    ├── anti-patterns.md
    └── examples.md
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh moscow-prioritization --json

# Check file size reduction
wc -l skills/moscow-prioritization/SKILL.md

# Verify references exist
ls -la skills/moscow-prioritization/references/

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D5: Progressive Disclosure | Score increase | >= 13/15 |
| SKILL.md line count | Lines | <= 200 |
| Reference files | Count | >= 4 |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D7: Pattern Recognition | Score increase | >= 9/10 |
| D4: Specification Compliance | Score increase | >= 13/15 |
| D8: Practical Usability | Score increase | >= 13/15 |
| Overall Score | Total points | >= 104/120 |
| Grade | Letter grade | >= B+ |

## Effort Estimate

- **T-shirt size**: L (8-12 hours)
- **Complexity**: High (significant restructure)
- **Risk**: Medium (content migration required)

## Dependencies

- None (standalone skill)

## Notes

- Prioritize Phase 1 (progressive disclosure) - it's the root cause of other issues
- 753-line skill is the longest in the audit batch
- Consider creating AGENTS.md for facilitation vs. individual prioritization workflows
