# Scenario 2: Flag Missing `for` Clause in Alerting Rule

## Context

A platform engineer submitted the following Prometheus alerting rule to monitor HTTP
error rates. The rule should fire when the error rate exceeds 5% for a sustained period,
not on transient spikes.

## Alerting Rule Under Review

```yaml
groups:
  - name: api-alerts
    rules:
      - alert: HighErrorRate
        expr: |
          rate(http_requests_total{job="api-service",status=~"5.."}[5m])
          /
          rate(http_requests_total{job="api-service"}[5m])
          > 0.05
        labels:
          severity: critical
        annotations:
          summary: "High error rate on API service"
          description: "Error rate is {{ $value | humanizePercentage }}"
```

## Task

Produce two output files:

1. `alert-review.md` — a validation report that:
   - Identifies the missing `for` clause and explains why it causes alert storms on
     transient spikes.
   - Confirms the expression itself is well-formed (label filters present, correct use
     of `rate()` on a counter).
   - Recommends a minimum `for` duration and explains the trade-off between sensitivity
     and stability.

2. `alert-corrected.yaml` — the corrected alerting rule with:
   - A `for` clause added with an appropriate duration.
   - All other fields preserved exactly as in the input.

## Output Specification

Produce `alert-review.md` and `alert-corrected.yaml`.
