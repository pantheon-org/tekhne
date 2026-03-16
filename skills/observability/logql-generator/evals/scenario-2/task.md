# Distributed Trace Correlation in Loki 3.x

## Problem Description

A large SaaS platform has recently migrated its logging infrastructure to Loki 3.x and has enabled structured metadata support. The platform's microservices emit JSON-formatted logs. Each log line carries a `trace_id` (UUID), `user_id` (numeric, high cardinality), and `request_id` (UUID) alongside a `level` field and a `message` field. The Loki configuration has `discover_log_levels: true` enabled.

The platform runs an `order-service` in Kubernetes. Relevant labels available on the streams are: `app`, `namespace`, `cluster`, and `pod`.

A customer support engineer needs help with two queries:

1. **Trace investigation** — Find all error-level log lines from the order-service for a specific trace ID (`"a1b2c3d4-e5f6-7890-abcd-ef1234567890"`) to reconstruct what happened during a failed checkout. The query should work efficiently with Loki 3.x.

2. **Per-namespace error breakdown** — A metric query that shows the rate of detected error-level events per namespace across the entire cluster (not limited to order-service), grouped by namespace. This should also leverage Loki 3.x automatic level detection.

## Output Specification

Produce a file named `trace_queries.md` containing:

- Both LogQL queries (clearly labelled)
- A brief explanation of each query, including which Loki 3.x capabilities they leverage
- A note about what would go wrong if the trace ID were placed differently in the first query
