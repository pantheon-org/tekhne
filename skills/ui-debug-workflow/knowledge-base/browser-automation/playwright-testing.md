# Playwright Testing

## Overview

Full Playwright scripts provide complete control over browser interactions, including real mouse events, keyboard input, and complex gestures that agent-browser cannot handle.

## When to Use Playwright

Use full Playwright when:
- Text selection must trigger contextual UI (toolbars, menus)
- Drag-and-drop interactions are required
- Complex event sequences with timing dependencies
- Visual regression testing (screenshot comparison)
- Comprehensive test coverage needed

## Basic Example

```typescript
// test-ui-changes.spec.ts
import { test, expect } from "@playwright/test";

test("code block annotation works", async ({ page }) => {
  await page.goto("http://localhost:19888");

  // Wait for page load
  await page.waitForSelector("pre code");

  // Take baseline screenshot
  await page.screenshot({ path: "baseline.png", fullPage: true });

  // Select text in code block (real mouse interaction)
  const codeBlock = page.locator("pre code").first();
  await codeBlock.scrollIntoViewIfNeeded();

  // Triple-click to select all text in code block
  await codeBlock.click({ clickCount: 3 });

  // Wait for toolbar to appear
  await page.waitForSelector('[role="toolbar"]', { timeout: 2000 });

  // Click "Redline" button
  await page.getByRole("button", { name: /redline/i }).click();

  // Verify annotation was created
  const annotations = await page.locator("mark[data-bind-id]").count();
  expect(annotations).toBeGreaterThan(0);

  // Take screenshot of annotated state
  await page.screenshot({ path: "annotated.png", fullPage: true });

  // Test removal
  await page.locator("mark[data-bind-id]").first().click({ button: "right" });
  await page.getByText("Remove").click();

  // Verify annotation removed and syntax highlighting restored
  const remainingAnnotations = await page.locator("mark[data-bind-id]").count();
  expect(remainingAnnotations).toBe(0);

  // Verify code block still has syntax highlighting
  const highlightedElements = await codeBlock.locator("span.hljs-*").count();
  expect(highlightedElements).toBeGreaterThan(0);

  // Final screenshot
  await page.screenshot({ path: "removed.png", fullPage: true });
});
```

## Run the Test

```bash
# Headed mode (see the browser)
npx playwright test test-ui-changes.spec.ts --headed

# Debug mode (step through)
npx playwright test test-ui-changes.spec.ts --debug

# Generate HTML report
npx playwright test test-ui-changes.spec.ts --reporter=html
```

## Interactive Debugging

```typescript
// debug-session.ts
import { chromium } from "playwright";

(async () => {
  const browser = await chromium.launch({ headless: false, slowMo: 500 });
  const page = await browser.newPage();

  // Enable console logging
  page.on("console", (msg) => console.log(`[BROWSER] ${msg.text()}`));

  await page.goto("http://localhost:19888");

  // Pause for manual inspection
  await page.pause();

  // Interactive REPL
  await page.evaluate(() => {
    debugger; // Breaks in DevTools
  });

  await browser.close();
})();
```

**Run with**: `npx playwright test debug-session.ts --debug`

## Capturing Browser Logs

```typescript
page.on("console", (msg) => {
  fs.appendFileSync("console.log", `[${msg.type()}] ${msg.text()}\n`);
});

page.on("pageerror", (error) => {
  fs.appendFileSync("console.log", `[ERROR] ${error.message}\n`);
});
```

## Visual Regression Testing

```typescript
test("visual regression", async ({ page }) => {
  await page.goto("http://localhost:3000");
  
  // Compare screenshot with baseline
  await expect(page).toHaveScreenshot("homepage.png", {
    maxDiffPixels: 100 // Allow minor differences
  });
});
```

## CI/CD Integration

```yaml
# .github/workflows/visual-regression.yml
name: Visual Regression Tests

on: [pull_request]

jobs:
  visual-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: oven-sh/setup-bun@v1

      - name: Install dependencies
        run: bun install

      - name: Install Playwright
        run: npx playwright install chromium

      - name: Build baseline (main)
        run: |
          git checkout main
          bun run build
          # Save baseline screenshots

      - name: Build changed (PR branch)
        run: |
          git checkout ${{ github.head_ref }}
          bun run build
          # Save changed screenshots

      - name: Compare screenshots
        run: ./scripts/compare-visual-evidence.sh

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: visual-test-results
          path: docs/changefix/
```

## Key Differences from agent-browser

| Feature | agent-browser | Playwright |
|---------|---------------|------------|
| Screenshot | ✅ Fast | ✅ Full control |
| DOM inspection | ✅ Accessibility tree | ✅ Full DOM access |
| Text selection | ❌ Doesn't trigger UI | ✅ Real mouse events |
| Drag-and-drop | ❌ Not supported | ✅ Full support |
| Keyboard input | ❌ Limited | ✅ Complete |
| Event timing | ❌ Manual waits | ✅ Smart waiting |
| Visual regression | ❌ Manual comparison | ✅ Built-in |

## Related Documentation

- [Agent-browser](./agent-browser.md) - Basic automation
- [Evidence Collection](../evidence-collection/) - Screenshot and log best practices
- [Example Tests](../../examples/ui-debug-test.spec.ts) - Complete test suite
