# Skill Quality Audit: bdd-testing

**Date**: 2026-02-21
**Auditor**: Skill Quality Auditor
**Skill Location**: `skills/bdd-testing/`

---

## Executive Summary

| Metric | Value |
|--------|-------|
| **Total Score** | 68/120 (D+) |
| **Grade** | D+ (Needs Major Work) |
| **Critical Issues** | 3 |
| **High Issues** | 2 |
| **Medium Issues** | 3 |

**Verdict**: The skill has excellent content in existing files but suffers from **critical false advertising** - AGENTS.md claims 42 reference files but only 7 exist. This makes the skill unusable as documented.

---

## Dimension Scores

| Dimension | Score | Weight | Details |
|-----------|-------|--------|---------|
| D1: Knowledge Delta | 14/20 | 20 | Good expert content in existing files, but 35+ missing files |
| D2: Mindset + Procedures | 10/15 | 15 | Has When to Apply, missing When NOT to Apply |
| D3: Anti-Pattern Quality | 5/15 | 15 | Anti-patterns in some files, dedicated file missing |
| D4: Specification Compliance | 12/15 | 15 | Good description with keywords, frontmatter present |
| D5: Progressive Disclosure | 15/15 | 15 | SKILL.md 64 lines - excellent, well-organized navigation |
| D6: Freedom Calibration | 8/15 | 15 | Process skill with appropriate balance |
| D7: Pattern Recognition | 8/10 | 10 | Good keywords in description |
| D8: Practical Usability | 6/15 | 15 | Existing examples are good, but most are missing |

---

## Critical Issues (Must Fix)

### CRIT-1: False Advertising - Missing Reference Files

**Impact**: Users cannot use the skill as documented

**Evidence**:
- AGENTS.md claims 42 reference files
- Only 7 files actually exist in `references/`
- 35+ files are documented but do not exist

**Missing Categories**:
- `scenarios/` - 0 of 7 files exist
- `patterns/` - 0 of 5 files exist (including anti-patterns)
- `collaboration/` - 0 of 8 files exist
- `cucumber/` - only 1 of 5 files exist

**Remediation**:
1. Either create all 35+ missing files, OR
2. Update AGENTS.md to reflect actual content
3. If files are planned, mark them as `[PLANNED]` not as existing

---

### CRIT-2: Missing Anti-Patterns Reference File

**Impact**: D3 score severely impacted

**Evidence**:
- SKILL.md references `patterns-common-antipatterns.md`
- File does not exist (404 error when reading)

**Remediation**:
Create `references/patterns-common-antipatterns.md` with:
- NEVER lists with WHY explanations
- Concrete code examples
- Consequences of anti-patterns

---

### CRIT-3: AGENTS.md Does Not Match Reality

**Impact**: Breaks skill-judge specification compliance

**Evidence**:
AGENTS.md lists detailed file structure that doesn't exist:
```
references/
├── principles/      # 6 files claimed, 5 exist
├── gherkin/         # 2 files claimed, 1 exists
├── collaboration/   # 8 files claimed, 0 exist
├── scenarios/       # 7 files claimed, 0 exist
├── patterns/        # 5 files claimed, 0 exist
└── cucumber/        # 5 files claimed, 1 exists
```

**Remediation**:
Update AGENTS.md to accurately reflect existing files only.

---

## High Issues

### HIGH-1: Missing When NOT to Apply Section

**Impact**: D2 score reduced

**Location**: SKILL.md

**Current**: Has "When to Apply" section
**Missing**: "When NOT to Apply" guidance

**Remediation**:
Add section:
```markdown
## When NOT to Apply BDD

- **Pure CRUD operations** - Use unit tests instead
- **One-off scripts** - BDD overhead not justified
- **Exploratory prototypes** - Requirements too fluid
- **Performance testing** - BDD not designed for this
```

---

### HIGH-2: Incomplete Cucumber Coverage

**Impact**: D8 score reduced

**Evidence**:
- Only `cucumber-setup.md` exists
- Missing: step-definitions, hooks, world-configuration, debugging

**Remediation**:
Create missing cucumber reference files or remove from navigation.

---

## Medium Issues

### MED-1: No Templates Directory

**Impact**: Users lack quick-start templates

**Remediation**:
Create `templates/` with:
- `feature-file.template.gherkin`
- `step-definition.template.ts`
- `cucumber-config.template.js`

---

### MED-2: Principles Files Could Include Anti-Patterns

**Impact**: D3 score could improve

**Observation**: Some principles files have anti-patterns sections (living-documentation.md), others don't

**Remediation**:
Add anti-patterns section to all principles files for consistency.

---

### MED-3: No Schemas or Scripts

**Impact**: Limited automation support

**Remediation**:
Consider adding:
- `schemas/gherkin-validation.schema.json`
- `scripts/run-bdd-coverage.sh`

---

## Strengths

1. **SKILL.md is excellent** - 64 lines, clean navigation, good frontmatter
2. **principles-example-mapping.md is model quality** - 416 lines with step-by-step process, visual layouts, NEVER/WHY examples
3. **principles-ubiquitous-language.md is model quality** - 451 lines with workshop exercises, code examples, pitfalls
4. **Good category organization** - principles, gherkin, collaboration, scenarios, patterns, cucumber
5. **Working code examples** - Cucumber setup and Gherkin syntax are complete and usable

---

## Recommendations

### Immediate (Before Publishing)

1. **Delete or mark all non-existent files** in AGENTS.md
2. **Create patterns-common-antipatterns.md** or remove reference
3. **Add "When NOT to Apply" section** to SKILL.md

### Short-term (Next Sprint)

1. Create `scenarios/` reference files (most user-critical)
2. Create missing `cucumber/` reference files
3. Add `templates/` directory with starter files

### Long-term

1. Complete all 42 reference files as documented
2. Add schemas for Gherkin validation
3. Add scripts for BDD workflow automation

---

## File Inventory

### Existing Files (7)

| File | Lines | Quality |
|------|-------|---------|
| cucumber-setup.md | 78 | Good |
| gherkin-syntax.md | 112 | Good |
| principles-core-philosophy.md | 69 | Good |
| principles-example-mapping.md | 416 | Excellent |
| principles-living-documentation.md | 316 | Excellent |
| principles-three-amigos.md | 80 | Good |
| principles-ubiquitous-language.md | 451 | Excellent |

### Missing Files (35+)

See AGENTS.md for full list of claimed but non-existent files.

---

## Comparison to Previous Audit

| Skill | Score | Grade | Key Issue |
|-------|-------|-------|-----------|
| agents-md | 97/120 | B+ | SKILL.md too long, missing anti-patterns format |
| bdd-testing | 68/120 | D+ | **False advertising - 35+ files missing** |

---

## Conclusion

The bdd-testing skill has **excellent content where it exists** but is **fundamentally incomplete**. The AGENTS.md claims a comprehensive reference library that doesn't exist, making the skill unusable as documented. 

**Priority**: Fix the false advertising immediately by either:
1. Creating all missing files (recommended - the existing content quality is high)
2. Updating AGENTS.md to match reality (minimum viable fix)

The skill author clearly understands BDD well - the existing 7 files demonstrate this. The gap is in completion, not quality.
