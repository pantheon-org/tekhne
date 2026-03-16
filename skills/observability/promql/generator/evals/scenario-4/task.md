# Generate a Complete Monitoring Query Package for a New Service

## Problem/Feature Description

A team is instrumenting a new internal service (`inventory-api`) for the first time. They have a Prometheus setup but no existing dashboards or alerting. The engineering manager wants a complete starter monitoring package that her team can hand directly to the on-call engineers — not just the queries, but enough context for someone unfamiliar with PromQL to understand what each query does, how to use it, and how to customize it for their own environment.

The service exposes:
- `inventory_http_requests_total` — a counter with labels `job`, `endpoint`, `method`, `status_code`
- `inventory_request_duration_seconds` — a classic histogram with labels `job`, `endpoint`, `le`
- `inventory_cache_size` — a gauge (number of items currently in cache) with label `job`

The team wants three queries:
1. Overall request throughput by endpoint (5-minute window)
2. P95 end-to-end request latency by endpoint (5-minute window)
3. Current cache size

## Output Specification

Produce a single file called `monitoring_package.md` that contains:
- Each query in a fenced PromQL code block
- For each query: a plain-English explanation of what it measures and what the output values represent
- For each query: a note on how to customize it (e.g., change the job name, adjust the time window, add label filters)
- A "Related Queries" section at the end suggesting at least two additional queries the team might want

The queries must be formatted across multiple lines if they involve more than one operator.
