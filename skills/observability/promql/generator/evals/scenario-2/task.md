# Write SLO Alerting Rules for a Checkout Service

## Problem/Feature Description

An e-commerce company has defined a 99.9% availability SLO for their `checkout-service`. The SRE team needs Prometheus alerting rules to detect when the service is burning through its error budget too quickly. They've been burned before by noisy alerts that fire on single-scrape anomalies and clear immediately — the team insists that any alert must require the fault condition to persist for several minutes before paging the on-call engineer.

The team also wants a fast-burn alert: if the service is consuming error budget at a rate that would exhaust the monthly budget within an hour, they want an immediate page. The relevant metric is `http_requests_total` with labels `job` and `status_code`. Error budget is 0.1% (0.001 error rate target).

Additionally, the team has a draft alert that they found in a runbook:

```yaml
- alert: HighErrorCount
  expr: increase(http_requests_total{job="checkout", status_code=~"5.."}[5m]) > 50
```

They want to know if this is correct for alerting purposes, and if not, they want the corrected version included.

## Output Specification

Produce a file called `alerting_rules.yaml` containing:
1. A corrected version of the draft `HighErrorCount` alert (if it needs fixing)
2. A multi-window burn-rate alert for the 99.9% SLO using both a 1-hour and a 5-minute window

The YAML should be valid Prometheus alerting rule format.
