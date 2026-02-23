---
plan_date: 2026-02-23
skill_name: ui-debug-workflow
source_audit: .context/audits/ui-debug-workflow-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: ui-debug-workflow

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 83/120 (69%) | 96/120 (80%) |
| **Grade** | D | B |
| **Priority** | Critical | - |
| **Effort** | Large (L) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Pattern recognition (D7)

**Verdict**: Major rewrite recommended. Critical gaps in multiple dimensions.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| No references directory | D5 (7/15) | Critical | Maintainability crisis |
| Weak anti-patterns | D3 (8/15) | High | Common mistakes repeated |
| Weak trigger keywords | D7 (6/10) | High | Skill may not activate |
| Moderate knowledge delta | D1 (15/20) | Medium | Generic guidance |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: Critical

**Target**: Increase from 7/15 to 13/15 (+6 points)

#### Step 1.1: Create references directory

**File**: `skills/ui-debug-workflow/references/`

```text
skills/ui-debug-workflow/
├── SKILL.md (hub, ~150 lines)
└── references/
    ├── debugging-checklist.md
    ├── browser-devtools-guide.md
    └── evidence-templates.md
```

#### Step 1.2: Extract detailed content

Move detailed content to reference files and update SKILL.md as navigation hub.

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add explicit anti-patterns

**File**: `skills/ui-debug-workflow/SKILL.md`

```markdown
## Anti-Patterns

### NEVER Debug Without Reproduction Steps

**WHY**: Without reliable reproduction, cannot verify fixes.

**BAD**: "It crashed yesterday, let me check the code."
**GOOD**: "I can reproduce by: 1) Go to /users, 2) Click sort, 3) Error appears."

### NEVER Skip Console Error Review

**WHY**: Console errors often reveal root cause directly.

**BAD**: Immediately diving into code changes.
**GOOD**: First check console for errors.
```

---

### Phase 3: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 3.1: Expand frontmatter description

**File**: `skills/ui-debug-workflow/SKILL.md`

```yaml
---
name: ui-debug-workflow
description: |
  Complete workflow for debugging UI changes with visual testing, evidence collection.
  Use when: debugging visual issues, investigating UI bugs, collecting bug evidence.
  
  Keywords: UI bug, visual issue, DevTools, CSS problem, component debugging,
  visual regression, evidence collection
---
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh ui-debug-workflow --json
bunx markdownlint-cli2 "skills/ui-debug-workflow/**/*.md"
ls skills/ui-debug-workflow/references/
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 13/15 |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| SKILL.md line count | <= 200 lines |
| References created | >= 3 files |
| Overall Score | >= 96/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | L | 3 hours |
| Phase 2: Anti-patterns | M | 1 hour |
| Phase 3: Triggers | S | 30 min |
| **Total** | **L** | **4.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/ui-debug-workflow/
```

## Notes

- Rating: **6/10** - Plan addresses critical gaps but needs more specific code examples
- Good structure with phases
- Needs more detailed code examples in remediation steps
- Good that it identifies the critical 0 references issue
