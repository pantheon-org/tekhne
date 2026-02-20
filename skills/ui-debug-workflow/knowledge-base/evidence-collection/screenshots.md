# Screenshots

## Naming Convention

Use descriptive, sequential naming:

```
[sequence]-[stage]-[description].png

Examples:
01-baseline-initial-load.png
02-baseline-text-selected.png
03-changed-initial-load.png
04-changed-text-selected.png
05-changed-annotation-applied.png
```

## What to Capture

- ✅ Initial page load
- ✅ Each step of the interaction flow
- ✅ Error states (if any)
- ✅ Final state after all interactions
- ✅ Full page AND focused element screenshots

## Best Practices

### Full Page vs Focused

```bash
# Full page screenshot
agent-browser --session test screenshot full-page.png

# Or with Playwright
await page.screenshot({ path: "full.png", fullPage: true });

# Focused element
const element = page.locator("pre code").first();
await element.screenshot({ path: "focused.png" });
```

### Timing

Always wait for page to be fully ready:

```bash
# agent-browser: manual wait
sleep 3

# Playwright: smart waiting
await page.waitForLoadState('networkidle');
await page.waitForSelector('pre code');
```

## Visual Comparison

### Using ImageMagick

```bash
# Install
brew install imagemagick  # macOS
apt-get install imagemagick  # Linux

# Compare two screenshots
compare baseline.png changed.png diff.png

# Highlight differences in red
compare -fuzz 5% -highlight-color red baseline.png changed.png diff.png
```

### Playwright Visual Regression

```typescript
test("visual regression", async ({ page }) => {
  await page.goto("http://localhost:3000");
  
  // Compare with baseline
  await expect(page).toHaveScreenshot("homepage.png", {
    maxDiffPixels: 100 // Allow minor differences
  });
});
```

## Troubleshooting

See [../troubleshooting/blank-screenshots.md](../troubleshooting/blank-screenshots.md) for common issues.
