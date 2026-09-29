---
name: aws-console-navigator
description: "Navigate the AWS Console via browser or Playwright MCP — SSO authentication, account selection, region switching, CloudWatch Logs Insights, alarms, metrics. Covers iframe DOM patterns for screenshot capture: scroll, row click, element interaction inside the microConsole iframe. Keywords: Playwright SSO iframe CloudWatch browser-automation screenshot microConsole logs-table__wrapper page.mouse.click frame.evaluate IAM-Identity-Center region cookie-banner sidebar-collapse locator-timeout."
sources: []
---

# AWS Console Navigator

## Mindset

The CloudWatch console is a **micro-frontend SPA embedded inside an iframe**
(`iframe[name="microConsole-Logs"]`). This has three non-obvious consequences:

1. **`window.document` is the outer shell, not CloudWatch.** All DOM queries must target
   the iframe's `contentDocument`. In Playwright, use `page.frame({ name: 'microConsole-Logs' })`.

2. **`locator.click()` on iframe elements times out.** Playwright's locator API hits a 5 s
   timeout when clicking elements inside cross-origin iframes. Use `page.mouse.click(x, y)`
   with coordinates calculated from the iframe bounding box + element's `getBoundingClientRect`.

3. **The results table is its own scroll container.** `window.scrollBy()` scrolls the outer
   shell. The CloudWatch results table lives in `DIV.logs-table__wrapper`. Scroll it directly:
   `frame.evaluate(() => document.querySelector('.logs-table__wrapper').scrollTop = N)`.

## When to Use This Skill

- Capturing AWS Console screenshots via Playwright MCP
- Authenticating via SSO and opening a specific account/role
- Getting step-by-step instructions to reach a CloudWatch Logs Insights query, alarm, or metric
- Guiding a user to navigate the console manually

## When NOT to Use This Skill

- The target service does not use the microConsole iframe (e.g. S3, EC2) — `locator.click()` works normally there
- You only need to construct a CloudWatch URL without opening a browser — use `aws-cloudwatch-url-builder` instead

---

## Navigating to CloudWatch Pages

### Logs Insights

1. Services → CloudWatch → Logs → Logs Insights
2. Select the log group, paste the query, set the time range (Absolute), click **Run query**

**Direct URL:** See `aws-cloudwatch-url-builder` for the encoded URL template.

### Alarms

1. CloudWatch → Alarms → All alarms → search by alarm name → click alarm name
2. Verify which metric is graphed before capturing screenshots

**Direct URL:**
```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}
```

### Metrics

1. CloudWatch → Metrics → All metrics → select namespace → filter by dimension → Graph metrics
2. Adjust time range in the graph toolbar

For full SSO auth steps and region codes, see `references/console-reference.md`.
Full Playwright code snippets are in `references/playwright-patterns.md`.

---

## Quick Playwright Patterns

Get the frame reference first — optionally cache it for reuse across multiple operations:

```javascript
const frame = page.frame({ name: 'microConsole-Logs' });
await page.waitForSelector('iframe[name="microConsole-Logs"]', { timeout: 15000 });
```

Scroll the results table (use `frame.evaluate`, not `window.scrollBy`):

```javascript
await frame.evaluate(() => {
  document.querySelector('.logs-table__wrapper').scrollTop = 600;
});
```

Expand a log row using page-level coordinates (consider pre-calculating for multiple rows):

```javascript
const iframeBox = await page.locator('iframe[name="microConsole-Logs"]').boundingBox();
const rowBox = await frame.locator('tr.logs-table__row').first().boundingBox();
await page.mouse.click(iframeBox.x + rowBox.x + 20, iframeBox.y + rowBox.y + rowBox.height / 2);
```

---

## Anti-Patterns

Each entry shows the **BAD** practice and the **GOOD** replacement.

---

❌ **NEVER use `locator.click()` directly on CloudWatch iframe elements**

WHY: Playwright's `locator.click()` uses accessibility tree traversal and times out after 5 s
when targeting elements inside a cross-origin or sandboxed iframe.

✅ BAD: `await frame.locator('tr').first().click()` → GOOD: calculate page-level coordinates
from `iframeBox + rowBox`, then call `page.mouse.click(absoluteX, absoluteY)`.

---

❌ **NEVER use `window.scrollBy()` or `window.scrollTo()` inside the CloudWatch iframe**

WHY: `window` in the iframe context scrolls `MAIN.logs__main__visual-refresh` (the outer
page-level scroller), not the results table. The results table is in a separate overflow
container (`DIV.logs-table__wrapper`) and will not move.

✅ Use: `frame.evaluate(() => document.querySelector('.logs-table__wrapper').scrollTop = N)`

---

❌ **NEVER query `window.document` from `page.evaluate()` to interact with CloudWatch content**

WHY: `page.evaluate()` runs in the outer shell document. CloudWatch content is inside the iframe.
`document.querySelector('.logs-table__wrapper')` returns null from `page.evaluate()`.

✅ Use `frame.evaluate()` where `frame = page.frame({ name: 'microConsole-Logs' })`.

---

❌ **NEVER assume `AWS/Lambda Errors = 0` means no alarm triggered**

WHY: The native `AWS/Lambda Errors` metric counts invocation-level failures (exit code ≠ 0).
An alarm driven by a **Logs metric filter** fires even when the invocation exits cleanly, as
long as ERROR-level log lines were emitted.

✅ Always screenshot the alarm detail page, not the raw `AWS/Lambda Errors` metric graph.

## References

- [SSO authentication flow and region codes](references/console-reference.md)
- [Playwright code patterns for iframe interaction](references/playwright-patterns.md)
- **aws-cloudwatch-url-builder** — companion skill for constructing encoded CloudWatch deep-link URLs
