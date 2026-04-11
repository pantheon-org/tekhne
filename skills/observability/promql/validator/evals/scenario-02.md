# Scenario 02: Flag Missing `for` Clause in Alerting Rule

## User Prompt

A platform engineer submitted the following Prometheus alerting rule to monitor HTTP error rates. The rule should fire when the error rate exceeds 5% for a sustained period, not on transient spikes.

**Alerting Rule Under Review:**

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

Produce two output files:

1. `alert-review.md` — a validation report that:
   - Identifies the missing `for` clause and explains why it causes alert storms on transient spikes.
   - Confirms the expression itself is well-formed (label filters present, correct use of `rate()` on a counter).
   - Recommends a minimum `for` duration and explains the trade-off between sensitivity and stability.

2. `alert-corrected.yaml` — the corrected alerting rule with:
   - A `for` clause added with an appropriate duration.
   - All other fields preserved exactly as in the input.

## Expected Behavior

1. Explicitly identify the missing `for` clause in the review report
2. Explain that without `for`, the alert fires on a single evaluation cycle, causing false positives from transient spikes
3. Recommend a `for` duration of at least 2m with a brief explanation of the sensitivity/stability trade-off
4. Add a `for:` field of 2m or greater to `alert-corrected.yaml`
5. Preserve the `expr`, `labels`, and `annotations` fields exactly as in the input with no other modifications

## Success Criteria

- **Missing `for` clause is explicitly identified**: The review report explicitly states that the alerting rule lacks a `for` clause
- **Alert storm risk is explained**: The report explains that without `for`, the alert fires on a single evaluation cycle, which causes false positives from transient spikes
- **`for` duration recommendation is reasonable**: The report recommends a `for` duration of at least 2m (the skill's stated minimum), with a brief explanation of the sensitivity/stability trade-off
- **Corrected YAML includes `for` clause**: The `alert-corrected.yaml` file contains a `for:` field with a duration of 2m or greater added to the rule
- **All other rule fields are preserved unchanged**: The corrected YAML preserves the `expr`, `labels`, and `annotations` fields exactly as in the input, with no other modifications

## Failure Conditions

- Fails to identify the missing `for` clause
- Does not explain why a missing `for` clause causes alert storms on transient spikes
- Recommends a `for` duration shorter than 2m without justification
- Produces a corrected YAML without a `for:` field
- Modifies `expr`, `labels`, or `annotations` fields beyond adding the `for` clause
