# Troubleshooting: Programmatic Selection Doesn't Trigger UI

## Cause

UI frameworks may ignore programmatic JavaScript events and only respond to real user interactions.

## Problem

```bash
# This doesn't trigger annotation toolbar:
agent-browser --session test eval "
  const el = document.querySelector('pre code');
  const range = document.createRange();
  range.selectNodeContents(el);
  window.getSelection().removeAllRanges();
  window.getSelection().addRange(range);
"
```

## Solution: Use Full Playwright

### Real Mouse Interactions

```typescript
import { test } from "@playwright/test";

test("select text with real mouse", async ({ page }) => {
  await page.goto("http://localhost:3000");
  
  const codeBlock = page.locator("pre code").first();
  
  // Triple-click to select all (real mouse event)
  await codeBlock.click({ clickCount: 3 });
  
  // Wait for toolbar to appear
  await page.waitForSelector('[role="toolbar"]', { timeout: 2000 });
});
```

### Alternative: Trigger Events Manually

Sometimes you can dispatch events:

```typescript
await page.evaluate(() => {
  const el = document.querySelector('pre code');
  const range = document.createRange();
  range.selectNodeContents(el);
  window.getSelection().removeAllRanges();
  window.getSelection().addRange(range);
  
  // Dispatch events
  el.dispatchEvent(new MouseEvent('mouseup', { bubbles: true }));
  document.dispatchEvent(new Event('selectionchange'));
});
```

But this doesn't always work. Real mouse interactions are more reliable.

## When agent-browser is Insufficient

**Signs you need full Playwright**:
1. Text selection doesn't trigger UI toolbars
2. Drag-and-drop required
3. Complex event sequences
4. Hover effects needed

**Solution**: See [Playwright Testing](../browser-automation/playwright-testing.md)

## Related Documentation

- [Agent-browser Limitations](../browser-automation/agent-browser.md#capabilities)
- [Playwright Testing](../browser-automation/playwright-testing.md)
