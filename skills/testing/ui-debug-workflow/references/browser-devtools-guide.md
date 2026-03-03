# Browser DevTools Guide

Use this guide to gather high-signal browser evidence.

## Console

- Filter by Errors first, then Warnings.
- Expand stack traces and copy file:line references.
- Record recurring messages and timestamps.

## Network

- Check failed requests (4xx/5xx) and payload mismatches.
- Validate request order for race-condition symptoms.
- Save HAR or request/response summaries when needed.

## Elements and Styles

- Inspect computed styles for layout regressions.
- Compare class list before/after interaction.
- Check visibility/overflow/z-index for hidden UI.

## Performance (Optional)

- Capture short profile for UI stutter/jank.
- Correlate long tasks with interaction lag.
