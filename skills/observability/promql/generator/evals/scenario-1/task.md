# Generate Latency Percentile Queries for a Payment Processing Service

## Problem/Feature Description

A fintech team runs a payment processing service (`payment-api`) and wants to add P50, P95, and P99 latency panels to their Grafana dashboard. Their Prometheus setup exposes latency as a classic histogram named `payment_request_duration_seconds` with the standard `_bucket`, `_count`, and `_sum` suffixes. Labels include `job`, `payment_method`, and `le`.

A previous engineer left the following query in a runbook, and the SRE team is unsure whether it is correct:

```
avg(payment_request_duration_seconds{quantile="0.95"})
```

The team needs you to generate the correct P50, P95, and P99 latency queries suitable for use in a Grafana dashboard, broken down by `payment_method`. Write a short explanation next to each query explaining what it computes.

## Output Specification

Produce a file called `latency_queries.promql` containing the P50, P95, and P99 queries, each preceded by a `# comment`. The queries should be scoped to the `payment-api` job and broken down by `payment_method`.
