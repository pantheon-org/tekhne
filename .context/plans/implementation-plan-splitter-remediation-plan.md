---
plan_date: 2026-02-23
skill_name: implementation-plan-splitter
source_audit: .context/audits/implementation-plan-splitter-audit-2026-02-22.md
---

# Remediation Plan: implementation-plan-splitter

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 104/120 (86%) | 112/120 (93%) |
| **Grade** | B+ | A |
| **Priority** | Low | - |

**Verdict**: Targeted improvements recommended. Highest-scoring skill in audit batch.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |
| Suboptimal progressive disclosure | D5 (10/15) | Medium | Discoverability |

## Detailed Remediation Steps

### Phase 1: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/implementation-plan-splitter/SKILL.md`

**Problem**: Workflow could be more explicit with entry/exit conditions.

1. **Add explicit workflow section with numbered steps**:

```markdown
## Workflow

### Step 1: Analyze Input Plan
- **Input**: Implementation plan document (markdown)
- **Action**: Parse plan structure, identify sections, estimate sizes
- **Output**: Plan analysis with section boundaries
- **Exit condition**: Plan successfully parsed

### Step 2: Identify Split Points
- **Input**: Plan analysis from Step 1
- **Action**: 
  - Find natural boundaries (phases, milestones, components)
  - Check section sizes against threshold (default: 500 lines)
  - Identify dependencies between sections
- **Output**: List of proposed split points with rationale
- **Exit condition**: At least one valid split point identified

### Step 3: Validate Dependencies
- **Input**: Split points, dependency graph
- **Action**: Ensure split maintains logical ordering
- **Output**: Validated split plan
- **Exit condition**: No circular dependencies introduced

### Step 4: Generate Sub-Plans
- **Input**: Validated split plan
- **Action**: 
  - Create individual sub-plan files
  - Add cross-references between related plans
  - Generate master index file
- **Output**: Array of sub-plan file paths
- **Exit condition**: All sub-plans created with valid frontmatter

### Step 5: Verify Completeness
- **Input**: Sub-plan files
- **Action**: 
  - Check all original content preserved
  - Verify all references resolve
  - Confirm no orphaned sections
- **Output**: Verification report
- **Exit condition**: All checks pass
```

2. **Add guardrails for ambiguous inputs**:

```markdown
## Guardrails

### When Input is Unclear
- If plan has no clear boundaries: Ask user for split preference
- If dependencies are complex: Generate dependency visualization first
- If plan is already optimal size: Recommend against splitting

### Handling Edge Cases
| Situation | Action |
|-----------|--------|
| Plan < 300 lines | Recommend not splitting |
| No natural boundaries | Offer chronological or component-based split options |
| Circular dependencies | Alert user, suggest restructuring |
| Mixed content types | Group by content type first |
```

### Phase 2: Progressive Disclosure (D5) - MEDIUM PRIORITY

**File**: `skills/implementation-plan-splitter/SKILL.md`

**Problem**: 338-line SKILL.md could benefit from reference extraction.

1. **Create `skills/implementation-plan-splitter/references/split-strategies.md`**:

```markdown
# Split Strategies

## Strategy Selection Guide

### By Component
Use when: Plan covers multiple independent components
Example: API + Database + Frontend

### By Phase
Use when: Plan follows sequential delivery phases
Example: Phase 1 (MVP) + Phase 2 (Enhancement) + Phase 3 (Scale)

### By Team
Use when: Different teams own different parts
Example: Backend team + Frontend team + DevOps team

### By Priority
Use when: Plan has must-have vs nice-to-have items
Example: P0 (Critical) + P1 (Important) + P2 (Future)

## Strategy Comparison

| Strategy | Best For | Risk |
|----------|----------|------|
| Component | Modular architecture | Missing integration points |
| Phase | Sequential delivery | Phase dependencies |
| Team | Parallel work streams | Coordination overhead |
| Priority | Incremental delivery | Scope creep |
```

2. **Create `skills/implementation-plan-splitter/references/dependency-handling.md`**:

````markdown
# Dependency Handling

## Dependency Types

### Hard Dependencies
- Must complete before dependent task starts
- Example: Database schema before API endpoints

### Soft Dependencies
- Preferred ordering, but can be parallelized
- Example: UI design before implementation

### External Dependencies
- Outside project control
- Example: Third-party API availability

## Cross-Reference Patterns

When splitting plans, maintain cross-references:

```markdown
---
related:
  upstream: [parent-plan-id]
  downstream: [dependent-plan-ids]
---
```
````

3. **Update SKILL.md to reference extracted content**:

```markdown
## Split Strategies

See [Split Strategies Reference](references/split-strategies.md) for detailed strategy guidance.

### Quick Reference
| Strategy | When to Use |
|----------|-------------|
| By Component | Multiple independent modules |
| By Phase | Sequential delivery phases |
| By Team | Parallel team work |
| By Priority | Incremental delivery |

## Dependency Handling

See [Dependency Handling Reference](references/dependency-handling.md) for dependency patterns.

### Key Rules
- Hard dependencies must be preserved in ordering
- Cross-references required between related plans
- External dependencies noted in frontmatter
```

### Phase 3: Enhance Existing Strengths

**File**: `skills/implementation-plan-splitter/SKILL.md`

The skill already scores well on D1 (18/20), D3 (13/15), D4 (15/15), D7 (10/10), and D8 (15/15). Minor enhancements:

1. **Add "When NOT to Split" section**:

```markdown
## When NOT to Split

### Avoid Splitting When
- Plan is under 300 lines
- Content is tightly coupled
- Single owner/assignee
- Timeline is under 1 week
- Breaking changes would cause confusion

### Anti-Pattern: Over-Splitting
- **WHY**: Too many files reduce coherence
- **BAD**: 10 sub-plans averaging 50 lines each
- **GOOD**: 2-4 sub-plans of 200-400 lines each
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh implementation-plan-splitter --json

# Verify references exist
ls -la skills/implementation-plan-splitter/references/

# Check SKILL.md line count
wc -l skills/implementation-plan-splitter/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| D5: Progressive Disclosure | Score increase | >= 13/15 |
| SKILL.md line count | Lines | <= 250 |
| Reference files | Count | >= 2 |
| Overall Score | Total points | >= 112/120 |
| Grade | Letter grade | >= A |

## Effort Estimate

- **T-shirt size**: S (2-3 hours)
- **Complexity**: Low
- **Risk**: Low (additive changes only)

## Dependencies

- None (standalone skill)

## Notes

- This skill is already the highest-scoring in the audit batch
- Focus on maintaining quality while improving structure
- Reference extraction is preventive for future maintainability
