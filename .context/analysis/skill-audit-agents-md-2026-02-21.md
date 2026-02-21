# Skill Quality Audit: agents-md

**Date**: 2026-02-21
**Auditor**: Automated (skill-quality-auditor)
**Skill Path**: `skills/agents-md/`

## Executive Summary

| Metric | Value | Target |
|--------|-------|--------|
| **Total Score** | 97/120 | ≥108 (A-grade) |
| **Grade** | B+ | A |
| **Lines (SKILL.md)** | 234 | <100 (navigation hub) |
| **Reference Files** | 7 | N/A |
| **Scripts** | 0 | N/A |
| **Templates** | 0 | N/A |

**Recommendation**: Refactor SKILL.md to navigation hub pattern (<100 lines), add anti-patterns section.

---

## Dimension Scores

| Dimension | Score | Max | Status |
|-----------|-------|-----|--------|
| D1: Knowledge Delta | 17 | 20 | Good |
| D2: Mindset + Procedures | 13 | 15 | Good |
| D3: Anti-Pattern Quality | 8 | 15 | **Needs Work** |
| D4: Specification Compliance | 14 | 15 | Excellent |
| D5: Progressive Disclosure | 10 | 15 | **Needs Work** |
| D6: Freedom Calibration | 13 | 15 | Good |
| D7: Pattern Recognition | 9 | 10 | Excellent |
| D8: Practical Usability | 13 | 15 | Good |

---

## Detailed Analysis

### D1: Knowledge Delta (17/20)

**Strengths**:
- Expert-only concepts: "nearest-wins hierarchy", "JIT indexing" - not common knowledge
- Production-ready decision tree for simple vs hierarchical approaches
- Context-sensitive approach philosophy
- Real monorepo patterns and structure guidance

**Weaknesses**:
- Some template content in SKILL.md is reference material, not expertise
- AGENTS.md structure templates could be extracted to references/

**Recommendation**: Extract template structures to `references/templates.md`.

---

### D2: Mindset + Procedures (13/15)

**Strengths**:
- **Mindset** (5/5): Core Principles section clearly defines philosophy
- **Procedures** (4/5): Three-phase workflow (Analyze → Create → Validate)
- **When/Not-When** (4/5): Decision tree provides guidance

**Weaknesses**:
- Could strengthen "when NOT to use" guidance
- Missing edge case handling (e.g., conflicting AGENTS.md files)

---

### D3: Anti-Pattern Quality (8/15)

**Critical Gap**: No dedicated anti-patterns section.

**Current State**:
- Quality Validation is a positive checklist
- No NEVER/WHY/consequences structure
- Missing common mistakes section

**Recommendation**: Add `references/anti-patterns.md` with:
- NEVER assume tech stack without detection
- NEVER copy full configuration content into AGENTS.md
- NEVER duplicate content between root and sub-AGENTS.md
- NEVER create AGENTS.md without running discovery commands

**Example Anti-Pattern**:

```markdown
### NEVER assume technology stack

**WHY**: Assumptions lead to incorrect documentation that misleads AI agents.

**Example**:
# BAD - Assumes React without verification
"Use React Testing Library for component tests..."

# GOOD - Detected and confirmed
# After running discovery: detected Vue 3 with Vitest
"Use @vue/test-utils with Vitest for component tests..."

**Consequence**: Agents follow incorrect patterns, users lose trust in documentation.
```

---

### D4: Specification Compliance (14/15)

**Strengths**:
- Description field has clear triggers: "create AGENTS.md", "update AGENTS.md", "generate agents"
- Usage guidance present: "Use when user asks to..."
- Frontmatter valid with name and description

**Minor Issue**:
- Could add more trigger variations for broader recognition

---

### D5: Progressive Disclosure (10/15)

**Critical Issue**: SKILL.md at 234 lines exceeds 100-line target.

**Current Structure**:
```
skills/agents-md/
├── SKILL.md (234 lines) ← TOO LONG
└── references/
    ├── anti-patterns.md
    ├── api-template.md
    ├── database-template.md
    ├── design-system-template.md
    ├── discovery-commands.md
    ├── testing-template.md
    └── troubleshooting.md
```

**Recommendation**: Refactor to navigation hub:

```markdown
# AGENTS.md Management (Target: <100 lines)

## When to Use
[Trigger phrases - 5 lines]

## Quick Decision
- Simple project → Single AGENTS.md
- Monorepo → Hierarchical structure
[5 lines]

## Workflow
1. **Analyze** → Load `references/discovery-commands.md`
2. **Choose** → See decision criteria below
3. **Generate** → Load domain template from references/
4. **Validate** → Run quality checklist

## References
- `discovery-commands.md` - Repository analysis
- `anti-patterns.md` - What to avoid
- `troubleshooting.md` - Common issues
- `*-template.md` - Domain-specific templates
```

---

### D6: Freedom Calibration (13/15)

**Assessment**: Well-calibrated for a process/generation skill.

- Decision tree provides structure without over-constraining
- Templates offer flexibility within patterns
- Phase-based approach allows agent judgment

---

### D7: Pattern Recognition (9/10)

**Strengths**:
- Clear trigger phrases in description
- Keywords cover main use cases

**Minor Enhancement**:
- Add trigger: "how to document codebase for AI agents"

---

### D8: Practical Usability (13/15)

**Strengths**:
- Concrete file path examples
- Copy-paste ready commands
- Quality checklist for validation

**Minor Issue**:
- Could add more executable examples in SKILL.md

---

## Artifact Convention Check

| Requirement | Status |
|-------------|--------|
| `templates/*.yaml` or `*.yml` | N/A (no templates directory) |
| `schemas/*.schema.json` | N/A (no schemas directory) |
| `scripts/*.sh` with shebang | N/A (no scripts directory) |

**Note**: Skill uses `references/` for all artifacts, which is acceptable.

---

## Action Items

### Priority: HIGH

1. **Refactor SKILL.md to <100 lines**
   - Extract template structures to `references/templates-overview.md`
   - Keep only navigation and decision guidance in SKILL.md
   - Target: 80-100 lines

2. **Add Anti-Patterns Section**
   - Create 5-7 NEVER rules with WHY/examples/consequences
   - Cover: tech assumptions, duplication, missing discovery, over-documentation
   - Score impact: +5-7 points

### Priority: MEDIUM

3. **Strengthen When/Not-When Guidance**
   - Add explicit "when NOT to create AGENTS.md"
   - Document edge cases (conflicting files, legacy codebases)

### Priority: LOW

4. **Enhance Trigger Recognition**
   - Add: "document codebase for AI", "AI-friendly documentation"
   - Consider: "LLM context setup"

---

## Projected Score After Fixes

| Dimension | Current | After Fixes |
|-----------|---------|-------------|
| D1: Knowledge Delta | 17 | 18 |
| D2: Mindset + Procedures | 13 | 14 |
| D3: Anti-Pattern Quality | 8 | 14 |
| D4: Specification Compliance | 14 | 15 |
| D5: Progressive Disclosure | 10 | 14 |
| D6: Freedom Calibration | 13 | 13 |
| D7: Pattern Recognition | 9 | 10 |
| D8: Practical Usability | 13 | 14 |
| **Total** | **97** | **112** |
| **Grade** | **B+** | **A** |

---

## Conclusion

The `agents-md` skill demonstrates strong domain expertise and practical utility. The primary issues are:

1. **SKILL.md length** (234 lines) violates navigation hub pattern
2. **Missing anti-patterns** reduces educational value

With targeted refactoring, this skill can achieve A-grade (≥108 points). The skill is well-structured overall and provides genuine value for AGENTS.md creation workflows.
