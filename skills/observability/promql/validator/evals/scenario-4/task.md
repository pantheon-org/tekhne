# Scenario 4: Explain a Multi-Function PromQL Query

## Context

A new team member is reviewing an existing Grafana dashboard and encounters a complex
PromQL query they do not understand. They need a clear plain-English explanation before
they can maintain or modify it safely.

## Query Under Review

```promql
histogram_quantile(
  0.95,
  sum by (le, job) (
    rate(http_request_duration_seconds_bucket{job="checkout-service"}[5m])
  )
)
```

## Task

Produce an explanation file `query-explanation.md` that covers:

1. **What metric is being queried** — name, inferred type, and what it measures.
2. **What each function does** — explain `rate()`, `sum by()`, and `histogram_quantile()`
   in plain English in the context of this specific query.
3. **The calculation performed** — describe step-by-step what the query computes,
   starting from the innermost expression.
4. **Output Labels** — list which labels appear in the result vector.
5. **Expected Result Structure** — state whether the result is an instant vector, range
   vector, or scalar, and how many series to expect.
6. **Common Pitfalls** — note at least one mistake to avoid when modifying this query
   (e.g., incorrect aggregation that loses the `le` label).

## Output Specification

Produce a single file `query-explanation.md` with all six sections.
