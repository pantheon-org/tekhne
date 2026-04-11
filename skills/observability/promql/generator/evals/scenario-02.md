# Scenario 02: Generate Latency Percentile Queries for a Payment Processing Service

## User Prompt

A fintech team runs a payment processing service (`payment-api`) and wants to add P50, P95, and P99 latency panels to their Grafana dashboard. Their Prometheus setup exposes latency as a classic histogram named `payment_request_duration_seconds` with the standard `_bucket`, `_count`, and `_sum` suffixes. Labels include `job`, `payment_method`, and `le`.

A previous engineer left the following query in a runbook, and the SRE team is unsure whether it is correct:

```
avg(payment_request_duration_seconds{quantile="0.95"})
```

The team needs you to generate the correct P50, P95, and P99 latency queries suitable for use in a Grafana dashboard, broken down by `payment_method`. Write a short explanation next to each query explaining what it computes.

Produce a file called `latency_queries.promql` containing the P50, P95, and P99 queries, each preceded by a `# comment`. The queries should be scoped to the `payment-api` job and broken down by `payment_method`.

## Expected Behavior

1. Use `histogram_quantile()` for all three latency queries — not `avg()`, not direct `{quantile="..."}` label selectors
2. Wrap each `histogram_quantile()` call with `sum by (le)` to preserve the `le` bucket label
3. Apply `rate()` to the `_bucket` metric inside the `sum by (le)` expression
4. Avoid the anti-pattern of `avg(...{quantile=...})` anywhere in the file
5. Include `{job="payment-api"}` or equivalent label filter in at least one query
6. Include `payment_method` in a `by()` clause or preserve it through the aggregation structure
7. Produce all three quantile values: `0.5` (P50), `0.95` (P95), and `0.99` (P99)

## Success Criteria

- **histogram_quantile used**: All three latency queries use `histogram_quantile()` — not `avg()`, not direct quantile label selectors
- **sum by (le) preserved**: Each `histogram_quantile()` call wraps a `sum by (le) (...)` expression to preserve the `le` bucket label
- **rate() on _bucket**: The inner expression applies `rate()` to the `_bucket` metric (e.g. `payment_request_duration_seconds_bucket`) rather than the raw counter
- **No avg(quantile) anti-pattern**: The file does NOT contain `avg(...{quantile=...})` or any query that averages across pre-aggregated quantile labels
- **job label filter present**: At least one query includes `{job="payment-api"}` or equivalent label filter
- **payment_method in by() clause**: At least one query includes `payment_method` in a `by()` clause or preserves it via the aggregation structure
- **Three quantile values produced**: Queries for all three percentiles are present: `0.5` (P50), `0.95` (P95), and `0.99` (P99)

## Failure Conditions

- Uses `avg()` applied to pre-aggregated `{quantile="..."}` labels instead of `histogram_quantile()`
- Omits `sum by (le)` around the bucket rate expression, breaking `histogram_quantile()`
- Applies `rate()` to the `_count` or `_sum` suffix instead of the `_bucket` metric
- Includes the anti-pattern `avg(...{quantile="0.95"})` from the runbook without correction
- Omits the `job` label filter from all queries
- Omits `payment_method` from all aggregation clauses
- Produces fewer than three quantile queries (missing P50, P95, or P99)
