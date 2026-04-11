# Scenario 03: Write SLO Alerting Rules for a Checkout Service

## User Prompt

An e-commerce company has defined a 99.9% availability SLO for their `checkout-service`. The SRE team needs Prometheus alerting rules to detect when the service is burning through its error budget too quickly. They've been burned before by noisy alerts that fire on single-scrape anomalies and clear immediately — the team insists that any alert must require the fault condition to persist for several minutes before paging the on-call engineer.

The team also wants a fast-burn alert: if the service is consuming error budget at a rate that would exhaust the monthly budget within an hour, they want an immediate page. The relevant metric is `http_requests_total` with labels `job` and `status_code`. Error budget is 0.1% (0.001 error rate target).

Additionally, the team has a draft alert that they found in a runbook:

```yaml
- alert: HighErrorCount
  expr: increase(http_requests_total{job="checkout", status_code=~"5.."}[5m]) > 50
```

They want to know if this is correct for alerting purposes, and if not, they want the corrected version included.

Produce a file called `alerting_rules.yaml` containing:
1. A corrected version of the draft `HighErrorCount` alert (if it needs fixing)
2. A multi-window burn-rate alert for the 99.9% SLO using both a 1-hour and a 5-minute window

The YAML should be valid Prometheus alerting rule format.

## Expected Behavior

1. Add a `for:` clause to every alert rule in the file — no alert rule should be missing it
2. Replace `increase()` with `rate()` in the corrected high-error alert expression
3. Implement a multi-window burn-rate alert using `and` to combine a long window (e.g. 1h) and a short window (e.g. 5m)
4. Apply a burn rate multiplier (e.g. 14.4 for a fast-burn alert targeting 1-hour exhaustion) in the burn-rate alert
5. Include a `job` label filter in alert expressions (e.g. `{job="checkout"}`)
6. Set the corrected alert threshold as a per-second rate value (a decimal < 1) rather than a raw event count
7. Include an explicit `by()` or `without()` clause in any `sum()` within alert expressions
8. Structure the YAML with valid Prometheus alerting rule format including `alert:`, `expr:`, and `for:` keys

## Success Criteria

- **for clause on all alerts**: Every alert rule in the file includes a `for:` clause — no alert rule is missing it
- **rate() not increase() in alert expr**: The corrected high-error alert uses `rate()` in its `expr`, not `increase()`
- **Multi-window alert present**: An alert rule uses `and` to combine two separate window conditions (one long window e.g. 1h, one short window e.g. 5m)
- **Burn rate multiplier applied**: The burn-rate alert expression includes a burn rate factor (e.g. 14.4 or similar multiplier) relative to the error budget
- **job label filter present**: Alert expressions include a job label filter (e.g. `{job="checkout"}`)
- **rate() per-second threshold in corrected alert**: The corrected alert threshold is expressed as a per-second rate value (a decimal < 1) rather than a raw event count
- **by() on aggregations**: Any `sum()` in alert expressions includes an explicit `by()` or `without()` clause
- **Valid YAML alert rule format**: The file uses valid Prometheus alerting rule YAML structure with `alert:`, `expr:`, and `for:` keys

## Failure Conditions

- Omits the `for:` clause from one or more alert rules
- Keeps `increase()` in the corrected high-error alert expression
- Implements the burn-rate alert as a single-window alert instead of multi-window
- Omits the burn rate multiplier from the fast-burn alert
- Omits the `job` label filter from alert expressions
- Sets a count-based threshold (e.g. `> 50`) instead of a per-second rate threshold in the corrected alert
- Uses a bare `sum()` without `by()` or `without()` in aggregations
- Produces invalid YAML or uses incorrect Prometheus rule schema
