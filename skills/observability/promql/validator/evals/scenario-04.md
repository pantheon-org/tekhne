# Scenario 04: Explain a Multi-Function PromQL Query

## User Prompt

A new team member is reviewing an existing Grafana dashboard and encounters a complex PromQL query they do not understand. They need a clear plain-English explanation before they can maintain or modify it safely.

**Query Under Review:**

```promql
histogram_quantile(
  0.95,
  sum by (le, job) (
    rate(http_request_duration_seconds_bucket{job="checkout-service"}[5m])
  )
)
```

Produce an explanation file `query-explanation.md` that covers:

1. **What metric is being queried** — name, inferred type, and what it measures.
2. **What each function does** — explain `rate()`, `sum by()`, and `histogram_quantile()` in plain English in the context of this specific query.
3. **The calculation performed** — describe step-by-step what the query computes, starting from the innermost expression.
4. **Output Labels** — list which labels appear in the result vector.
5. **Expected Result Structure** — state whether the result is an instant vector, range vector, or scalar, and how many series to expect.
6. **Common Pitfalls** — note at least one mistake to avoid when modifying this query (e.g., incorrect aggregation that loses the `le` label).

Produce a single file `query-explanation.md` with all six sections.

## Expected Behavior

1. Identify `http_request_duration_seconds_bucket` as a histogram metric and describe what histograms measure
2. Explain `rate()` (per-second rate of bucket observations), `sum by (le, job)` (aggregate across instances while keeping `le` and `job` labels), and `histogram_quantile()` (interpolate the 95th percentile from bucket counts) in plain English in the context of this specific query
3. Describe the correct evaluation order: rate of buckets → sum to aggregate instances → histogram_quantile to compute the percentile
4. State that the output labels are `job` (not `le`, as `histogram_quantile` removes it), and that the result is an instant vector with one series per distinct `job` value
5. Document at least one concrete pitfall (e.g., dropping `le` from `sum by()` which breaks `histogram_quantile`, or applying `avg()` instead of `sum()` on buckets)

## Success Criteria

- **Metric type correctly identified as histogram**: The explanation states that `http_request_duration_seconds_bucket` is a histogram metric, not a counter or gauge, and describes what histograms measure
- **Each function explained accurately in context**: The explanation covers `rate()` (per-second rate of bucket observations), `sum by (le, job)` (aggregate across instances while keeping `le` and `job` labels), and `histogram_quantile()` (interpolate the 95th percentile from bucket counts)
- **Step-by-step calculation is correct**: The explanation describes the correct evaluation order: rate of buckets → sum to aggregate instances → histogram_quantile to compute the percentile
- **Output labels and result structure are correct**: The explanation states that the output labels are `job` (not `le`, as `histogram_quantile` removes it), and that the result is an instant vector with one series per distinct `job` value
- **At least one meaningful pitfall documented**: The pitfalls section includes at least one concrete mistake (e.g., dropping `le` from `sum by()` which breaks `histogram_quantile`, or applying `avg()` instead of `sum()` on buckets)

## Failure Conditions

- Misidentifies the metric type (e.g., calls it a counter or gauge instead of a histogram)
- Explains `rate()`, `sum by()`, or `histogram_quantile()` incorrectly or out of context
- Describes the evaluation order incorrectly (e.g., says `histogram_quantile` runs before `sum`)
- States incorrect output labels (e.g., claims `le` is present in the output, or lists `instance` when it was aggregated away)
- Omits the pitfalls section or lists only vague warnings without a concrete example
