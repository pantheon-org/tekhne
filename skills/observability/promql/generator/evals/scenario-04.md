# Scenario 04: Create Recording Rules to Speed Up Slow Dashboard Queries

## User Prompt

A DevOps team supports a multi-tenant SaaS platform. Their Grafana dashboards are timing out because several expensive PromQL queries run on every panel refresh. The head of infrastructure has determined that three queries are the main culprits — they are all computed in multiple dashboard panels and run over large time ranges. The team wants to pre-compute these queries as Prometheus recording rules so they can be reused across dashboards without re-evaluating the full expression each time.

Additionally, a teammate proposed the following query for a "per-user request breakdown" panel:

```promql
sum by (user_id) (rate(http_requests_total[5m]))
```

The senior engineer flagged this as potentially dangerous to the Prometheus instance but hasn't explained why. The team wants the recording rules written up, and also wants an explanation of the issue with the proposed query and what a safer alternative looks like.

The three queries to pre-compute:
1. The per-job request rate over 5 minutes: `sum by (job) (rate(http_requests_total[5m]))`
2. The per-job 5xx error rate over 5 minutes: `sum by (job) (rate(http_requests_total{status_code=~"5.."}[5m]))`
3. The per-job P95 latency: `histogram_quantile(0.95, sum by (job, le) (rate(http_request_duration_seconds_bucket[5m])))`

Produce two files:

1. `recording_rules.yaml` — Prometheus recording rule YAML for the three queries above, using correct recording rule naming conventions.
2. `cardinality_analysis.md` — A short Markdown document (1-2 paragraphs) explaining the problem with the `sum by (user_id)` query and providing a safer alternative query.

## Expected Behavior

1. Name all three recording rules using the `level:metric:operations` naming format (e.g. `job:http_requests:rate5m`)
2. Include exactly three `record:` entries in `recording_rules.yaml` covering request rate, error rate, and latency
3. Use valid Prometheus recording rule YAML with `record:` and `expr:` keys
4. Identify `user_id` as a high-cardinality label in `cardinality_analysis.md` and explain it produces too many time series
5. Provide a safer alternative query that does NOT group by `user_id` (e.g. groups by `job`, `status_code`, or other low-cardinality labels)
6. Include an explicit `by()` clause in all three recording rule expressions
7. Use `histogram_quantile(0.95, ...)` with `sum by (job, le)` for the latency recording rule
8. Include a `status_code` label filter in the error rate recording rule expression

## Success Criteria

- **Recording rule naming convention**: All three recording rules use the `level:metric:operations` naming format (e.g. `job:http_requests:rate5m`) — NOT arbitrary names
- **Three recording rules present**: `recording_rules.yaml` contains exactly three `record:` entries covering request rate, error rate, and latency
- **Valid YAML recording rule format**: The file uses valid Prometheus recording rule YAML with `record:` and `expr:` keys (and optionally `labels:`)
- **High-cardinality label identified**: `cardinality_analysis.md` explicitly identifies `user_id` as a high-cardinality label and explains it produces too many series
- **Safer alternative query provided**: `cardinality_analysis.md` provides a safer alternative query that does NOT group by `user_id` (e.g. groups by `job`, `status_code`, or other low-cardinality labels)
- **by() on all recording rule exprs**: All three recording rule expressions include an explicit `by()` clause in any aggregation
- **Latency rule uses histogram_quantile**: The latency recording rule expression uses `histogram_quantile(0.95, ...)` with `sum by (job, le)`
- **Label filter in error rate rule**: The error rate recording rule expression includes a `status_code` label filter to select 5xx errors

## Failure Conditions

- Uses arbitrary names (e.g. `request_rate`, `error_rate`) instead of the `level:metric:operations` convention
- Produces fewer or more than three recording rules
- Produces invalid YAML or uses incorrect Prometheus rule schema
- Fails to identify `user_id` as high-cardinality or explain the cardinality explosion
- Provides no safer alternative or proposes an alternative that still groups by `user_id`
- Omits `by()` clauses from recording rule expressions
- Uses `avg_over_time` or a non-quantile function instead of `histogram_quantile` for the latency rule
- Omits the `status_code` filter from the error rate rule
