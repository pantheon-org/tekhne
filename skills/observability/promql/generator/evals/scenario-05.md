# Scenario 05: Generate a Complete Monitoring Query Package for a New Service

## User Prompt

A team is instrumenting a new internal service (`inventory-api`) for the first time. They have a Prometheus setup but no existing dashboards or alerting. The engineering manager wants a complete starter monitoring package that her team can hand directly to the on-call engineers — not just the queries, but enough context for someone unfamiliar with PromQL to understand what each query does, how to use it, and how to customize it for their own environment.

The service exposes:
- `inventory_http_requests_total` — a counter with labels `job`, `endpoint`, `method`, `status_code`
- `inventory_request_duration_seconds` — a classic histogram with labels `job`, `endpoint`, `le`
- `inventory_cache_size` — a gauge (number of items currently in cache) with label `job`

The team wants three queries:
1. Overall request throughput by endpoint (5-minute window)
2. P95 end-to-end request latency by endpoint (5-minute window)
3. Current cache size

Produce a single file called `monitoring_package.md` that contains:
- Each query in a fenced PromQL code block
- For each query: a plain-English explanation of what it measures and what the output values represent
- For each query: a note on how to customize it (e.g., change the job name, adjust the time window, add label filters)
- A "Related Queries" section at the end suggesting at least two additional queries the team might want

The queries must be formatted across multiple lines if they involve more than one operator.

## Expected Behavior

1. Accompany each of the three queries with a written explanation of what it measures (not just the expression)
2. Include at least one customization note per query (how to change job name, time window, or add label filters)
3. Add a "Related Queries" section with at least two additional related query suggestions
4. Format queries with more than one operator across multiple lines
5. Use `rate()` on `inventory_http_requests_total` (counter); do NOT use `rate()` on `inventory_cache_size` (gauge)
6. Use `histogram_quantile()` wrapping `sum by (le) (...rate(inventory_request_duration_seconds_bucket...))` for the latency query
7. Include explicit `by()` clauses in the throughput and latency queries (e.g. `by (endpoint)`)
8. Include a `job` label filter for the `inventory-api` job in at least one query
9. Explain what the numeric output values represent in at least one query explanation (e.g. "requests per second", "seconds at the 95th percentile")

## Success Criteria

- **Plain-English explanation per query**: Each of the three queries is accompanied by a written explanation of what it measures (not just the query expression alone)
- **Customization notes per query**: Each query has at least one customization note (e.g. how to change the job name, time window, or add label filters)
- **Related queries section**: The document includes a section with at least two additional related query suggestions beyond the three required queries
- **Multi-line formatting for complex queries**: Queries with more than one operator or aggregation are formatted across multiple lines (not written as a single long line)
- **rate() on counter, not gauge**: The throughput query uses `rate()` on `inventory_http_requests_total`; the cache size query does NOT use `rate()` on `inventory_cache_size`
- **histogram_quantile with sum by (le)**: The latency query uses `histogram_quantile()` wrapping a `sum by (le) (...rate(inventory_request_duration_seconds_bucket...))` expression
- **by() on aggregations**: The throughput and latency queries both include an explicit `by()` clause (e.g. `by (endpoint)`)
- **Label filter on queries**: At least one query includes a job label filter for the inventory-api job
- **Output value explanation**: At least one query explanation describes what the numeric output value represents (e.g. "requests per second", "seconds at the 95th percentile")

## Failure Conditions

- Omits written explanations for one or more queries
- Omits customization notes for one or more queries
- Does not include a related queries section or includes fewer than two suggestions
- Formats all queries as single long lines without line breaks
- Applies `rate()` to the gauge metric `inventory_cache_size`
- Uses `avg_over_time` or a non-quantile function instead of `histogram_quantile` for P95
- Omits `by()` clauses from throughput or latency queries
- Omits the `job` label filter from all queries
- Explains queries without describing what the output numbers represent
