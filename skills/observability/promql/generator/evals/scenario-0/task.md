# Build a RED Method Dashboard Query Set for an API Gateway

## Problem/Feature Description

A platform engineering team is building a Grafana dashboard for their API gateway (`api-gateway`). They want to implement the RED method (Rate, Errors, Duration) across all endpoints. The team's Prometheus scrapes two metric types:

- `http_requests_total` — a counter with labels `job`, `endpoint`, and `status_code`
- `node_memory_MemAvailable_bytes` — a gauge (available memory in bytes) with label `instance`

An intern recently submitted dashboard panels that apply the same `rate()` function to both metrics. The senior engineer suspects this is wrong for the gauge metric and has asked you to generate a correct set of queries for the dashboard.

The team wants three queries:
1. Per-endpoint request throughput (requests per second over a 5-minute window), broken down by endpoint
2. The overall error ratio (5xx responses as a fraction of all requests) over 5 minutes for the `api-gateway` job
3. Available memory on each instance (current value, with instance label preserved)

Produce the queries in a file called `queries.promql`. Each query should be on its own line with a comment above it explaining what it measures.

## Output Specification

Produce a file `queries.promql` containing the three queries described above, each preceded by a `# comment` describing what it measures.
