---
name: ui-debug-workflow
description:
  Complete workflow for debugging UI changes with visual testing, evidence collection, and comprehensive reporting
license: MIT
compatibility: opencode
metadata:
  applies_to: ui, frontend, react, web-development, testing, debugging
  methodology: visual-testing, evidence-based-debugging
  tools: playwright, browser-automation, screenshots, git
---

# UI Debug Workflow

Systematic, evidence-based workflow for debugging and validating UI changes.

## What I Do

I guide you through a 4-phase debugging workflow:

1. **Capture Baseline** - Document current behavior before changes
2. **Apply & Test Changes** - Implement fix and capture new behavior
3. **Compare & Analyze** - Document differences and verify the fix
4. **Generate Report** - Create comprehensive debug report with evidence

## When to Use Me

- ✅ Validating UI bug fixes
- ✅ Testing new UI features
- ✅ Visual regression testing
- ✅ Interactive behavior testing
- ✅ Creating evidence-based reports for stakeholders

## Prerequisites

```bash
# Install Playwright browsers
npx playwright install chromium

# Verify agent-browser (optional but recommended)
which agent-browser
```

**Project requirements**: Build scripts, development server, Git

## Quick Start

### Automated Workflow

Use the provided scripts for complete automation:

```bash
~/.config/opencode/skills/ui-debug-workflow/scripts/full-debug-session.sh \
  main \
  fix/my-feature \
  "http://localhost:3000" \
  "npm run build" \
  "npm start"
```

**Output**: Complete debug report in `./docs/change-fix/YYYY-MM-DD-HHMMSS/`

### Manual Workflow

#### Phase 1: Capture Baseline (~10-15 min)

```bash
# 1. Checkout baseline
git checkout main

# 2. Build and start
npm run build
npm start  # Note the port

# 3. Capture evidence
./scripts/capture-evidence.sh baseline http://localhost:3000 ./baseline

# Evidence collected:
# - Screenshots at key points
# - DOM snapshots
# - Element counts and status
```

#### Phase 2: Apply & Test Changes (~10-15 min)

```bash
# 1. Checkout fix branch
git checkout fix/my-feature
git diff main...HEAD > changes.diff

# 2. Rebuild
npm run build
npm start  # Same port

# 3. Capture evidence
./scripts/capture-evidence.sh changed http://localhost:3000 ./changed
```

#### Phase 3: Compare & Analyze (~5-10 min)

```bash
# Compare evidence
./scripts/compare-evidence.sh ./baseline ./changed ./comparison.md

# Review:
# - Side-by-side screenshots
# - JSON data diffs
# - Visual differences
```

#### Phase 4: Generate Report (~15-20 min)

Fill out the comprehensive debug report template:

- Executive summary with status (✅/⚠️/❌)
- Test environment details
- Before/after evidence
- Test results table
- Findings and recommendations

**See**: [knowledge-base/reporting/comprehensive-template.md](./knowledge-base/reporting/comprehensive-template.md)

## Browser Automation

### agent-browser (Basic)

Good for: Screenshots, DOM inspection, simple clicks

```bash
agent-browser --session test open http://localhost:3000
agent-browser --session test screenshot output.png
agent-browser --session test snapshot > dom.txt
agent-browser --session test close
```

**Limitations**: Cannot trigger complex UI interactions (toolbars, drag-and-drop)

**See**: [knowledge-base/browser-automation/agent-browser.md](./knowledge-base/browser-automation/agent-browser.md)

### Playwright (Full Control)

Required for: Real mouse interactions, complex gestures, visual regression

```typescript
import { test, expect } from "@playwright/test";

test("ui test", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.locator("pre code").click({ clickCount: 3 }); // Real mouse
  await page.screenshot({ path: "screenshot.png" });
});
```

**See**:
[knowledge-base/browser-automation/playwright-testing.md](./knowledge-base/browser-automation/playwright-testing.md)

## Evidence Collection

### Screenshots

**Best practices**:

- Use sequential naming: `01-initial-load.png`, `02-text-selected.png`
- Capture full page AND focused elements
- Take screenshots at each interaction step

**See**: [knowledge-base/evidence-collection/screenshots.md](./knowledge-base/evidence-collection/screenshots.md)

### DOM Snapshots

**Two types**:

1. Accessibility tree (quick, for element IDs)
2. Full HTML (complete, for debugging)

**See**: [knowledge-base/evidence-collection/dom-snapshots.md](./knowledge-base/evidence-collection/dom-snapshots.md)

### Logs

**Capture**:

- Build output: `npm run build 2>&1 | tee build.log`
- Server logs: `npm start 2>&1 | tee server.log`
- Browser console (via Playwright)
- Git context: `git diff main...HEAD > changes.diff`

**See**: [knowledge-base/evidence-collection/logs.md](./knowledge-base/evidence-collection/logs.md)

## Workflow Checklist

**Phase 1: Baseline** (~10-15 min)

- [ ] Checkout baseline branch
- [ ] Build application
- [ ] Start server (note port)
- [ ] Capture screenshots
- [ ] Save DOM snapshots
- [ ] Document bugs
- [ ] Save to `baseline/`

**Phase 2: Changes** (~10-15 min)

- [ ] Checkout fix branch
- [ ] Save git diff
- [ ] Rebuild application
- [ ] Restart server (same port)
- [ ] Repeat identical test steps
- [ ] Save to `changed/`

**Phase 3: Compare** (~5-10 min)

- [ ] Compare screenshots side-by-side
- [ ] Diff DOM snapshots
- [ ] Analyze differences
- [ ] Run comparison script

**Phase 4: Report** (~15-20 min)

- [ ] Fill out report template
- [ ] Include evidence
- [ ] Document test results
- [ ] Add recommendations

**Total time**: ~40-50 minutes

## Scripts

All scripts located in `scripts/`:

- **`capture-evidence.sh`** - Single evidence capture session
- **`compare-evidence.sh`** - Compare baseline vs changed
- **`full-debug-session.sh`** - Complete end-to-end workflow

**Example**:

```bash
./scripts/capture-evidence.sh session-name http://localhost:3000 ./output
```

## Troubleshooting

**Common Issues**:

1. **agent-browser not found**
   - Solution: Use Playwright directly or install agent-browser
   - See:
     [knowledge-base/troubleshooting/agent-browser-not-found.md](./knowledge-base/troubleshooting/agent-browser-not-found.md)

2. **Blank screenshots**
   - Cause: Page not loaded
   - Solution: Add wait conditions
   - See: [knowledge-base/troubleshooting/blank-screenshots.md](./knowledge-base/troubleshooting/blank-screenshots.md)

3. **Text selection doesn't trigger UI**
   - Cause: Programmatic selection ignored
   - Solution: Use Playwright with real mouse
   - See:
     [knowledge-base/troubleshooting/programmatic-selection.md](./knowledge-base/troubleshooting/programmatic-selection.md)

## Knowledge Base

Detailed documentation organized by domain:

```sh
knowledge-base/
├── browser-automation/
│   ├── agent-browser.md           # Basic automation
│   └── playwright-testing.md      # Full browser control
├── evidence-collection/
│   ├── screenshots.md             # Screenshot best practices
│   ├── dom-snapshots.md           # DOM capture techniques
│   ├── logs.md                    # Build, server, browser logs
│   └── git-context.md             # Version control info
├── reporting/
│   └── comprehensive-template.md  # Full report template
└── troubleshooting/
    ├── agent-browser-not-found.md
    ├── blank-screenshots.md
    └── programmatic-selection.md
```

## Examples

Complete test suite: [examples/ui-debug-test.spec.ts](./examples/ui-debug-test.spec.ts)

## Remember

**Evidence-based debugging is systematic, not speculative.**

- ✅ Capture baseline BEFORE making changes
- ✅ Test changes in identical conditions
- ✅ Document everything (screenshots, logs, commands)
- ✅ Compare objectively (side-by-side evidence)
- ✅ Report comprehensively (what worked, what didn't, what's next)

**Use this skill when**:

- Visual changes need validation
- Stakeholders need evidence
- Regression testing required
- Documentation is important

**Skip this skill when**:

- Unit tests cover the change
- Change is non-visual (API, backend)
- Issue is trivial and low-risk
