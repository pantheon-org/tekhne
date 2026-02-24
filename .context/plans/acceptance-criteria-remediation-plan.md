---
plan_date: 2026-02-23
skill_name: acceptance-criteria
source_audit: .context/audits/acceptance-criteria-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: acceptance-criteria

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 90/120 (75%) | 102/120 (85%) |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Pattern recognition (D7), Anti-pattern quality (D3), Practical usability (D8)

**Verdict**: Targeted improvements recommended. Moderate scores across dimensions - focus on specificity and usability.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak trigger keywords for discovery | D7 (6/10) | Medium | Skill may not activate when needed |
| Anti-patterns lack concrete examples | D3 (11/15) | Medium | Common mistakes may be repeated |
| Missing executable commands | D8 (10/15) | Medium | Reduced practical usability |
| SKILL.md needs content extraction | D5 (11/15) | Medium | Maintainability concerns |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 1.1: Expand frontmatter description

**File**: `skills/acceptance-criteria/SKILL.md`

```yaml
---
name: acceptance-criteria
description: |
  Write effective acceptance criteria that capture requirements from the user's perspective.
  Use when: writing user stories, defining done criteria, creating testable requirements,
  applying INVEST criteria, or specifying acceptance tests.
  
  Keywords: acceptance criteria, user story, INVEST, testable requirements,
  definition of done, story points, acceptance testing, functional requirements
---
```

#### Step 1.2: Add "Use When" section

```markdown
## Use When

- "Write acceptance criteria for this feature"
- "Define done for a user story"
- "Create testable requirements"
- "Apply INVEST criteria to stories"
- "Write acceptance tests"
- NOT for: Technical specifications, architecture decisions
```

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 11/15 to 14/15 (+3 points)

#### Step 2.1: Add explicit anti-patterns

**File**: `skills/acceptance-criteria/SKILL.md`

````markdown
## Anti-Patterns

### NEVER: Use vague, non-measurable criteria

**WHY**: Cannot verify completion without objective measures.

**BAD**:
```markdown
The system should be fast
The UI should be user-friendly
```

**GOOD**:
```markdown
Page load time < 2 seconds (Lighthouse score > 90)
New users can complete checkout in under 5 minutes
```

### NEVER: Write criteria as implementation instructions

**WHAT** to test, not HOW to implement.

**BAD**:
```markdown
Click the submit button and validate the form fields using JavaScript
```

**GOOD**:
```markdown
Form submission displays success message within 1 second
```

### NEVER: Skip negative test cases

**WHY**: Users encounter error conditions; must be handled gracefully.

**BAD**:
```markdown
User can log in with valid credentials
```

**GOOD**:
```markdown
User can log in with valid credentials
Invalid credentials display error message without exposing system details
Session timeout redirects to login within 30 seconds
```
````

#### Step 2.2: Add repository-specific risk tie-ins

Document common acceptance criteria mistakes specific to this repository's domain.

---

### Phase 3: Practical Usability (D8) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Add Quick Commands section

**File**: `skills/acceptance-criteria/SKILL.md`

Add a Quick Commands section with examples:

- Validate format: `grep -E "(should be|will be|might be)" criteria.md`
- Count criteria: `grep -c "^## " criteria.md`
- Generate template using the provided template structure

#### Step 3.2: Add expected output examples

````markdown
## Expected Outputs

### Good Acceptance Criteria Structure
```
## Scenario: User checkout

Given: User has items in cart
When: User clicks checkout
Then: Order confirmation displayed
And: Confirmation email sent

## Acceptance Criteria
- [ ] Confirmation shows order number
- [ ] Total matches cart total
- [ ] Email sent within 5 seconds
```
````

---

### Phase 4: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 11/15 to 14/15 (+3 points)

#### Step 4.1: Create references directory

Create `skills/acceptance-criteria/references/` if not exists.

#### Step 4.2: Extract detailed examples

**File**: `skills/acceptance-criteria/references/examples.md`

Move detailed scenario examples, INVEST criteria explanations, and templates.

#### Step 4.3: Update SKILL.md as hub

Replace detailed content with navigation links:

```markdown
## Quick Reference

| Topic | Reference |
| --- | --- |
| INVEST Criteria | [references/invest-criteria.md](references/invest-criteria.md) |
| Examples | [references/examples.md](references/examples.md) |
| Templates | [references/templates.md](references/templates.md) |
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json
bunx markdownlint-cli2 "skills/acceptance-criteria/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D7 Pattern Recognition | Score >= 9/10 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D8 Practical Usability | Score >= 13/15 |
| D5 Progressive Disclosure | Score >= 13/15 |
| SKILL.md line count | <= 120 lines |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Triggers | S | 20 min |
| Phase 2: Anti-patterns | M | 45 min |
| Phase 3: Commands | S | 30 min |
| Phase 4: Disclosure | S | 30 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/acceptance-criteria/SKILL.md
```

## Notes

- Rating: **6/10** - Decent plan structure but needs template standardization
- This plan follows legacy format; needs update to match current template standards
- Actions are specific and actionable
- No code examples in remediation steps - should add for clarity
