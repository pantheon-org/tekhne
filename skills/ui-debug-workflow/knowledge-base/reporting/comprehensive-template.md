# Comprehensive Debug Report Template

Use this template for complete debug reports with evidence.

## Template

````markdown
# Debug Report: [Issue Title]

**Date**: YYYY-MM-DD  
**Reporter**: [Your name]  
**Status**: ✅ Fixed | ⚠️ In Progress | ❌ Not Fixed

---

## Executive Summary

[2-3 sentences summarizing the issue, fix, and result]

## Issue Description

**Original Problem**:
- Description of the bug or missing feature
- Steps to reproduce
- Expected vs actual behavior

**Impact**:
- User-facing impact
- Technical impact
- Severity: Critical | High | Medium | Low

## Changes Implemented

**Files Modified**:
```bash
M packages/ui/components/Viewer.tsx
M packages/ui/utils/highlighter.ts
```

**Key Changes**:
1. **File:line** - Description of change
2. **File:line** - Description of change

**Git Diff**: [See Appendix A]

## Test Environment

| Component | Version/Details |
|-----------|-----------------|
| **OS** | macOS 14.2 |
| **Browser** | Chromium 120.0.6099.129 |
| **Node/Bun** | Bun 1.0.23 |
| **Build Tool** | Vite 5.0.10 |
| **Server** | localhost:19888 |
| **Test Date** | YYYY-MM-DD HH:MM UTC |

## Test Methodology

### Baseline Capture
1. Checked out `main` branch (commit `abc123`)
2. Built application: `bun run build:hook`
3. Started server on port 19888
4. Captured screenshots and DOM snapshots
5. Documented observed behaviors and bugs

### Changed State Testing
1. Checked out `fix/feature` branch (commit `def456`)
2. Rebuilt application with changes
3. Restarted server on same port
4. Repeated identical test steps
5. Compared results with baseline

### Test Cases

- ✅ **TC-01**: Test description
- ✅ **TC-02**: Test description
- ⚠️ **TC-03**: Test description (partial)
- ❌ **TC-04**: Test description (failed)

## Evidence

### Test Case 01: Description

**Before (Baseline)**:

![Baseline screenshot](./baseline/01-screenshot.png)

**Observations**:
- What you saw
- Issues identified

**After (With Fix)**:

![Changed screenshot](./changed/01-screenshot.png)

**Observations**:
- What changed
- Issues resolved

**Result**: ✅ **PASS** - Description of outcome

---

[Repeat for each test case]

---

## Results Summary

| Test Case | Description | Baseline | Changed | Status |
|-----------|-------------|----------|---------|--------|
| TC-01 | Description | ✅ Pass | ✅ Pass | ✅ No regression |
| TC-02 | Description | ❌ Fail | ✅ Pass | ✅ **FIXED** |
| TC-03 | Description | ✅ Pass | ✅ Pass | ✅ No regression |

**Overall Result**: ✅ **FIXED** - Summary

## Known Limitations

### Limitation 1
**Issue**: Description  
**Impact**: What it affects  
**Mitigation**: How you worked around it

## Recommendations

### Immediate Actions
1. ✅ Action item with status
2. ⚠️ Action item with warning
3. 📝 Action item for documentation

### Future Improvements
1. Description of improvement
2. Description of improvement

## Appendix

### Appendix A: Git Diff

```diff
[Include git diff or link to file]
```

### Appendix B: Build Output

```
[Build log excerpt]
```

### Appendix C: Browser Console Logs

```
[Console output]
```

### Appendix D: Test Artifacts

**Location**: `./docs/changefix/YYYY-MM-DD-issue-name/`

**Contents**:
```
baseline/
  01-initial-load.png
  dom-snapshot.html
changed/
  01-initial-load.png
  dom-snapshot.html
changes.diff
build-output.log
```

---

**Report Generated**: YYYY-MM-DD HH:MM UTC  
**Tool**: OpenCode UI Debug Workflow Skill
````

## Simplified Template

For quick reports:

```markdown
# Debug Report: [Issue]

## Summary
- **Issue**: Brief description
- **Fix**: What was changed
- **Result**: ✅ Fixed | ⚠️ Partial | ❌ Not Fixed

## Evidence

### Before
![Baseline](./baseline/screenshot.png)
- Observed bug: Description

### After
![Changed](./changed/screenshot.png)
- Bug fixed: Description

## Changes
- File:line - Description

## Test Results
- ✅ TC-01: Description
- ✅ TC-02: Description

## Next Steps
1. Action item
2. Action item
```

## Related Documentation

- [Evidence Collection](../evidence-collection/) - How to gather evidence
- [Git Context](../evidence-collection/git-context.md) - Including version info
