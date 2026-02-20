# UI Debug Workflow Skill - Complete Summary

## Overview

Created a comprehensive OpenCode skill for systematic UI debugging based on the Plannotator code block annotation testing session.

**Skill Location**: `~/.config/opencode/skills/ui-debug-workflow/`

---

## What Was Created

### 1. Core Documentation (31+ KB total)

**SKILL.md** (27 KB) - Complete workflow documentation including:
- 4-phase debugging workflow (baseline → changed → compare → report)
- Browser automation guide (agent-browser + Playwright)
- Evidence collection best practices
- Comprehensive report template with example
- Troubleshooting guide
- Advanced techniques (visual diffs, CI/CD integration)

**README.md** (4 KB) - Quick start guide:
- Installation instructions
- Usage examples
- Troubleshooting
- Links to full documentation

**INSTALL-SUMMARY.md** (6 KB) - This summary document

### 2. Automation Scripts (3 executable bash scripts)

**capture-evidence.sh**:
- Opens application in agent-browser
- Captures 8 screenshots at key interaction points
- Saves DOM snapshots (accessibility tree + full HTML)
- Analyzes code blocks, syntax highlighting, annotations
- Outputs structured JSON data files

**compare-evidence.sh**:
- Compares baseline and changed evidence directories
- Diffs JSON data with jq
- Creates side-by-side screenshot comparison
- Generates visual diffs with ImageMagick
- Outputs markdown comparison report

**full-debug-session.sh**:
- Complete end-to-end automation
- Switches branches, builds both versions
- Runs evidence capture for baseline and changed
- Generates comparison report
- Creates debug report template
- Outputs: `./docs/change-fix/YYYY-MM-DD-HHMMSS/`

### 3. Example Test Suite

**ui-debug-test.spec.ts** (9 KB):
- Playwright test suite for code block annotations
- 8 test cases covering:
  - Page load with syntax highlighting
  - Text selection in code blocks
  - Annotation creation (auto and manual)
  - Annotation removal
  - Syntax highlighting restoration
  - Multiple annotations
  - Comprehensive report generation
- Demonstrates real mouse interactions vs programmatic
- Shows how to capture visual evidence in tests

---

## Key Learnings Documented

### Agent-browser Capabilities

✅ **Adequate for**:
- Page navigation and screenshots
- DOM inspection (accessibility tree)
- JavaScript execution
- Basic element clicking
- Visual state capture

❌ **Insufficient for**:
- Complex user gestures (text selection, drag-and-drop)
- Triggering contextual UI (toolbars appearing on selection)
- Real mouse interaction testing
- Interactive behavior validation

### When to Use Each Tool

| Task | Tool | Reason |
|------|------|--------|
| Screenshots | agent-browser | Fast, simple, adequate |
| DOM inspection | agent-browser | Good enough for structure analysis |
| Complex interactions | Playwright | Real browser events, full control |
| Visual regression | Playwright | Screenshot comparison built-in |
| CI/CD testing | Playwright | Better automation, reporting |

---

## Workflow Overview

### Phase 1: Baseline Capture (before changes)

```bash
git checkout main
npm run build
npm start
./scripts/capture-evidence.sh baseline-session http://localhost:3000 ./baseline
```

**Output**: Screenshots, DOM snapshots, structured data (JSON)

### Phase 2: Changed State Testing (after changes)

```bash
git checkout fix/my-feature
npm run build
npm start
./scripts/capture-evidence.sh changed-session http://localhost:3000 ./changed
```

**Output**: Same evidence structure for comparison

### Phase 3: Comparison

```bash
./scripts/compare-evidence.sh ./baseline ./changed ./comparison.md
```

**Output**: Side-by-side comparison report with diffs

### Phase 4: Report Generation

Fill out the comprehensive debug report template:
- Summary with status (✅/⚠️/❌)
- Test environment details
- Git changes
- Evidence (screenshots, logs, diffs)
- Test results table
- Findings and recommendations

---

## Usage Examples

### Quick Start: Full Automated Session

```bash
cd ~/.config/opencode/skills/ui-debug-workflow

./scripts/full-debug-session.sh \
  main \
  fix/code-block-annotations \
  "http://localhost:19888" \
  "bun run build:hook" \
  "bun run apps/hook/server/index.ts plan"
```

**Result**: Complete debug report in `./docs/change-fix/YYYY-MM-DD-HHMMSS/`

### In OpenCode Chat

```
User: Follow the ui-debug-workflow to validate this UI fix

Agent: I'll use the ui-debug-workflow skill to:
1. Capture baseline evidence from main branch
2. Capture changed evidence from your fix branch
3. Compare visual and functional differences
4. Generate comprehensive report with screenshots

[Agent runs scripts and generates report]
```

### Manual Step-by-Step

```bash
# 1. Baseline
git checkout main && npm run build
./scripts/capture-evidence.sh baseline http://localhost:3000 ./baseline

# 2. Changed
git checkout fix/feature && npm run build
./scripts/capture-evidence.sh changed http://localhost:3000 ./changed

# 3. Compare
./scripts/compare-evidence.sh ./baseline ./changed ./report.md
```

---

## Evidence Artifacts Generated

Each capture session creates:

| File | Purpose |
|------|---------|
| `01-initial-load.png` | Page on first load |
| `02-dialog-closed.png` | After dismissing dialogs |
| `03-code-blocks-highlighted.png` | Code blocks outlined |
| `04-text-selected.png` | Text selection state |
| `05-annotations-check.png` | Annotation verification |
| `06-syntax-highlighting.json` | Structured highlighting data |
| `07-final-state.png` | Final screenshot |
| `07-full-html.html` | Complete DOM snapshot |
| `*-dom.txt` | Accessibility tree snapshots |
| Various JSON files | Structured test data |

---

## Report Template Structure

The skill provides a comprehensive debug report template:

```markdown
# Debug Report: [Issue Title]

## Summary
- Issue description
- Fix applied
- Result status

## Changes Made
- Git diff
- Files modified

## Test Environment
- Browser, OS, tools, versions

## Evidence
### Before (Baseline)
- Screenshots
- Observations
- Bugs identified

### After (With Fix)
- Screenshots
- Observations
- Bugs resolved

## Test Results
- Test case checklist with ✅/⚠️/❌

## Findings
- Issues fixed
- Regressions (if any)
- Outstanding issues

## Recommendations
- Next steps
- Improvements

## Appendix
- Logs, diffs, artifacts
```

---

## Real-World Application

This skill was created based on testing the Plannotator code block annotation fix:

**Original Issue**:
- `surroundContents()` broke syntax highlighting in code blocks
- Annotations failed with DOM exceptions
- Removing annotations didn't restore syntax highlighting

**Testing Process**:
1. Built baseline (main branch with bugs)
2. Built changed (fix branch with corrections)
3. Captured visual evidence from both
4. Discovered agent-browser limitations for interactive testing
5. Documented full workflow for future use

**Result**: Complete, repeatable debugging workflow with visual proof

---

## Prerequisites

### Required

```bash
# Playwright browsers
npx playwright install chromium

# agent-browser (if using that approach)
# Usually available as CLI tool
```

### Optional (for enhanced features)

```bash
# ImageMagick (visual diffs)
brew install imagemagick  # macOS
apt-get install imagemagick  # Linux

# jq (JSON diffing)
brew install jq  # macOS
apt-get install jq  # Linux
```

---

## File Structure

```
~/.config/opencode/skills/ui-debug-workflow/
├── SKILL.md (27 KB)              # Complete documentation
├── README.md (4 KB)              # Quick start
├── INSTALL-SUMMARY.md (6 KB)    # This file
├── scripts/
│   ├── capture-evidence.sh      # Single evidence capture
│   ├── compare-evidence.sh      # Baseline vs changed
│   └── full-debug-session.sh    # End-to-end automation
├── examples/
│   └── ui-debug-test.spec.ts    # Playwright test template
└── templates/                    # (for future additions)
```

**Total Size**: ~50 KB documentation + scripts

---

## Benefits

✅ **Systematic**: Step-by-step workflow, not ad-hoc testing  
✅ **Evidence-based**: Visual proof with screenshots and logs  
✅ **Comprehensive**: Complete before/after documentation  
✅ **Repeatable**: Scripts automate the entire process  
✅ **Shareable**: Reports suitable for stakeholders  
✅ **Educational**: Documents what works and what doesn't  

---

## Next Steps

### To Use Immediately

1. Install Playwright: `npx playwright install chromium`
2. Run full session on your project
3. Review evidence in `./docs/change-fix/`
4. Fill out report template

### To Extend the Skill

- Add more test scenarios to `examples/`
- Create project-specific templates in `templates/`
- Add integrations (Cypress, Selenium, etc.)
- Contribute CI/CD workflow examples

---

## Skill Metadata

```yaml
name: ui-debug-workflow
description: Complete workflow for debugging UI changes with visual testing
license: MIT
compatibility: opencode
applies_to: ui, frontend, react, web-development, testing, debugging
methodology: visual-testing, evidence-based-debugging
tools: playwright, browser-automation, screenshots, git
```

---

## Documentation

- **Main docs**: `~/.config/opencode/skills/ui-debug-workflow/SKILL.md`
- **Quick start**: `~/.config/opencode/skills/ui-debug-workflow/README.md`
- **OpenCode skills**: https://opencode.ai/docs/skills

---

**Created**: 2026-02-05  
**Session Duration**: ~50 minutes  
**Based On**: Plannotator code block annotation debugging session  
**Status**: ✅ Complete and ready to use

---

## Quick Reference

### Run Full Session
```bash
~/.config/opencode/skills/ui-debug-workflow/scripts/full-debug-session.sh \
  baseline-branch changed-branch url build-cmd start-cmd
```

### Capture Evidence Only
```bash
~/.config/opencode/skills/ui-debug-workflow/scripts/capture-evidence.sh \
  session-name url output-dir
```

### Compare Evidence
```bash
~/.config/opencode/skills/ui-debug-workflow/scripts/compare-evidence.sh \
  baseline-dir changed-dir output-report
```

### Run Playwright Tests
```bash
npx playwright test examples/ui-debug-test.spec.ts --headed
```

---

**The skill is now installed and ready to use for systematic UI debugging with visual evidence!**
