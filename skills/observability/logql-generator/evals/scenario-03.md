# Scenario 03: Distributed Trace Correlation in Loki 3.x

## User Prompt

A large SaaS platform has recently migrated its logging infrastructure to Loki 3.x and has enabled structured metadata support. The platform's microservices emit JSON-formatted logs. Each log line carries a `trace_id` (UUID), `user_id` (numeric, high cardinality), and `request_id` (UUID) alongside a `level` field and a `message` field. The Loki configuration has `discover_log_levels: true` enabled.

The platform runs an `order-service` in Kubernetes. Relevant labels available on the streams are: `app`, `namespace`, `cluster`, and `pod`.

A customer support engineer needs help with two queries:

1. **Trace investigation** — Find all error-level log lines from the order-service for a specific trace ID (`"a1b2c3d4-e5f6-7890-abcd-ef1234567890"`) to reconstruct what happened during a failed checkout. The query should work efficiently with Loki 3.x.

2. **Per-namespace error breakdown** — A metric query that shows the rate of detected error-level events per namespace across the entire cluster (not limited to order-service), grouped by namespace. This should also leverage Loki 3.x automatic level detection.

Produce a file named `trace_queries.md` containing:

- Both LogQL queries (clearly labelled)
- A brief explanation of each query, including which Loki 3.x capabilities they leverage
- A note about what would go wrong if the trace ID were placed differently in the first query

## Expected Behavior

1. Keep `trace_id` out of the stream selector braces (do not use `{trace_id="..."}`)
2. Filter `trace_id` as a label filter after the stream selector (e.g. `{app="order-service"} | trace_id="a1b2c3d4..."`)
3. Place the `trace_id` or `detected_level` filter before the json parser for bloom filter acceleration
4. Use `detected_level` (the Loki 3.x automatic label) in at least one query instead of parsing level from JSON
5. Keep `user_id` out of the stream selector braces as well
6. Use `sum by (namespace)` or equivalent grouping for the per-namespace metric query
7. Include a note warning that placing high-cardinality IDs in the stream selector causes index bloat
8. Identify at least one specific Loki 3.x feature (structured metadata, bloom filters, `detected_level`, etc.) by name

## Success Criteria

- **trace_id not in stream selector**: The trace investigation query does NOT place `trace_id` inside the stream selector braces `{trace_id="..."}`
- **trace_id as post-stream filter**: The trace investigation query filters `trace_id` as a label filter AFTER the stream selector (e.g. `{app="order-service"} | trace_id="a1b2c3d4..."`)
- **Structured metadata before parser**: In the trace investigation query, the `trace_id` or `detected_level` filter appears BEFORE the json parser (for bloom filter acceleration)
- **detected_level used**: At least one query uses `detected_level` (the Loki 3.x automatic label) rather than parsing `level` from JSON to detect error events
- **user_id not in stream selector**: Neither query places `user_id` inside the stream selector braces
- **sum by namespace grouping**: The per-namespace metric query uses `sum by (namespace)` or equivalent grouping on the namespace label
- **High-cardinality warning note**: The output document contains a note explaining that placing high-cardinality IDs (`trace_id`, `user_id`) in the stream selector causes index bloat or degraded performance
- **Loki 3.x capabilities identified**: The explanation mentions at least one specific Loki 3.x feature by name (structured metadata, bloom filters, `detected_level`, `approx_topk`, or pattern match operators)

## Failure Conditions

- Places `trace_id` inside the stream selector braces, causing index bloat
- Filters `trace_id` only via the stream selector and not as a post-stream label filter
- Places the structured metadata filter after the json parser, bypassing bloom filter acceleration
- Uses `| json | level="error"` instead of leveraging the `detected_level` auto-label
- Places `user_id` inside the stream selector braces
- Omits `namespace` grouping from the per-namespace metric query
- Provides no warning about high-cardinality label placement
- Fails to name any specific Loki 3.x feature in the explanation
