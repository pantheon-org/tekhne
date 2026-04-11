# Scenario 03: Prepare a CloudWatch Logs Insights Page for a Clean Screenshot

## User Prompt

You have a Playwright `page` object. The browser is on a CloudWatch Logs Insights page showing query results. You need to prepare the page for a clean screenshot by:

1. Hiding the query histogram chart above the results table
2. Collapsing the left navigation sidebar

Write the Playwright code to achieve both.

## Expected Behavior

1. Use `frame.evaluate()` (not `page.evaluate()`) to hide the histogram, since it is inside the CloudWatch iframe
2. Target the histogram element by `.logs__histogram-container` class
3. Hide the histogram by setting `style.display = 'none'`
4. Use `page.locator()` (not `frame.locator()`) to target the sidebar close button, since it is in the outer shell
5. Use an `aria-label` selector such as `'Close navigation'` or `'Collapse sidebar'` to find the sidebar button
6. Check `isVisible()` or use a try/catch before clicking the sidebar button (it may already be collapsed)
7. Call `page.frame({ name: 'microConsole-Logs' })` before querying histogram elements

## Success Criteria

- **Histogram hidden via frame.evaluate**: Code uses frame.evaluate() (not page.evaluate()) to hide the histogram, since it is inside the iframe
- **Targets .logs__histogram-container**: Histogram element is selected by .logs__histogram-container class
- **Sets display:none on histogram**: Histogram is hidden by setting style.display = 'none'
- **Sidebar collapsed via page locator**: Sidebar close button is targeted via page.locator() (not frame.locator()), since it is in the outer shell
- **Sidebar aria-label used for close button**: Code uses aria-label selector such as 'Close navigation' or 'Collapse sidebar' to find the button
- **Existence check before clicking sidebar button**: Code checks isVisible() or uses a try/catch before clicking the sidebar button (it may already be collapsed)
- **Obtains iframe frame for histogram work**: Code calls page.frame({ name: 'microConsole-Logs' }) before querying histogram

## Failure Conditions

- Uses `page.evaluate()` instead of `frame.evaluate()` to hide the histogram
- Does not target `.logs__histogram-container` class for the histogram element
- Does not set `style.display = 'none'` on the histogram
- Uses `frame.locator()` instead of `page.locator()` for the sidebar button (sidebar is outside the iframe)
- Does not use an `aria-label` selector to find the sidebar close button
- Clicks the sidebar button without first checking if it is visible
- Does not call `page.frame(...)` before querying histogram elements in the iframe
