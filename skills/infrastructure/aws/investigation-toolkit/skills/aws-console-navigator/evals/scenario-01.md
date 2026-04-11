# Scenario 01: Expand a Log Row in CloudWatch Logs Insights via Playwright

## User Prompt

You have a Playwright `page` object. The browser is on a CloudWatch Logs Insights page showing a results table with 2 rows. You need to click on the first row to expand it and reveal the full log event JSON.

Write the Playwright code to expand the first row.

## Expected Behavior

1. Call `page.frame({ name: 'microConsole-Logs' })` (or equivalent) to obtain the iframe frame object
2. Not use `locator.click()` on an element inside the iframe
3. Call `boundingBox()` on the iframe element from the outer page context
4. Call `boundingBox()` on the row element using the frame locator
5. Call `page.mouse.click(x, y)` with absolute coordinates combining the iframe and row offsets
6. Compute coordinates using a formula like: `x = iframeBox.x + rowBox.x + offset`, `y = iframeBox.y + rowBox.y + rowBox.height/2`
7. Target the first row using `.first()` or index `[0]`

## Success Criteria

- **Obtains iframe frame object**: Code calls page.frame({ name: 'microConsole-Logs' }) or equivalent to get the iframe context
- **Does NOT use locator.click() on iframe elements**: Code does not call .click() on a locator targeting an element inside the iframe
- **Gets iframe bounding box**: Code calls boundingBox() on the iframe element from the outer page context
- **Gets row bounding box from frame**: Code calls boundingBox() on the row element using the frame locator
- **Uses page.mouse.click with absolute coordinates**: Code calls page.mouse.click(x, y) where x and y combine iframe and row offsets
- **Coordinate formula is correct**: x = iframeBox.x + rowBox.x + offset, y = iframeBox.y + rowBox.y + rowBox.height/2 (or similar centred y)
- **Targets first row correctly**: Code selects the first row (e.g. .first() or [0] index) not all rows

## Failure Conditions

- Does not call `page.frame(...)` to obtain the iframe context
- Uses `locator.click()` directly on an element inside the CloudWatch iframe
- Does not call `boundingBox()` on the iframe element
- Does not call `boundingBox()` on the row element within the frame
- Uses `page.mouse.click` but with incorrect coordinates (not combining iframe and row offsets)
- Coordinate formula produces incorrect center-of-row y position
- Targets all rows instead of specifically the first row
