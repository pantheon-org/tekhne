# Troubleshooting: Blank Screenshots

## Cause

Page not fully loaded before screenshot is captured.

## Solution: Add Wait Conditions

### agent-browser

```bash
# Wait for specific element
agent-browser --session test eval "
  await new Promise(resolve => {
    const check = () => {
      if (document.querySelector('pre code')) resolve();
      else setTimeout(check, 100);
    };
    check();
  });
"

# Then take screenshot
agent-browser --session test screenshot output.png
```

### Playwright

```typescript
// Wait for network to be idle
await page.waitForLoadState('networkidle');

// Wait for specific element
await page.waitForSelector('pre code');

// Wait for multiple elements
await Promise.all([
  page.waitForSelector('pre code'),
  page.waitForSelector('button'),
]);

// Then take screenshot
await page.screenshot({ path: "output.png" });
```

## Alternative: Manual Sleep

```bash
# Simple but not ideal
sleep 3
agent-browser --session test screenshot output.png
```

## Related Issues

- [Programmatic Selection Issues](./programmatic-selection.md)
- [Screenshots](../evidence-collection/screenshots.md)
