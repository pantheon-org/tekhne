# Playwright Patterns for AWS Console

## Prerequisites

```javascript
// Always get the frame reference first — all CloudWatch interactions go through it
const frame = page.frame({ name: 'microConsole-Logs' });
```

## Wait for CloudWatch to Load

```javascript
await page.waitForSelector('iframe[name="microConsole-Logs"]', { timeout: 15000 });
```

## Dismiss Cookie Consent Overlay

```javascript
const cookieBtn = page.locator('button:has-text("Accept"), button:has-text("Reject all")').first();
if (await cookieBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
  await cookieBtn.click();
}
```

## Collapse Left Sidebar (Wider Screenshot)

```javascript
const closeBtn = page.locator('[aria-label="Close navigation"], [aria-label="Collapse sidebar"]').first();
if (await closeBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
  await closeBtn.click();
}
```

## Scroll the Results Table

```javascript
const frame = page.frame({ name: 'microConsole-Logs' });
await frame.evaluate(() => {
  document.querySelector('.logs-table__wrapper').scrollTop = 600;
});
```

## Expand a Log Row (Page-Coordinate Click)

Must use page-level coordinates — `locator.click()` times out on cross-origin iframe elements.

```javascript
const iframeBox = await page.locator('iframe[name="microConsole-Logs"]').boundingBox();
const frame = page.frame({ name: 'microConsole-Logs' });
const rowBox = await frame.locator('tr.logs-table__row').first().boundingBox();
await page.mouse.click(
  iframeBox.x + rowBox.x + 20,
  iframeBox.y + rowBox.y + rowBox.height / 2
);
```

## Hide Histogram (Cleaner Screenshot)

```javascript
const frame = page.frame({ name: 'microConsole-Logs' });
await frame.evaluate(() => {
  const el = document.querySelector('.logs__histogram-container');
  if (el) el.style.display = 'none';
});
```
