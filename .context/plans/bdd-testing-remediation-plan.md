---
plan_date: 2026-02-23
skill_name: bdd-testing
source_audit: .context/audits/bdd-testing-audit-2026-02-22.md
---

# Remediation Plan: bdd-testing

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 84/120 (70%) | 102/120 (85%) |
| **Grade** | C | B+ |
| **Priority** | High | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Anti-pattern quality (D3), Practical usability (D8), Pattern recognition (D7)

**Verdict**: Priority improvements required. Multiple dimensions need enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Anti-pattern quality weak | D3 (8/15) | High | Common mistakes may be repeated |
| Practical usability gaps | D8 (8/15) | High | Missing executable commands |
| Pattern recognition lacking | D7 (6/10) | Medium | Skill may not activate |
| Freedom calibration | D6 (10/15) | Medium | Constraint balance issues |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add concrete NEVER statements

**File**: `skills/bdd-testing/SKILL.md`

````markdown
## Anti-Patterns

### NEVER Write Implementation Details in Gherkin

**WHY**: Gherkin should describe behavior, not implementation.

**BAD**:
```
When I click the submit button
Then the form validates using JavaScript validation
```

**GOOD**:
```
When I submit the form
Then I should see a success message
```

### NEVER Skip the Three Amigos Session

**WHY**: Without business, development, and test perspectives, scenarios are incomplete.

### NEVER Use Technical Jargon in Feature Files

**WHY**: Feature files should be readable by all stakeholders.
````

---

### Phase 2: Practical Usability (D8) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add copy/paste commands

**File**: `skills/bdd-testing/SKILL.md`

````markdown
## Quick Commands

### Run Cucumber Tests
```bash
npx cucumber-js features/
```

### Generate Step Definitions
```bash
npx cucumber-js --dry-run features/
```
````

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 3.1: Expand frontmatter

**File**: `skills/bdd-testing/SKILL.md`

```yaml
---
name: bdd-testing
description: |
  Master Behavior-Driven Development (BDD) testing with Gherkin and Cucumber.
  Use when: writing BDD tests, creating feature files, implementing step definitions.
  
  Keywords: BDD, Gherkin, Cucumber, Given When Then, feature files,
  step definitions, acceptance criteria, Three Amigos
---
```

---

### Phase 4: Freedom Calibration (D6) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 4.1: Clarify hard vs soft constraints

**File**: `skills/bdd-testing/SKILL.md`

Add constraint classification.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh bdd-testing --json
bunx markdownlint-cli2 "skills/bdd-testing/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | M | 1 hour |
| Phase 2: Commands | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| Phase 4: Constraints | S | 20 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/bdd-testing/SKILL.md
```

## Notes

- Rating: **7/10** - Good plan with clear phases and specific anti-patterns
- Already includes Timeline table
- Anti-patterns have good WHY/BAD/GOOD format
- Minor: Could add more verification steps
