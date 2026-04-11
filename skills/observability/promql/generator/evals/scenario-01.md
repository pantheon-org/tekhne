# Scenario 01: Build a RED Method Dashboard Query Set for an API Gateway

## User Prompt

A platform engineering team is building a Grafana dashboard for their API gateway (`api-gateway`). They want to implement the RED method (Rate, Errors, Duration) across all endpoints. The team's Prometheus scrapes two metric types:

- `http_requests_total` — a counter with labels `job`, `endpoint`, and `status_code`
- `node_memory_MemAvailable_bytes` — a gauge (available memory in bytes) with label `instance`

An intern recently submitted dashboard panels that apply the same `rate()` function to both metrics. The senior engineer suspects this is wrong for the gauge metric and has asked you to generate a correct set of queries for the dashboard.

The team wants three queries:
1. Per-endpoint request throughput (requests per second over a 5-minute window), broken down by endpoint
2. The overall error ratio (5xx responses as a fraction of all requests) over 5 minutes for the `api-gateway` job
3. Available memory on each instance (current value, with instance label preserved)

Produce the queries in a file called `queries.promql`. Each query should be on its own line with a comment above it explaining what it measures.

## Expected Behavior

1. Apply `rate()` or `increase()` to the counter metric `http_requests_total` for the throughput query
2. Use the gauge metric `node_memory_MemAvailable_bytes` directly (without `rate()` or `increase()`) for the memory query
3. Include a `job` label filter (e.g. `{job="api-gateway"}`) in at least one counter query
4. Use `sum by (endpoint)` or equivalent in the throughput query for per-endpoint breakdown
5. Include an explicit `by()` or `without()` clause in the error ratio aggregation
6. Divide the 5xx request rate by the total request rate (binary `/` operator between two `rate()` expressions)
7. Use a regex match on `status_code` to select 5xx errors (e.g. `status_code=~"5.."`)
8. Return the memory metric without aggregation or with `by(instance)` to preserve the instance label

## Success Criteria

- **rate() on counter**: The throughput query uses `rate()` or `increase()` applied to `http_requests_total` (a counter), not a raw selector
- **No rate() on gauge**: The memory query does NOT wrap `node_memory_MemAvailable_bytes` in `rate()` or `increase()` — it uses the metric directly or with an `*_over_time()` function
- **Label filter on counter queries**: At least one counter query includes a job label filter (e.g. `{job="api-gateway"}`)
- **by() on throughput aggregation**: The per-endpoint throughput query uses `sum by (endpoint)` or an equivalent explicit `by()` clause
- **by() on error ratio**: Any aggregation in the error ratio query includes an explicit `by()` or `without()` clause rather than a bare `sum`/`count`
- **Error ratio uses division**: The error ratio query divides 5xx request rate by total request rate (binary `/` operator between two `rate()` expressions)
- **Status code regex for 5xx**: The error ratio query uses a regex match on `status_code` to select 5xx errors (e.g. `status_code=~"5.."`)
- **Memory query preserves instance label**: The memory query either has no aggregation (returning all instances) or explicitly keeps the instance label via `by(instance)`

## Failure Conditions

- Applies `rate()` to the gauge metric `node_memory_MemAvailable_bytes`
- Uses a raw selector (no `rate()`) for the counter-based throughput query
- Omits the `job` label filter from counter queries
- Omits `by (endpoint)` from the per-endpoint throughput aggregation
- Uses a bare `sum()` without `by()` or `without()` in the error ratio
- Computes error ratio by subtraction instead of division
- Filters 5xx errors with a literal match (`status_code="500"`) instead of a regex
- Aggregates away the `instance` label from the memory query
