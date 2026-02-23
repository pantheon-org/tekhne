---
plan_date: 2026-02-23
skill_name: ui-debug-workflow
source_audit: .context/audits/ui-debug-workflow-audit-2026-02-22.md
---

# Remediation Plan: ui-debug-workflow

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 83/120 (69%) |
| **Current Grade** | D |
| **Target Score** | 96+/120 (B or higher) |
| **Priority** | Critical - Major rewrite recommended |
| **Estimated Effort** | High (5-6 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Weak progressive disclosure | Critical | D5 | 7/15 | 13/15 |
| Weak anti-pattern quality | High | D3 | 8/15 | 13/15 |
| Weak pattern recognition | High | D7 | 6/10 | 9/10 |
| Moderate knowledge delta | Medium | D1 | 15/20 | 17/20 |
| Moderate specification compliance | Medium | D4 | 10/15 | 13/15 |
| Moderate freedom calibration | Medium | D6 | 10/15 | 13/15 |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Critical Priority

**Goal**: Increase D5 score from 7/15 to 13/15

**Current State**:
- SKILL.md: 307 lines
- references/: 0 files (critical gap)

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 1.1**: Create references directory structure

```bash
mkdir -p skills/ui-debug-workflow/references
```

**Step 1.2**: Create `skills/ui-debug-workflow/references/debugging-checklist.md`

```markdown
# UI Debugging Checklist

## Initial Triage

- [ ] Identify the exact reproduction steps
- [ ] Confirm the issue in multiple browsers
- [ ] Check browser console for errors
- [ ] Verify network requests in DevTools
- [ ] Note the expected vs. actual behavior

## Visual Debugging

- [ ] Inspect element styles
- [ ] Check computed styles
- [ ] Look for CSS specificity conflicts
- [ ] Verify responsive breakpoints
- [ ] Check for overflow issues

## JavaScript Debugging

- [ ] Add console.log statements
- [ ] Set breakpoints in DevTools
- [ ] Check React DevTools (if applicable)
- [ ] Verify state/props values
- [ ] Check event handlers

## Evidence Collection

- [ ] Screenshot the issue
- [ ] Record screen if animation involved
- [ ] Copy console errors
- [ ] Document reproduction steps
- [ ] Note environment details
```

**Step 1.3**: Create `skills/ui-debug-workflow/references/browser-devtools-guide.md`

```markdown
# Browser DevTools Guide

## Chrome DevTools

### Elements Panel
- Inspect and modify DOM
- View computed styles
- Check CSS box model
- Force element states (:hover, :focus)

### Console Panel
- View JavaScript errors
- Execute commands
- Filter by severity
- Preserve log across reloads

### Network Panel
- Monitor HTTP requests
- Check response payloads
- Identify slow requests
- Block specific requests

### Performance Panel
- Record runtime performance
- Identify layout thrashing
- Find long tasks
- Analyze paint timing

### React DevTools (Extension)
- Component tree inspection
- Props and state viewer
- Profiler for render performance
- Highlight updates option
```

**Step 1.4**: Create `skills/ui-debug-workflow/references/evidence-templates.md`

````markdown
# Evidence Collection Templates

## Bug Report Template

```markdown
## Issue Description
[One sentence describing the problem]

## Reproduction Steps
1. Navigate to [URL]
2. Click on [element]
3. Observe [behavior]

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happens]

## Environment
- Browser: [Chrome 120 / Firefox 121 / Safari 17]
- OS: [macOS / Windows / Linux]
- Screen: [1920x1080 / mobile]

## Evidence
- Screenshot: [attached]
- Console errors: [paste]
- Network requests: [screenshot or HAR file]
```

## Fix Verification Template

```markdown
## Fix Summary
[One sentence describing the fix]

## Root Cause
[Why the bug occurred]

## Solution
[How it was fixed]

## Verification
- [ ] Issue no longer reproducible
- [ ] No regressions in related features
- [ ] Tests added/updated
- [ ] Documentation updated (if needed)

## Screenshots
### Before
[attachment]

### After
[attachment]
```
````

**Step 1.5**: Update SKILL.md as navigation hub

Replace verbose content with:

```markdown
## Quick Reference

- [Debugging Checklist](references/debugging-checklist.md) - Step-by-step debugging workflow
- [Browser DevTools Guide](references/browser-devtools-guide.md) - DevTools usage reference
- [Evidence Templates](references/evidence-templates.md) - Bug report and verification templates
```

---

### Phase 2: Anti-Pattern Quality (D3) - High Priority

**Goal**: Increase D3 score from 8/15 to 13/15

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 2.1**: Add explicit anti-patterns section

```markdown
## Anti-Patterns

### NEVER Debug Without Reproduction Steps

- **WHY**: Without reliable reproduction, you cannot verify fixes.
- **Consequence**: Wasted time, incomplete fixes, recurring bugs.
- **BAD**: "It crashed yesterday, let me check the code."
- **GOOD**: "I can reproduce it by: 1) Go to /users, 2) Click sort, 3) Error appears."

### NEVER Skip Console Error Review

- **WHY**: Console errors often reveal the root cause directly.
- **Consequence**: Missing obvious clues, debugging in wrong direction.
- **BAD**: Immediately diving into code changes.
- **GOOD**: First check console for errors, then investigate code.

### NEVER Assume Browser Consistency

- **WHY**: Different browsers have different rendering engines and DevTools.
- **Consequence**: Fixes that work in one browser fail in others.
- **BAD**: Testing only in Chrome.
- **GOOD**: Reproduce in at least 2 browsers before concluding.

### NEVER Make Changes Without Evidence

- **WHY**: Guessing leads to incorrect fixes and new bugs.
- **Consequence**: Wasted time, potential regressions.
- **BAD**: "This looks wrong, let me change it."
- **GOOD**: "The console shows TypeError on line 42. That's the issue."

### NEVER Close Bugs Without Verification Steps

- **WHY**: Others need to confirm the fix works.
- **Consequence**: Unclosed feedback loop, potential recurrence.
- **BAD**: "Fixed in PR #123."
- **GOOD**: "Fixed in PR #123. Verify by: 1) Go to /users, 2) Click sort, 3) Should see sorted list."

### NEVER Ignore Visual Regression Testing

- **WHY**: CSS changes can break unrelated layouts.
- **Consequence**: Undetected visual bugs in production.
- **BAD**: "Just a small style change, no need to test."
- **GOOD**: Run visual regression tests or manually check related components.
```

---

### Phase 3: Pattern Recognition (D7) - High Priority

**Goal**: Increase D7 score from 6/10 to 9/10

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 3.1**: Enhance frontmatter

```yaml
---
name: ui-debug-workflow
description: |
  Complete workflow for debugging UI changes with visual testing, evidence collection, 
  and comprehensive reporting.
  
  Use when: debugging visual issues, investigating UI bugs, collecting bug evidence,
  verifying UI fixes, or conducting visual regression testing.
  
  Triggers: "UI bug", "visual issue", "not rendering", "CSS problem", "layout broken",
  "component not working", "DevTools", "inspect element", "browser console",
  "visual regression", "screenshot comparison", "evidence collection".
---
```

**Step 3.2**: Add explicit trigger section

````markdown
## When to Use This Skill

### Explicit Triggers
- "Debug this UI issue"
- "Visual regression detected"
- "Why isn't this rendering correctly?"
- "Help me use DevTools"
- "Collect evidence for this bug"

### Implicit Triggers
- User reports a visual bug
- User sees unexpected UI behavior
- User asks about browser DevTools
- User is investigating CSS issues
- User needs to document a bug

### Decision Tree

```
What type of issue?
├── Visual (layout, styling)
│   → Check Elements panel, computed styles
│   → Use responsive design mode
│
├── Functional (clicks not working)
│   → Check Console for errors
│   → Add breakpoints in Sources
│   → Verify event handlers
│
├── Performance (slow, janky)
│   → Use Performance panel
│   → Check for layout thrashing
│   → Profile render times
│
└── Network (not loading)
    → Check Network panel
    → Verify API responses
    → Check CORS headers
```
````

---

### Phase 4: Knowledge Delta (D1) - Medium Priority

**Goal**: Increase D1 score from 15/20 to 17/20

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 4.1**: Add non-obvious guidance

```markdown
## Non-Obvious Debugging Insights

### Layout Thrashing Detection

When animations feel janky, check for layout thrashing:
- Reading `offsetWidth` after writing styles
- Alternating style reads and writes in loops
- Fix: Batch reads, then batch writes

### CSS Specificity Conflicts

When styles don't apply:
1. Check computed styles panel for crossed-out rules
2. Look for `!important` overrides
3. Verify CSS module scoping
4. Check for inline styles from JavaScript

### React-Specific Issues

Common React debugging gotchas:
- Stale closures in event handlers
- Missing dependency arrays in hooks
- Incorrect key props causing re-renders
- State mutation instead of replacement

### Z-Index Stacking Contexts

When z-index doesn't work:
- Check for new stacking contexts (transform, opacity, position)
- Use stacking context inspector in DevTools
- Remember: z-index only works within same context
```

---

### Phase 5: Specification Compliance (D4) - Medium Priority

**Goal**: Increase D4 score from 10/15 to 13/15

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 5.1**: Complete frontmatter

```yaml
---
name: ui-debug-workflow
description: |
  Complete workflow for debugging UI changes with visual testing, evidence collection, 
  and comprehensive reporting.
  
  Use when: debugging visual issues, investigating UI bugs, collecting bug evidence.
keywords:
  - debugging
  - UI
  - DevTools
  - visual testing
  - evidence collection
  - bug report
  - browser console
---
```

**Step 5.2**: Add scope section

```markdown
## Scope

### In Scope
- Browser DevTools usage
- Visual debugging workflows
- Evidence collection procedures
- Bug documentation
- Fix verification

### Out of Scope
- Backend debugging
- Database debugging
- Network infrastructure issues
- Security penetration testing
```

---

### Phase 6: Freedom Calibration (D6) - Medium Priority

**Goal**: Increase D6 score from 10/15 to 13/15

**File**: `skills/ui-debug-workflow/SKILL.md`

**Step 6.1**: Clarify hard vs. soft constraints

```markdown
## Constraints vs. Flexibility

### Hard Constraints (Always Required)

1. Always capture reproduction steps before debugging
2. Always check console errors first
3. Always document evidence before closing bugs
4. Always verify fixes with the same reproduction steps

### Flexible Guidance (Adapt to Context)

- Browser choice for testing (prioritize user's primary browser)
- Level of detail in bug reports (match stakeholder needs)
- Tools selection (DevTools vs. extensions vs. external)
- Visual testing approach (manual vs. automated)

### Fallback Paths

If issue cannot be reproduced:
1. Gather all available context (user reports, logs)
2. Hypothesize most likely causes
3. Add defensive logging
4. Monitor for recurrence
```

---

## Verification Commands

```bash
# Run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh ui-debug-workflow --json

# Verify references created
ls -la skills/ui-debug-workflow/references/

# Check line count
wc -l skills/ui-debug-workflow/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Validate markdown
bunx markdownlint-cli2 "skills/ui-debug-workflow/**/*.md"
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 96/120 |
| Grade | B or higher |
| D5: Progressive Disclosure | >= 13/15 |
| D3: Anti-Pattern Quality | >= 13/15 |
| D7: Pattern Recognition | >= 9/10 |
| D1: Knowledge Delta | >= 17/20 |
| D4: Specification Compliance | >= 13/15 |
| D6: Freedom Calibration | >= 13/15 |
| Reference files created | >= 3 |
| Anti-patterns with BAD/GOOD | >= 6 |
| SKILL.md line count | < 200 lines |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Progressive Disclosure | 2 hours | None |
| Phase 2: Anti-Patterns | 1 hour | None |
| Phase 3: Pattern Recognition | 1 hour | None |
| Phase 4: Knowledge Delta | 1 hour | Phase 1 |
| Phase 5: Specification Compliance | 30 min | None |
| Phase 6: Freedom Calibration | 30 min | None |
| Verification | 30 min | All phases |

**Total Estimated Time**: 6 hours
