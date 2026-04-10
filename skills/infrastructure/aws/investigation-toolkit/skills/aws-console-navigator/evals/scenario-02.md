# Scenario 02: Scroll the CloudWatch Logs Results Table in Playwright

## User Prompt

You have a Playwright `page` object. The browser is on a CloudWatch Logs Insights page. A log row has been expanded but the full JSON content is below the visible area.

Write Playwright code to scroll down inside the results table to reveal the content. Use a scroll offset of 600 pixels.

## Expected Behavior

1. Call `page.frame({ name: 'microConsole-Logs' })` to obtain the CloudWatch iframe context
2. Not use `window.scrollBy()` or `window.scrollTo()` for the scroll
3. Not use `page.evaluate()` to query CloudWatch DOM elements — use `frame.evaluate()` instead
4. Target `.logs-table__wrapper` as the scroll container element
5. Set `.scrollTop = 600` directly on the container (not `scrollBy` or `scrollIntoView`)
6. Use `frame.evaluate()` not `page.evaluate()` for the DOM manipulation

## Success Criteria

- **Obtains iframe frame object**: Code calls page.frame({ name: 'microConsole-Logs' }) to get the CloudWatch iframe context
- **Does NOT use window.scrollBy or window.scrollTo**: Code does not call window.scrollBy() or window.scrollTo() to perform the scroll
- **Does NOT use page.evaluate for CloudWatch content**: Code does not call page.evaluate() to query CloudWatch DOM elements — uses frame.evaluate()
- **Targets logs-table__wrapper container**: Code queries .logs-table__wrapper as the scroll target element
- **Sets scrollTop directly**: Code sets .scrollTop = 600 (not scrollBy, not scrollIntoView) on the container
- **Uses frame.evaluate not page.evaluate**: The evaluate call is on the frame object, not on page

## Failure Conditions

- Does not call `page.frame(...)` to obtain the iframe frame
- Uses `window.scrollBy()` or `window.scrollTo()` for the scroll operation
- Uses `page.evaluate()` instead of `frame.evaluate()` to query CloudWatch DOM elements
- Does not target `.logs-table__wrapper` as the scroll container
- Uses `scrollBy()` or `scrollIntoView()` instead of setting `.scrollTop` directly
- The evaluate call is on `page` instead of the frame object
