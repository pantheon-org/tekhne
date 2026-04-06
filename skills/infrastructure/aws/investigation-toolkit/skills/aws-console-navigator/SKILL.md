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
   with coordinates calculated from the iframe's bounding box + element's `getBoundingClientRect`.

3. **The results table is its own scroll container.** `window.scrollBy()` scrolls the outer
   shell. The CloudWatch results table lives in `DIV.logs-table__wrapper`, a separate
   overflow container. Scroll it directly: `frame.evaluate(() => document.querySelector('.logs-table__wrapper').scrollTop = N)`.

## When to Use This Skill

- You are capturing AWS Console screenshots via Playwright MCP
- You need to authenticate via SSO and open a specific account/role
- You need step-by-step instructions to reach a CloudWatch Logs Insights query, alarm, or metric
- You are guiding a user to navigate the console manually

---

## SSO Authentication Flow

### Step 1 — Open the SSO portal

URL: `https://d-996716c793.awsapps.com/start`

Wait for: `AWS IAM Identity Center` heading or the account list to appear.

### Step 2 — Select the account

The portal lists accounts. Use the search box or scroll to find the target account name or ID.
Click the account row to expand it and reveal available permission sets (roles).

| AWS Profile | Account alias pattern |
|-------------|----------------------|
| `ppl-sw-pr` | `ppl-sw-pr` or similar |

### Step 3 — Open the console for the correct role

Click the role name (e.g. `PowerUserAccess`, `AdministratorAccess`) under the account.
A new browser tab opens with the AWS Management Console for that account + role.

> **Cookie/banner dismissal:** dismiss before navigating further:
> - Cookie banner: click `Accept all` or `Reject all`
> - New experience banner: click `X` or `Dismiss`

### Step 4 — Set the region

The region selector is in the top-right corner of the console header. Click it and type or
select the target region (e.g. `EU (Ireland)` → `eu-west-1`).

Confirm the URL contains `region=eu-west-1` after switching.

---

## Navigating to CloudWatch Pages

### CloudWatch Logs Insights

**Manual path:**
1. Services → CloudWatch (or search "CloudWatch" in the top bar)
2. Left sidebar → Logs → Logs Insights
3. In the log group selector, type the log group name and select it
4. Paste the query into the query editor
5. Set the time range (Absolute: enter start/end times)
6. Click **Run query**

**Direct URL:** See `aws-cloudwatch-url-builder` skill for the encoded URL template.

### CloudWatch Alarms

**Manual path:**
1. CloudWatch → Alarms → All alarms
2. Search for the alarm name in the filter box
3. Click the alarm name to open the detail page

**Direct URL:**
```
https://{region}.console.aws.amazon.com/cloudwatch/home?region={region}#alarmsV2:alarm/{alarm_name}
```

> Always verify which metric is graphed before capturing screenshots — the alarm may use a
> CloudWatch Logs metric filter, not the native `AWS/Lambda Errors` metric.

### CloudWatch Metrics

**Manual path:**
1. CloudWatch → Metrics → All metrics
2. Select namespace (e.g. `AWS/Lambda`)
3. Filter by dimension (e.g. `FunctionName`)
4. Select the metric(s) and click **Graph metrics**
5. Adjust the time range in the graph toolbar

**Direct URL:** See `aws-cloudwatch-url-builder` skill for the encoded URL template.

---

## Playwright Patterns

### Wait for CloudWatch to finish loading

```javascript
await page.waitForSelector('iframe[name="microConsole-Logs"]', { timeout: 15000 });
```

### Dismiss cookie consent overlay

```javascript
const cookieBtn = page.locator('button:has-text("Accept"), button:has-text("Reject all")').first();
if (await cookieBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
  await cookieBtn.click();
}
```

### Collapse left sidebar (wider screenshot)

```javascript
const closeBtn = page.locator('[aria-label="Close navigation"], [aria-label="Collapse sidebar"]').first();
if (await closeBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
  await closeBtn.click();
}
```

### Scroll the results table

```javascript
const frame = page.frame({ name: 'microConsole-Logs' });
await frame.evaluate(() => {
  document.querySelector('.logs-table__wrapper').scrollTop = 600;
});
```

### Expand a log row (page-coord click)

```javascript
const iframeBox = await page.locator('iframe[name="microConsole-Logs"]').boundingBox();
const frame = page.frame({ name: 'microConsole-Logs' });
const rowBox = await frame.locator('tr.logs-table__row').first().boundingBox();
await page.mouse.click(
  iframeBox.x + rowBox.x + 20,
  iframeBox.y + rowBox.y + rowBox.height / 2
);
```

### Hide histogram (cleaner screenshot)

```javascript
const frame = page.frame({ name: 'microConsole-Logs' });
await frame.evaluate(() => {
  const el = document.querySelector('.logs__histogram-container');
  if (el) el.style.display = 'none';
});
```

---

## Anti-Patterns

❌ **NEVER use `locator.click()` directly on CloudWatch iframe elements**

Playwright's `locator.click()` uses accessibility tree traversal and times out after 5 s
when targeting elements inside a cross-origin or sandboxed iframe. The click appears to
register but nothing happens, or a `TimeoutError` is thrown.

✅ Calculate page-level coordinates from the iframe bounding box + element
`getBoundingClientRect()`, then use `page.mouse.click(absoluteX, absoluteY)`.

---

❌ **NEVER use `window.scrollBy()` or `window.scrollTo()` inside the CloudWatch iframe**

`window` in the iframe context scrolls `MAIN.logs__main__visual-refresh` (the outer
page-level scroller), not the results table. The results table is in a separate overflow
container (`DIV.logs-table__wrapper`) and will not move.

✅ Use: `frame.evaluate(() => document.querySelector('.logs-table__wrapper').scrollTop = N)`

---

❌ **NEVER query `window.document` from `page.evaluate()` to interact with CloudWatch content**

`page.evaluate()` runs in the outer shell document. CloudWatch content is inside the iframe.
Calling `document.querySelector('.logs-table__wrapper')` from `page.evaluate()` returns null.

✅ Use `frame.evaluate()` where `frame = page.frame({ name: 'microConsole-Logs' })`.

---

❌ **NEVER assume a screenshot of "Lambda Errors = 0" means no alarm**

The native `AWS/Lambda Errors` CloudWatch metric counts invocation-level failures (exit code ≠ 0).
An alarm driven by a **Logs metric filter** (e.g. `ErrorCount-PlayerLambdaFunction`) will fire
even when the invocation exits cleanly, as long as ERROR-level log lines were emitted.
A screenshot showing `AWS/Lambda Errors = 0` is not evidence that the alarm condition was absent.

✅ Always screenshot the alarm detail page, not the raw `AWS/Lambda Errors` metric graph.

---

## Region Codes Reference

| Console display name  | Region code       |
|-----------------------|-------------------|
| EU (Ireland)          | eu-west-1         |
| EU (Frankfurt)        | eu-central-1      |
| US East (N. Virginia) | us-east-1         |
| US West (Oregon)      | us-west-2         |
| AP (Sydney)           | ap-southeast-2    |

## References

- **aws-cloudwatch-url-builder** (companion skill in this tile) — construct encoded deep-link URLs for Logs Insights queries, Alarm detail pages, and Metrics graphs
